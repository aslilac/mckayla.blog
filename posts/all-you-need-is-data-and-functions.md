---
title: All you need is data and functions
author: Kayla Washburn
date: 2023.4.1
summary: Thriving in a world without traits or interfaces
tags: programming, rust, gleam, traits
accent_color: "#ffaffe"
cover:
  avif: https://cdn.mckayla.cloud/-/2577cf423ed0449181007c0ade133be3/Gleam-Dark.avif
  default: https://cdn.mckayla.cloud/-/2577cf423ed0449181007c0ade133be3/Gleam-Dark.webp
---

It's really easy to tend towards complexity as engineers. I think on some level, we love
complexity. There are obviously bad types of complexity, but I think there are other types
of it that we seek out, because there's something satisfying about wrapping your head
around it; and I think _a lot_ of that kind of complexity ends up in our programming
languages.

## Traits

Over the last year, I've been fairly involved with a young programming language named
[Gleam]. It's designed around a functional programming style, and the core language is
simple enough to wrap your head around in only a day or two. People like to say
that a lot, and I've heard it a lot about languages like [Go] and [Zig] (that _definitely_
didn't click for me right away), but Gleam really seems to be the kind of language that
people can just pick up and start being productive with.

The language is simple to the point where it's a bit of a point of contention; although
"language complexity" almost always is. Gleam doesn't even have `if` conditions, and
instead encourages you to use pattern matching. There are some things that might come
along eventually, like optional arguments and a hygienic macro system, but there are other
things that have been explicitly omitted from the language, like traits.

