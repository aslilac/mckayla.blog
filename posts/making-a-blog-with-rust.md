---
title: Making a blog with Rust
author: Kayla Washburn
date: 2022.11.1
summary: I used Rust to make this blog; but why?
tags: programming, rust
cover:
  avif: https://cdn.mckayla.cloud/-/16a595829d914009bdc6f49d101c37a9/cover.avif
  webp: https://cdn.mckayla.cloud/-/16a595829d914009bdc6f49d101c37a9/cover.webp
  default: https://cdn.mckayla.cloud/-/16a595829d914009bdc6f49d101c37a9/cover.jpeg
---

I'm a TypeScript developer by trade. I've been writing TypeScript professionally for
years, and there are [countless][next] [options][nuxt] [for][svelte] [static][jekyll]
[sites][hugo] [and][gatsby] [blogs][docusaurus] out there that I could've used, and that
would leverage my experience. A [bunch][vercel] [of][netlify] [companies][render] will
even automate your builds and host your blog for free! and isn't Rust, like, really hard?
Don't people only use it for performance sensitive things? It's also notorious for having
slow build times, so wouldn't the performance benefits be negated by the bad build times
if all I'm using it for is *build*ing a static site? Was I determined to build my blog on
hard mode?

## Performance

> wouldn't the performance benefits be negated by the bad build times?

Yes. The build for this blog is slower than an equivalent TypeScript blog would be.
It's quite ironic; but performance isn't why I picked Rust. It's also not _that_ bad.
Even a completely cold build (including downloading dependencies) still takes less than
one minute, and warm builds take less than a second, so it's not really an issue. As far
as production goes, the build runs once, generates all of the static files, and those are
then served from a CDN, which is essentially as fast as you're ever gonna get.

## Why Rust then?

Alright,

Wanna try out framework behind this blog? It's called [Pocky]

[docusaurus]: https://docusaurus.io
[gatsby]: https://www.gatsbyjs.com
[hugo]: https://gohugo.io
[jekyll]: https://jekyllrb.com
[netlify]: https://www.netlify.com
[next]: https://nextjs.org
[nuxt]: https://nuxtjs.org
[pocky]: https://crates.io/crates/pocky
[render]: https://render.com
[svelte]: https://kit.svelte.dev
[vercel]: https://vercel.com
