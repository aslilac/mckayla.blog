---
title: Thoughts about package management
author: Kayla Washburn
date: 2023.7.15
accent_color: "#f9b14d"
---

<!-- Given that work on the official package manager has begun, I thought it'd be fun to talk about package management, package authoring, and things to consider for the future. -->

## Prior art

- npm
- conan
- vcpkg
- pip
- hex
- cargo
- opam
- hackage
- elm
- ...and countless system package managers

<!-- I'm gonna talk about npm a fair bit, because I know it -->

+++

<!-- If you ask anyone who has used any of these for basically any amount of time, you'll get at least a few complaints, probably a lot. -->

## How can we do better?

We have a lot of opportunities to learn!

+++

# Known problems

+++

## The leftpad incident

<!-- If you don't know, the incident I'm referring to happened back in 2016 when the author of a package named leftpad, for padding strings, unpublished over 250 packages that he maintained because he got mad. leftpad was used by a popular project called babel, which is a JavaScript-to-JavaScript compiler, that lets you write code using new syntax, and compile it into code that will still run in older browsers and such. The moral of this story is... -->

+++

## The leftpad incident

- Don't let people delete things
- Like, ever
- Seriously

<br />

- ...but also faker (wide-spread DDoS attacks)
- ...and also node-ipc (literal cyber-warfare)

+++

## Managing upgrades

- "lock" files
  <!-- These files can be massive, package-lock.json >250kloc -->
- Supply chain security
  <!-- Downstream vulnerabilities, shady dependencies we didn't even know we had -->
- What if we need incompatible versions?
  <!--
  - I really want to use the new version, but this other dependency hasn't been used in a while and needs an older version.
  - Including multiple versions has drawbacks. Interop, size, complexity. -->

+++

## Not getting along with the in-laws

- What if I like _my_ dependency, but I don't like _their_ dependencies?
  - License conflicts
    <!-- "GPL virus" -->
  - Missing features
    <!-- HTTP/3 -->
  - Lack-luster performance
    <!-- Some new algorithm or technology has enabled doing The Thing way faster! -->
  - Disagreements
    <!-- I don't want your weird logging thing, I want you to use mine -->

+++

## Wasted cycles

