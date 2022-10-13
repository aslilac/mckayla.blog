---
title: A Tale of Two Clang
author: Kayla Washburn
date: 2019.8.10
summary: Why C++ makes me sad
tags: programming
---

# Author's Note

Don't take this post too seriously. I wrote this post a long time along, when my
understanding of C++ was much more basic. I've learned a lot between then and now, and as
it turns out, a lot of problems I had with the complexity involved in compiling a C++
program are platform specific.

- The experience of getting up and running with either GCC or Clang on Linux is actually
  fine, as I have since learned.
- I acknowledge that the "all in one installer experience" that I asked for does exist on
  macOS and Windows. I just wish they were separate from Xcode and Visual Studio.
- It'd be even nicer if there was a cross-platform installation experience, which would
  provide the same compiler feature set, behavior, and command line options (which as I
  mention, is something that Rust has, and which I greatly appreciate)
- You can actually install Clang through Visual Studio these days (which is cool!), but
  you have to know to do it, so people trying to learn will undoubtably still get tripped
  up by MSVC's command line options. Compatibility makes this a hard problem to solve,
  unfortunately.

---

# Prologue

I really dislike Microsoft Visual Studio. I find it to be vastly over complicated and hurt
more than it helps, particularly for teams who need to work across multiple operating
systems. Despite this, there are some things that it is an absolute essential for when it
comes to Windows development (like if you want real debugging), and there are times when
Windows development itself is an absolute essential (like when a boss or professor tells
you that you have to).

My primary machine is a MacBook Pro, which means I primarily use macOS. I mostly do full
stack web development, and I find the experience of it to be better on a Unix machine.
Linux would be ideal as that’s what most servers run, but that has some problems of its
own if you want to do more than *just* programming.

<figure>
<img alt="a snippet of code showing a simple C++ program"
	src="https://cdn.mckayla.cloud/-/713b9687c4974abebb6b9dbb1a02bb5a/hello.avif" />
<figcaption>How hard could it be?</figcaption>
</figure>

# Setting up a compiler should be easy, right?

Some time ago when I was in a C++ class on data structures at my university, I needed to
submit a Windows executable as a part of each assignment. No big deal, I have a Windows
VM on my laptop and a Windows PC at home, so I could figure out something.

The obvious answer (and what my teacher recommended) was to use Visual Studio inside of
my VM. I didn’t really want to do this though, because Visual Studio is incredibly heavy
to run, a heavy battery drainer (especially when VM overhead is adding on to it) and
working inside of it is the kind of thing I have nightmares about. It’s huge, bloated and
getting it to do what I want feels like pulling teeth. I only had a few goals for my
solution: simple and cross-platform.

So I first looked at my options. There are two main options for cross-platform C++
compilers: Clang and GCC. Clang ships with Xcode’s Command Line Tools on macOS, and has a
Windows installer available directly from the developers. GCC is available through
Homebrew on macOS, but since Homebrew requires Xcode (and thus we would already have
Clang) and GCC on Windows requires Cygwin (which is objectively not simple), Clang was the
obvious choice. So I just run the Windows installer and I’m good to go right?

# Nope.

Here’s the thing about Clang on Windows: Clang is just a compiler, without any runtime or
function definitions. They do have libc++, their implementation of the standard library,
but there isn’t a precompiled Windows version, and I didn’t have a working compiler.
No help there.

I spent longer than I care to admit trying to find a way to work around this without
installing Visual Studio. I wanted a simple solution, and installing a several gigabyte
IDE just for the standard library headers felt incredibly over complicated. I begrudgingly
did it anyway.

# Things are complicated.

After thinking about it, I realized that macOS made me do the exact same thing, just with
a different IDE. I had Xcode installed, and I virtually never touched it. It’s no small
package either, weighing in at over a dozen gigabytes itself. All this, just so that I can
have access to `clang++` from my command line. It makes sense though. C++ comes from a time
when this decoupling was necessary. Target platforms weren’t as homogenous as there are
now, and intermediate representations were a twinkle in the eye of an engineer somewhere,
wishing machines had enough RAM to support them.

Today, this coupling makes for a very confusing experience though for someone who is just
trying to learn how to program in C++. There are so many choices, and so many of them
obfuscate what the computer is really doing behind layer upon layer of abstractions.
Working directly with a compiler yourself ends up being a tedious process, because nobody
talks about how to do it anymore; they only talk about the abstractions. When people only
talk about the abstractions things at the lower level get lost inside of them or left
behind. It doesn’t have to be like this though!

# Setting up a compiler should be easy.

Node.js, while not a compiler, is something that I work with frequently, and has probably
spoiled me a little. They have installers for all platforms with the only thing you need
to get going: an executable that runs your code. It also includes npm, but you don’t have
to install it if you don’t want to. The point is that Node.js does a lot of work to make
the out of the box experience as seamless as possible.

Some newer compiled languages, even low level ones like Rust, give you the same single
package treatment. If you want to program in Rust you have to install a small tool called
rustup. It’s a version manager, and bundles a few other tools together, like cargo. But if
you just want to use rustc directly, it doesn’t force you to pay a ton extra. It’s all
small, doesn’t get in your way, and most importantly is useful. Cargo (the package
manager) works at a low enough level to feel like you’re in control and you know what is
happening, even though it abstracts a lot of extra work away. You run it directly from
the command line, and you don’t have to dig through menus, menus, and more menus to
configure it the way you need.

It would be nice to see a big player in the C++ world step up and offer something like
this: An all in one installer experience for Windows and \*nix that only gives you useful
command line utilities, a precompiled standard library, and none of the other fluff.
I know it’s wishful thinking, but a girl can dream.
