---
title: Circular references
author: Kayla Washburn
date: 2023.5.16
summary: in Rust and more!
tags: programming, rust, go, zig, memory
accent_color: "#00a2ef"
---

While talking with some friends in Discord a while ago, one of them asked me how you'd
create circular references in Rust. I'd never really felt the need for them before, so I
went to the [Rust playground] to figure out what a bare-minimum circular reference would
look like. I find that most things in Rust aren't as hard as I think people expect, surely
this is no exception, right?

<aside>
This post doesn't really have a point; it's just a couple of code samples, some
remarks about them, and some talk about language design. :)
</aside>

## Rust

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Friend {
  name: String,
  bestie: Weak<RefCell<Friend>>,
}

impl Friend {
  fn named(name: &str) -> Self {
    Friend { name: name.to_string(), bestie: Weak::new() }
  }
}

fn main() {
  // `Rc` handles reference counting, `RefCell` allows us to obtain a mutable reference
  let lilac   = Rc::new(RefCell::new(Friend::named("lilac")));
  let rattard = Rc::new(RefCell::new(Friend::named("rattard")));

  // Downgrading to `Weak` is important! Otherwise this memory will never be freed!
  lilac.borrow_mut().bestie = Rc::downgrade(&rattard);
  rattard.borrow_mut().bestie = Rc::downgrade(&lilac);

  // ...and then use it!
  println!(
    "my best friend's username is {}!",
    lilac
      .borrow()  // Borrow from my `RefCell`
      .bestie    // `Weak<_>`
      .upgrade() // `Weak` -> `Option<Rc<_>>`
      .unwrap()  // `Option<Rc<RefCell<_>>>` -> `Rc<RefCell<_>>`
      .borrow()  // Borrow from the `RefCell` of my bestie
      .name      // Get their name! Easy!
  );
}
```

Ok, that's not so bad! It's definitely a _little_ bad, but I can still grok what's happening,
and hopefully so can you. The simplest thing I could get to work was to create the
circular reference through a `Weak<RefCell<_>>`. There are a few drawbacks to
this design though...

- The `Weak` reference to your `bestie` can be lost, if no one else is holding a strong
  reference. This is partially by design, because otherwise these two values will keep
  each other alive forever.
- A lot of noise when reaching into the reference (and it gets even noisier if you add
  error handling for the case where `.upgrade()` returns `None`!)
- You have to _know_ about `Rc`, and `Weak`, and `RefCell`, and how these all interact.
  It might seem simple to some developers, but I know it wasn't for me! It took a lot of
  trial and error to get to this version!

It's kind of a lot of work to put into a solution that still has gotchas, but it made me
curious: what does this problem look like in other languages?

## Go

```go
package main

import "fmt"

type Friend struct {
  Name   string
  Bestie *Friend
}

func main() {
  // Garbage collection means no need for reference counting, smart pointers, etc.
  lilac   := Friend{Name: "lilac"}
  rattard := Friend{Name: "rattard"}

  // Just grab some pointers! The garbage collector can handle this just fine!
  lilac.Bestie = &rattard
  rattard.Bestie = &lilac

  // Just read the properties! No nested layers of unfolding!
  fmt.Printf("my best friend's username is %s!", lilac.Bestie.Name)
}
```

It's essentially the same amount of code if you're going by "number of lines to set things
up", but the lines of the Rust version are **much** denser. Go's garbage collection allows
us to just have the values point directly at each other.

- These references are strong, so we don't need to worry about them disappearing on us
  like the `Weak` Rust version, but the values can also both still be freed if they become
  disconnected from the rest of the graph.
- Go does have `nil`, so there's some worry about dereferencing uninitialized pointers,
  but at least we _don't_ need to worry about dereferencing _freed_ pointers.
- Garbage collection isn't a cure-all, nor is it free, but it definitely makes this
  sort of thing simpler.

## Zig

For another point of comparison, we can look at the same thing in Zig!

```zig
const std = @import("std");

const Friend = struct {
  name: []const u8,
  bestie: *Friend,
};

