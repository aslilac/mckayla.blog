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

I finally did it! I finally managed to tunnel through the decision paralysis and get a working blog setup that I am quite happy with. It's home grown and written in Rust, with the help of [a][serde] [few][serde_yaml] [great][pulldown] [packages][handlebars] from the community.

There are [countless][next] [options][nuxt] [for][sveltekit] [static][jekyll] [sites][hugo] [and][gatsby] [blogs][docusaurus] out there that I could've used; and for some of them there are even a [bunch][vercel] [of][netlify] [companies][render] that will automate your builds and host your blog for free! and isn't Rust, like, really hard? Don't people only use it for performance sensitive things? It's also notorious for having slow build times, so wouldn't the performance benefits be negated by the bad build times if all I'm using it for is building a static site? Was I just determined to build my blog on hard mode?

## Reinventing the wheel

I had one overarching goal: an easy authoring experience, without sacrificing hackability. I'm a hacker to my core. I love to poke at things, tear things apart, and shuffle around the parts as I put them back together. I tried to get started with a couple existing tools, but ultimately I just wasn't having fun.

- [Next] is great, and I use it for a bunch of different things, but it's a React framework, not a blog framework. I don't want to author my blog posts as React components, and I didn't want to deal with all of React and Next's complexity just to generate a site from some .md files. [Nuxt], [Gatsby] and [SvelteKit] are similar stories.
- Both [Jekyll] and [Hugo] could probably accomplish what I want, but they also have a huge surface area, and I didn't want to sink days into learning them just to decide that they weren't right for the job. I'd also describe them both as "config first", but I prefer the "code first" approach of [Next] and the like.

In the spirit of "easy authoring", what I really wanted was a framework that would allow me to write a .md file, push it to Github, and have the result be published without any additional intervention. No fussing with databases or a CMS. Just Markdown and Git.

In the spirit of "hackability", I wanted deep control over how the resulting site was generated. No theming system or looking through documentation for config options. Just HTML and CSS (with some Rust to make it all happen).

While I did reinvent _some_ of the wheel, I also avoiding reinventing as much of it as possible. I used an existing (and excellent) Markdown parser called [pulldown], and a Rust version of [handlebars] for templating. One of the features pulldown doesn't have is [frontmatter] support, but luckily there's [serde_yaml], and it was pretty easy to create a very simple parser to separate the metadata from the markup, which could both then be passed to the approriate _real_ parsers to do the actual work. Building things yourself is okay, but building your own buggy Markdown parser, instead of using an existing one that does what you need, is a great way to make sure you never make any progress on writing posts for your Markdown powered blog.

## Why Rust though?

If you haven't used Rust in the last couple years, I would highly recommend giving it a chance. It's incredibly easy to get off the ground with, which is something I really value. TypeScript is the language I've used the most, and I love it, but it has a lot of problems. Specifically, I think anyone who works with it will agree that tooling is currently a pain. TypeScript tooling ranges from "slow and inflexible" to "less slow but taped together", and the amount of configuration required before you can actually start doing any work is a massive pain. It kills my motivation to start a new project when instead of getting work done I just end up faffing around with build tooling. Rust, on the contrary, only requires that I run `cargo init`. I can start writing code immediately, without worrying about linting, code formatting, package management, or build tooling, because `cargo` does all of that for me.

It's also just a fantastic language. There's good reason for its consistently high rankings on StackOverflow surveys, and for all of the evangelists out there. The trait system feels like what object-oriented program was supposed to be, and like what interfaces in languages like TypeScript and Go want to be. Passing errors around as values is a much more intuitive approach to error handling that I wish every language could embrace the way Rust does. There might not always be a package for exactly what you need, but you might be surprised by how often you will find one.

Rust doesn't actually have very many ideas or features that are new or unique. But it mixes all of its parts together in a way that feels cohesive and magical. It is so much more than the sum of its parts, and it manages to maintain the fun of programming for me in a way that makes other languages feel dull in comparison.

### tl;dr

If there's one thing you take away from this post though, I want it to be this: **Rust is great for so much more than performance**. It has a few quirks that tend to confuse people initially (like the fact that it has so many different string types), but once you start to get a little more comfortable it's an incredibly productive programming language for just about anything, big or small.

<!--
<aside><em>If you've been wanting to learn Rust, but have been scared off by talk of "the
borrow checker", I'm working on a series of posts where I hope to show that it isn't all
that scary.</em></aside>
-->

---

Wanna try out the framework I made as a part of this blog? It's called [Pocky], and I'd love to hear about anything you make with it!

[docusaurus]: https://docusaurus.io
[frontmatter]: https://jekyllrb.com/docs/front-matter/
[gatsby]: https://www.gatsbyjs.com
[handlebars]: https://crates.io/crates/handlebars
[hugo]: https://gohugo.io
[jekyll]: https://jekyllrb.com
[netlify]: https://www.netlify.com
[next]: https://nextjs.org
[nuxt]: https://nuxtjs.org
[pocky]: https://crates.io/crates/pocky
[pulldown]: https://crates.io/crates/pulldown-cmark
[render]: https://render.com
[serde]: https://crates.io/crates/serde
[serde_yaml]: https://crates.io/crates/serde_yaml
[sveltekit]: https://kit.svelte.dev
[vercel]: https://vercel.com
