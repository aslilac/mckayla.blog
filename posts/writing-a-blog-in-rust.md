---
title: Making a blog with Rust
author: Kayla Washburn
date: 2022.11.1
summary: I made this blog using Rust; but why?
---

I'm a TypeScript developer by trade. I've been writing TypeScript professionally for
years, and there are [countless] [options] [for] [static] [sites] [and] [blogs]
out there that I could've used, and that would leverage my experience. A [bunch] [of]
[companies] out there will even build and host your blog for free! and isn't Rust, like,
really hard? Don't people only use it for performance sensitive things? It's also
notorious for having slow build times, so wouldn't the performance benefits be
negated by the bad build times if all I'm using it for is *build*ing a static site?

![](https://cdn.mckayla.cloud/-/16a595829d914009bdc6f49d101c37a9/cover.avif)

## Performance

> wouldn't the performance benefits be negated by the bad build times?

Yes. The build for this blog is slower than an equivalent TypeScript blog would be.
It's quite ironic; but performance isn't why I picked Rust. Even a completely cold build
(including downloading dependencies) still takes less than one minute, and warm builds
take less than a second, so it's not really an issue. As far as production performance,
the build runs once, and the generated static files are served from a CDN, which is
essentially as fast as you're ever gonna get, and not any different than a typical
blog written in TypeScript.

## Why Rust?

Alright,