- `GET https://npmjs.com/leftpad/-/leftpad-1.0.0.tar.gz` <!-- and wait -->
- `ungzip`
- `tar -x`
  - A _bunch_ of files I don't care about
    - LICENSE
      <!-- arguable, but who looks at the LICENSE files in their node_modules/? -->
    - package.json
      <!-- needed for node, but I don't care about it -->
    - CODE_OF_CONDUCT.md, CHANGELOG.md, README.md, .github/ (!?!?), .editorconfig (!?!?)
      <!-- These are files that are definitely useless, yet I'm wasting precious seconds for them to be written to my disk. -->
    - Compilation artifacts
      <!-- In the JavaScript ecosystem, if you have a TypeScript dependency, every one source file is 1:3 because you probably have source maps and definitions (which are TypeScript's equivalent of "header files") -->

...and now do this ~~hundreds~~ thousands of times

+++

## ~~Wasted~~ Dangerous cycles

- `postinstall`, `prepare`, etc.
- setup.py
- build.rs
- ...build.zig? 💀

<!--
Tons of package managers provide authors ways to run whatever code they want during installation.
In any other context, we'd consider remote arbitrary code execution to be a disasterous outcome.
Languages like Elm, Gren, Roc won't even let libraries run arbitrary code at _runtime_, let alone let some malicious package ruin your life from a single typo.
-->

+++

# Potential solutions

+++

## Don't have a package manager

<!-- I like to always keep "do we really need this?" as a question in my mind. This is probably too extreme, but don't just dismiss it as an impossibility! Think about what living in a world that looks like this would be like. How bad would it be, really? buuuut... -->

+++

### Package managers have good bits

- Code reuse
  <!-- Open source is great! Using someone else's code is a great way to benefit from all the lessons they've learned. for free! and package managers make it easier to participate. -->
- Just build the thing that you _want_ to build
  <!-- Initial simplicity, getting things working quickly, solve the problem you came here to solve. No need to fuss with submodules, maintaining a local copy, whatever. Just run a command and get the thing. -->
- Working together
  <!-- Computers are complicated. There's a lot of work to be done (documentation, testing, research, implementation). Share the weight! -->

<!-- So we need to talk about *actual* solutions -->

+++

## Avoiding dangerous cycles

- How about we just don't execute arbitrary code in the package manager/during builds? 😅
  - Come up with alternative ways to describe behaviors, rather than letting anyone do anything
    <!--
    - npm "solves" this by letting you have platform specific dependencies
    - `--ignore-scripts` -->
  - At least _ask_ the user if it's okay to just run stuff
    <!-- Ask for every package, and every version, which wants to execute something. Cache it forever, so that it can't somehow change silently and run again. Speaking of caching... -->

+++

## Avoiding wasted cycles

- Cache downloads locally
  <!-- Eventually npm added a local immutable cache -->
- Package formats
  - [ESZip]
  <!-- ESZip is a custom format that we created at Deno, which represents an entire module graph, with all modules being stored in the order that the runtime will want to evaluate them, as a single file, with source maps. -->
  - [.crate] files
  <!-- Actually just a normal .tar file, with some rules around how stuff is stored -->
  - .tar files
  <!-- Even just loading source files from an uncompressed .tar file would be great -->
- Just be aware of what you're aiming for
  <!--
  - Only upload files that match `src/**/*.zig` by default
  - ...with some mechanism for people to include other sorts of files as necessary -->

[eszip]: https://github.com/denoland/eszip
[.crate]: https://users.rust-lang.org/t/where-to-learn-about-crate-file-format/11496

+++

## Avoiding the rest...

- **What if dependencies couldn't have dependencies?**

+++

## How would that work?

```zig
test "simple test" {
  var list = std.ArrayList(i32).init(std.testing.allocator); // <-- This line here!
  defer list.deinit();
  try list.append(42);
  try std.testing.expectEqual(@as(i32, 42), list.pop());
}
```

<!-- As users of Zig, we've already seen an easy solution. This snippet of code is taken from the output of running `zig init-exe`. `std.ArrayList` *could* just make assumptions about how it should allocate memory, but it doesn't. It has a dependency, memory allocation, but it gives *you* the opportunity to specify how that should work, by giving it an allocator. This is one of the first concepts that you'll run into as a Zig author. -->

+++

<!-- ...and it's usually known as dependency injection. -->

## Dependency Injection

```zig
var client = requestz.Client.init(
  std.testing.allocator,
);
```

+++

## Dependency Injection

```zig
var client = requestz.Client.init(
  std.testing.allocator,
  std.net.tcpConnectToAddress, // <-- What if we could provide our own TCP logic!
);
```

+++

## Dependency Injection

```zig
var client = requestz.Client.init(
  std.testing.allocator,
  .{
    // ✨ fancy HTTP/3 stuff here ✨
  },
);
```

<!-- ...or more realistically, we might want to pass in a custom HTTP orchestrator that our higher level client can use. -->

+++

## Benefits!

- No lock files
- No conflicts
- Much, much simpler package manager
  <!-- All it has to do is download and install the exact packages you told it to. No building a complicated tree and trying to figure out how to best prune it and deduplicate things. No conflict resolution, just doing exactly what it's told to do. -->
- No need for weird things like "peer dependencies"
  <!-- This happens a lot with React. You have some UI library you're using, and you need to make sure it's using the same version of React as the rest of your application. If you have two dependencies, and you need to be absolutely sure they're using the same version of some third dependency as you are, just pass it to them! If you need to pass it around a lot, factor it out into a new function or module. We should be building strong systems that we're confident in. Why would we just cross our fingers and hope it doesn't go wrong? We should be making strong guarantees about these sorts of things. -->
- All of your dependencies become explicit
  <!-- You get to see the actual complexity. Nothing hiding behind the scenes. Nothing new can show up without your knowledge. Babel didn't depend on leftpad for many years, and plenty of people who were effected had *no idea* they depended on it. -->
- You own the glue code!
  <!-- If there's an incompatibility with a newer version of some package, you can fix it yourself! No need to wait for someone else. and since you already wrote the patch, maybe you can upstream it! Future versions can make everyones glue code simpler again. -->
- Flexibility and testing
  <!--
  - Maybe you're writing a backend for an app with user uploads...start off with a file-system based blob storage provider, swap out with s3 when you need to. maybe only swap out in production.
  - Postgres in production, use SQLite in tests (or at least a test oriented postgres provider)
  - You can fight over if this is a good idea or not, but I think it's good for individuals to be able to make these sorts of decisions for themselves, based off of their own circumstances, preferences, and tolerance for complexity.
  - Mock your HTTP requests with local files instead
    - If I'm writing tests for an API client, I don't care about if we're actually hitting a real server during the tests, I care about my API client responding appropriately to certain responses and conditions, and those might actually be easier to control if they _don't_ go over the network. -->
- `comptime` means we can have this flexibility without runtime cost

+++

**I don't actually think entirely disallowing packages to have dependencies is the right approach**

...but I think these kinds of things are important to keep in mind as the package ecosystem develops for Zig.

<!-- Maybe it is the right approach! If there was a language that could pull it off, it would be Zig. Maybe we should try it. I *do* at least think that it's a nice ideal to strive for. If you're authoring a package, and you find yourself wanting to add a dependency, you should think about what the best way to get that dependency is. It might be to pick a package and use it directly, but it might not be. Don't assume, decide. -->

+++

## Less dramatic idea speed-round ⏱️

- Maybe don't just trust random peoples code blindly
  - Moderation?
  - Moderation, only for packages that require ACE as an installation strategy?
  - Moderation, only for unverified authors?
  - No moderation, but author verification?
    <!-- The problem with any amount of moderation is that somebody has to do it. How do they get paid? How do we pick people we trust to wield this power? -->
- What if adding a new dependency was a breaking change?
- What if adding a new maintainer was a breaking change?
- What if upgrading was more interactive?
  - Maybe `zig upgrade` gives you a decision tree...
    - "This major version is a breaking change because..."
      <!-- This is totally possible. Elm has had this for years! -->
    - "Hayleigh has been added as a maintainer"
      <!-- Might not actually be that useful, since lots of packages get published from CI/CD systems these days, but I think it's still an intersting idea. Maybe it could be set up to notify you when new people get write access to a Github repo. -->

+++

<!-- As Richard said in the episode of Software Unscripted that motivated this talk... -->

## It's a culture thing.

- It's up to everyone to cultivate an ecosystem where we use dependencies responsibly
- ...where we can trust and enjoy the dependencies we get to use
- ...where we can enjoy creating new libraries for others
- ...where we don't have over complicated, prolific libraries that replicate a single function from the standard library :^)

+++

![String.prototype.padStart() documentation on MDN](https://cdn.mckayla.cloud/-/evu7y733kyx4xd/padStart1.webp) ![String.prototype.padStart() compatibility information on MDN](https://cdn.mckayla.cloud/-/evu7y733kyx4xd/padStart2.webp)

<!-- This is a bit tongue-in-cheek, because `padStart` is an addition to the language that came *after* the leftpad incident; these are all versions from the year or two after. But the note I want to end on is this... -->

+++

<!-- As we enter an era of packages for Zig... -->

If you're writing a package, keep the language you are working with in mind. **A language that focuses so heavily on low-level control deserves a package ecosystem that gives you the same kind of respect.**

<!-- Zig is a unique language, which deserves a unique package ecosystem. -->

<!-- Questions? are we doing Q&A at this? -->