I find [trait systems][rust traits] like [Rust]'s to be incredibly intuitive. They enable
your code to be quite generic _and_ composable, in a way that I personally think feels
quite nice. Interfaces in languages like [Go][go interfaces] or [C#][c# interfaces] fill
very similar needs. The exact details vary slightly, but the idea is simple: **give
developers a way to talk about shared behavior across different types.**

For quite a while, I felt like traits were a glaring omission from the language, and I
brought it up in the [Gleam Discord] several times. It would end pretty much the same way
every time. "They don't allow you to express anything new, and they'd add a lot of
complexity to the language." I always found this a bit frustrating. Every time I thought
I had a compelling enough use case for traits, it was never enough to sway the others. It
was pretty defeating, because I never really understood what the alternative was. How can
I express myself generically in a language like Gleam, with a nominal typing system and no
support for traits or interfaces?

## ...are just types.

Let's take a simple Rust trait for example, [`Display`][rust display]. If you've written any amount of
Rust at all, you should recognize this trait. For those who haven't, it's a core trait in
the language similar to [`Show`][haskell show] in [Haskell], or any object with a [`toString`][javascript tostring] method in
JavaScript. It just allows you to describe how you could convert some value into
human-readable text. For example...

```rust
struct Friend {
  name: String,
}

impl Display for Friend {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "Hi, my name is {}!", self.name)
  }
}
```

Hopefully everyone can understand what's going on here. We're just saying "given something
to write to, and a formatter provided by the language as our intermediary, write this text."

We can now use this implementation like so...

```rust
let louis = Friend { name: "Louis" };
println!("{}", louis);
```

...to write the text "Hi, my name is Louis!" to stdout! The Rust compiler knows that any
instance of `Friend` can work with `println!` by using the implementation of the `Display`
trait that we provided it, so how might you express the same thing in Gleam?

```gleam
pub type Friend {
  Friend(name: String)
}

pub fn to_string(self: Friend) -> String {
  "Hi, my name is " <> self.name <> "!"
}
```

...and usage would look like...

```gleam
let louis = friend.Friend("Louis")
io.println(louis |> friend.to_string())
```

It might not be immediately obvious (I know it wasn't for me), but I think this example
can help you begin to pull away the veil: **traits are just types**. Our `Display` trait in
this example, can be represented by the `String` type, and a function which converts from
our original type to a `String`.

## Wait, what?

I imagine that plenty of people won't be convinced by this. `Display` provides more
functionality than just converting to a string after all, it also allows you to specify
details about how that string should look! But a function can do that as well!

```gleam
pub type DisplayOptions {
  DisplayOptions(
    precision: Int,
    alignment: Alignment,
    // ...and so on
  )
}

pub fn format(self: Friend, options: DisplayOptions) -> String {
  ...
}
```

If you want it to look more like a Rust trait, you could shuffle this just a little and
introduce a new `Display` type to make you feel more at home.

```gleam
pub type Display {
  Display(formatter: fn(DisplayOptions) -> String)
}

pub fn format(self: Display, options: DisplayOptions) -> String {
  self.formatter(options)
}
```

...and a function to convert from our `Friend` type to this new `Display` type.

```gleam
pub fn to_display(self: Friend) -> Display {
  Display(fn(options: DisplayOptions) { ... })
}
```

Now to put it to use, you'd just do something like...

```gleam
let louis = friend.Friend("Louis") |> friend.to_display()
io.println(louis |> display.format(DisplayOptions(...)))
```

It requires a few more moving pieces, but it also looks quite a bit more like the traits
you might be used to. If you're still not convinced, let's look at another example.

## No, really, they're just types.

Obviously I'm being a bit simplistic when I say that traits are just types, but they're
really not all that different. Take for example the [`Iterator` trait][rust iterator] in Rust and the
[`Iterator` type][gleam iterator] in Gleam. They both accomplish the same thing, just in slightly different
ways; and honestly it has more to do with the fact that data is immutable in Gleam than
the fact that it's a type instead of a trait.

We'll start again with Rust, and we'll do our comparison by defining an iterator which
produces the Fibonacci sequence.

```rust
struct Fib(u64, u64);

impl Default for Fib {
  fn default() -> Self {
    Self(0, 1)
  }
}

impl Iterator for Fib {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    (self.0, self.1) = (self.1, self.0 + self.1);
    Some(self.0)
  }
}
```

You could use this like so...

```rust
let fib = Fib::default()
  .take(10)
  .collect::<Vec<_>>();
println!("fib: {:?}", fib);
```

Simple enough! We defined a type to represent our state, and a function which mutates our
state into the next state and returns our iterator value.

In Gleam, our implementation looks like...

```gleam
fn next(state: #(Int, Int)) -> #(Int, Int) {
  let #(a, b) = state
  #(b, a + b)
}

pub fn new() -> iterator.Iterator(Int) {
  #(0, 1)
  |> iterator.iterate(next)
  |> iterator.map(pair.second)
}
```

The important difference here, is that our `next` function doesn't mutate state, and
instead returns the next state. Because our state requires two integers, and we want our
output to only be one integer per iteration, we use `iterator.map` to transform from our
internal state to our external interface. Like I warned before, this has much less to do
with traits vs types, and much more to do with immutability.

You could use this iterator like so...

```gleam
fib.new()
|> iterator.take(10)
|> iterator.to_list()
|> io.debug()
```

Note how similar that usage is to the Rust version! We still get to provide nice, generic
interfaces for using our type to anyone who is able to convert from their own type to our
`Iterator` type (like `take`), and we didn't need a trait system to accomplish it.

The biggest difference between most trait/interface systems and trait-types is that
the language is essentially doing implicit conversions for you, from your data-type to
the trait-type.

## What about composition?

Hopefully by now you're convinced that individually, types have as much to offer as
traits. But one of the most important aspects of traits is their ability to be composed.

Say for example, you have a function which should be able to accept any iterable object
that yields values which are `Display`. In Rust, you can use traits together to describe
more complex mixtures of behavior while still remaining generic. You could express this by
saying...

```rust
fn print_things_from_an_iterator<I>(iter: I)
where
  I: Iterator,
  I::Item: Display,
{
  for it in iter {
    println!("{}", it);
  }
}
```

You might expect this kind of generic program to be harder in Gleam without traits, but
honestly it isn't. Just define a function that expects your trait-types!

```gleam
fn print_things_from_an_iterator(
  iter: iterator.Iterator(Display),
) {
  use it <- iterator.each(iter)
  {
    it
    |> display.format(DisplayOptions(...))
    |> io.println()
  }
}
```

Which we can use by doing something like...

```gleam
["Louis", "Hayleigh", "Kayla"]
|> iterator.from_list()
|> iterator.map(Friend)
|> iterator.map(friend.to_display)
|> print_things_from_an_iterator()
```

Compare this to Rust's...

```rust
let names = ["Louis", "Hayleigh", "Kayla"];
let friends = names.iter()
  .map(|name| Friend { name });
print_things_from_an_iterator(friends);
```

Really the only difference is that we manually convert from `Friend` to `Display` in
Gleam, where Rust already knows how to use `Friend` as a `Display`.

## tl;dr

Instead of a trait, just make a type that implements the generic behavior you want, and
then write a function to convert your data-type into your trait-type. If you need some
data-type specific logic, then pass around functions as necessary (usually from your
conversion function).

Traits make sense in Rust, because the alternative I've proposed wouldn't work well with
Rust's memory model or focus on performance, and using functions in this first-class manor
would be much more complicated. For a language like Gleam, that doesn't have those concerns,
but that _is_ very concerned with being simple and keeping concept count low, they don't
make as much sense. After all, all you really need is data and functions. :^)

---

If you've never heard of Gleam, you should check it out! It's a neat little language
with a lovely community around it. Come hang out with all of us in [Discord][gleam discord]!

[c# interfaces]: https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/types/interfaces
[gleam]: https://gleam.run
[gleam discord]: https://discord.gg/Fm8Pwmy
[gleam iterator]: https://hexdocs.pm/gleam_stdlib/gleam/iterator.html
[go]: https://go.dev
[haskell]: https://www.haskell.org/
[haskell show]: https://hackage.haskell.org/package/base-4.18.0.0/docs/Prelude.html#t:Show
[go interfaces]: https://gobyexample.com/interfaces
[javascript tostring]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
[rust]: https://rust-lang.org
[rust display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[rust iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[rust traits]: https://doc.rust-lang.org/book/ch10-02-traits.html
[zig]: https://ziglang.org