pub fn main() !void {
  // Bring your own heap allocator!
  var allocatorBackend = std.heap.GeneralPurposeAllocator(.{}){};
  const allocator = allocatorBackend.allocator();
  // We're being good noodles and doing some work to make sure we're not leaking memory.
  defer {
    const leaked = allocatorBackend.deinit();
    if (leaked == true) @panic("oh no!");
  }

  // We don't need wrappers here either, because Zig makes us manage our own memory.
  // We do however, need to allocate some memory ourselves, and then free it later.
  var lilac   = try allocator.create(Friend);
  var rattard = try allocator.create(Friend);
  defer {
    allocator.destroy(lilac);
    allocator.destroy(rattard);
  }
  lilac.*   = Friend{ .name = "lilac", .bestie = rattard };
  rattard.* = Friend{ .name = "rattard", .bestie = lilac };

  // The compiler will dereference the pointers for us!
  std.log.info("my best friend's name is {s}", .{lilac.bestie.name});
}
```

It's interesting how Zig shuffles around the complexity. I'd say it's about as complicated
as the Rust version, the complexity is just in different places.

- We can just use pointers, and the compiler will dereference them for us when we want to
  read fields from our values, but it's now a lot less obvious what will happen if that
  memory is uninitialized.
- Memory allocation is very explicit in Zig. Rust made us acknowledge the sausage, but
  Zig shows us how it's made.
- We do get to guarantee that the pointers won't be `null`, since we're using `*Friend`
  as the type instead of the nullable `?*Friend`.
- There's no initial period where the values have been initialized but aren't linked.
  We're allocating the memory before initializing the values, so we already know where
  both values will reside, and can initialize them with the correct pointers.

## Rust, again

Circling back to Rust, if we're willing to forego Rust's memory safety, we can
actually get a pretty comparable version.

```rust
use std::ptr::null;

struct Friend {
  name: String,
  bestie: *const Friend,
}

impl Friend {
  fn named(name: &str) -> Self {
    Friend { name: name.to_string(), bestie: null() }
  }
}

fn main() {
  // We're just using `Box` to allocate on the heap
  let mut lilac   = Box::new(Friend::named("lilac"));
  let mut rattard = Box::new(Friend::named("rattard"));

  // `&Friend` gets turned into `*const Friend`
  lilac.bestie = &*rattard;
  rattard.bestie = &*lilac;

  // It's a little messier than the Zig syntax, but I think it's ok for dangerous things
  // to look dangerous. (Also the dereference needs to be wrapped in an `unsafe` block.)
  println!(
    "my best friend's username is {}!",
    unsafe { &(*lilac.bestie).name },
  );
}
```

- Both values will be freed! It's up to us now to ensure that the pointer we're
  dereferencing is valid, but there are ways to handle that.
- Way fewer charades to reach in and grab stuff when you know that it's valid to do so.
  No more peeling back layers!
- We lose the safety promises that Rust usually offers, as evidenced by the required use
  of an `unsafe` block to dereference our raw pointer.

## JavaScript

A "simplest" example might be JavaScript, which really still looks basically the same as
the other languages, just with a lot of fluff pulled away.

```js
let lilac = { name: "lilac" };
let rattard = { name: "rattard" };

lilac.bestie = rattard;
rattard.bestie = lilac;

console.log(lilac.bestie.name);
```

- Create two objects
- Link them to each other
- Do the thing

# Remarks

I'm not entirely convinced that circular references are even a particularly interesting
problem, or that they really solve any problem that wouldn't be better solved in another
manner. I actually kind of like that doing this "the right way" in Rust is obnoxious,
because hopefully that pain can discourage this sort of design, and push people towards
something more servicable. In all of these languages, it either required interior
mutability, or very low-level control over the allocation process in the case of Zig.
There are many languages where that's just not accessible; like Gleam or Elixir.

Despite that, I think each of these examples makes for a great showcase of relative
priorities between these languages. Go and Zig both aim to be "simple", and yet have
remarkably different approaches to things. Rust tries to be approachable _and_ exact at
the same time, and sometimes those goals are in tension. There's a balance to be struck
between so many factors, and every language lands in a slightly different spot.

### tl;dr

- Go is totally willing to let you do this, and is happy to help. I think this sort of
  thing is what people are talking about when they call Go "simple". You can just do
  stuff without needing to worry too much about how it works, because it's willing to do
  a lot of work for you.
- Rust will let you do it, and will do its best to make sure that you do it in a way
  which is robust and correct, it just won't do it for you. It makes you do it yourself,
  but at least it provides a lot of pieces for you to build with.
- Zig will let you do it, but _really_ makes you do it on your own. No smart pointers, no
  garbage collector, just a lot of responsibility, and a lot of control.

[rust playground]: https://play.rust-lang.org/
[rc]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=99d543f32f77883075c536d8a54ce8c3
[weak]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1695944d9f4ac060aeb0a9c606bff21b
