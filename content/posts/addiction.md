---
title: Addiction.
author: Kayla Washburn
date: 2023.6.15
status: unlisted
accent_color: "#5c6"
cover:
  avif: https://cdn.mckayla.cloud/-/44v3h82e68i26z/DSC02213.avif
  default: https://cdn.mckayla.cloud/-/44v3h82e68i26z/DSC02213.webp
---

I'm admitting to myself that I'm an addict. Not to a substance, but to a couple seemingly innocent activities that have become compulsive behaviors, and that I have an incredibly hard time resisting. They have had serious, long term, negative impacts on my life. They started off innocent enough, and even once I realized they were hurting me I made excuses. I thought about all the time I was able to keep things healthy and balanced, and I just needed to work on getting back to that. I'm finally acknowledging that framing it that way will never result in things improving.

I'm being vague up front, because I genuinely don't even know how to describe these addictions to another person without sounding crazy, or spending an hour on it. I've spoken with a couple of close friends about it though, and they're the sorts of things that other other ADHD people might relate to, though taken to a bit of an extreme.

## YouTube

I have for many, many years, been an avid [YouTube] watcher. As a conservative estimate (since YouTube doesn't actually provide users with the kinds of "watch time" statistics that creators get), I've spent over 10,000 hours watching videos on the platform over the last 7 years. I estimate that by assuming that I've watched an average of 4 hours of videos every day over that length of time; which again, I would say is conservative. That's **426 days**. It's **16.7%** of the time I spent being alive. Again, I believe this estimate to be low.

The reason that amount is so high, is because when I'm watching something on YouTube it's almost never the _only_ thing I'm doing. For years, I successfully used it as background noise to keep my ADHD riddled brain occupied enough that I could focus on more mundane things, like chores. One thing I watched a lot of was people playing _[Mario Maker]_, a game I thought was interesting, but that I'd never played because I never had a Wii U. The content was long (mostly uploads of live streams), low-stakes (if I walked out of the room for a couple minutes, I wouldn't be missing much), and plentiful. Harmless! I could support some very niche creators, have some nice background noise, and it seemed to have a positive impact on my life. It fit into my routine nicely, and I knew I was spending a lot of time watching YouTube, but it never felt like a bad thing. I could stop when I wanted, and I was still doing all of the adult things I needed to be doing, like completing my homework, showering, doing laundry, etc.

It's hard to pin-point exactly when it went wrong, and I want to resist the urge to blame COVID-19, but I really think that's when the decline started. It stopped being background noise that gave way to more important things, and started being the way I filled all of my time since I couldn't really hang out with friends. The lack of social interaction made me _really_ depressed, since as much as I might have not wanted to admit it at the time, I am a very social creature. As anyone who's ever struggled with depression can surely vouch, it can make it really hard to function. YouTube started to not just be background noise for the mundane, but for basically everything. I started watching it nearly all day every day. I'd watch it while I ate, while I fell asleep, while I was at work; anything to avoid being alone with my thoughts.

As time went on, the content I watched started to change as well. I barely watch any streamers or video game related videos any more, and now watch a lot of math/science/educational content and video essays. This is to say, I shifted away from content that worked well as background noise, and towards content that is a bit more cerebral. Harder to walk away from because I might miss something important, harder to stop watching in the middle because I get invested and want to finish, and just generally much more distracting when trying to focus on something else. The content changed, but the way I consumed it didn't. I was still acting like it was background noise, like I'd still be able to focus on other things, like I could watch 4+ hours of it in a day in a way that was healthy. It always worked just fine while I was in school, why would it be an issue now? The change happened so gradually that it was hard to notice it ever changed at all, but none of that was really true anymore.

## ...Coding?

In October of 2020, I created a repo on Github which has now become known as [xaslilac/sandbox]. It went by a few other names before, but I think sandbox is really quite an accurate name for its purpose: for me to play around and build in, without any real end goal or purpose. The roots of the repo actually go back a little bit farther than that, (to January of the same year, right before COVID-19 closed the city of San Francisco) when my roommate convinced me to try C, which I had used before. I hacked together a little app that generated some HTML in C one night to see what it was like, and I was surprised at how much I enjoyed it. Over the course of the next four years and 1,000+ commits, that little C program would eventually be joined by similar programs written in...

- Systems languages like C++ (and cpp2, and Circle C++), Rust, Zig, Swift, Go
- .NET languages like C#, F#
- ML languages like Elm, Haskell, OCaml, ReasonML
- BEAM languages like Erlang, Elixir, Gleam
- Niche languages like Hare, Io
- JVM languages like Java, Kotlin
- Older languages like Perl, PHP, Objective-C
- Newer languages like Odin, Unison
- Other popular languages like Julia, Lua, Ruby, SQL
- x86_64 Assembly (in both AT&T and Intel syntax), and aarch64 assembly.
- SQL (for Postgres and SQLite)

Most of the programs are pretty small, but the are over 10,000 lines of code between them, and I've put a lot of time into configuring build systems and CI for each of the examples as well. I used Github Actions for CI, and the main workflow file is 1,700 LOC by itself, even after I factored a lot of the examples out into smaller, modular actions. Each language present represents hours of setup, learning, and trial and error. It's become my default thing-to-do when I'm bored, resulting in countless hours spent on it. I don't even know how I'd approximate this one, but I'm sure it'd be many thousands of hours, much like my YouTube consumption.

I had unknowingly crafted the perfect drug for myself. Something that felt like learning (while providing minimal value to my life outside of itself), and that gave me plenty of opportunities to feel that "I did it!" rush of dopamine through my brain whenever I got something to work. All the programs were small, but even doing something as simple as reading a file or making an HTTP request still felt like a new and exciting achievement as long as the programming language that I was doing it from was new. It was basically an endless source of that same feeling that made me fall in love with programming in the first place, and I had gamified it for myself.

As someone who loves learning, this was an easy trap to fall into. In hindsight, I can see that I didn't actually learn all that much from all of the time I spent in this repo, it just _felt_ like learning. I've written code in dozens of programming languages now, but I'm still only competent in like 3 or 4 of them at most, and I definitely haven't mastered any of them thanks to all of this work. I spent literal days writing a "Hello world!" program in assembly for ARM64 Windows. It was grueling, required far too much digging, trial and error, and frustration, and I learned nothing of value from it. Even if I worked on Windows or LLVM, I'd either probably be generating assembly binaries directly, or writing code in C++ that'd get compiled to assembly by code someone else wrote. It's such a specific area of expertise to try to gain knowledge in, and I didn't even really learn anything! It's too simple of a program to really glean anything important from, and it was endlessly frustrating to get working.

## The vicious cycle

The way that these two play off of each other is particularly rough. My YouTube habit combined with the change in content means I spend a lot of time sitting on my couch. If I have my laptop with me, I'll inevitably reach for it to work on something while I let YouTube play in the background. In the absence of anything more compelling, I'll end up working on something in sandbox. If I get bored of sandbox, I might as well finish the video I'm watching, right? and since I'm sitting here with my laptop and sandbox is already open, I might as well pick something to play with. Oh, that video is over? Well, now I'm in the middle of working on something, so I'll just have to pick another video to watch.

I hope you can see how this devolves into an entire day of sitting on the couch and accomplishing nothing of value _very_ quickly, while starting off as something seemingly innocent. I'm just gonna watch some YouTube while I eat lunch, and _then_ I'll get back to work..right?

## Fixing my brain

Hopefully, I've painted a compelling enough picture so far that you can see how each of these started, and how they became serious problems without me realizing. I have at this point though, known that YouTube and my sandbox repo have been serious problems for at least a year, and have failed to take it seriously until now. This time I'm serious about breaking out of my self-destructive rut. I've talked with my partner about the way that I've been struggling with this, and she's supportive and wants to be helpful. I couldn't bring myself to just delete sandbox, but know that having access to it is dangerous, so I've instead transferred the repository to a close friend, and deleted all of my local copies. She's agreed to give it back to me in a month if I want it, after I've had time to break the habit. I'm hoping to come up with a way to preserve it for myself, without reintroducing the temptation. Luckily I'll have many more spare cycles in the upcoming month to figure out what that might look like.

At this point, I've been depriving myself of healthy levels of boredom for years. I caught up on laundry for the first time in over a year last week, and have only shaved my legs a handful of times in the last couple years. I don't shower as regularly as I used to (not to an entirely gross level, but I used to shower daily), and I've gotten really bad at helping with the dishes. So many video games I've wanted to play more of, so many friends I wish I'd spent more time with, so many opportunities I've lost. It really sucks to see it all lost to such pointless filler. I haven't even mentioned the toll on my mental health that watching so many leftist video essays has taken. I feel so hopeless and disillusioned with the state of the world. I wish I could be more optimistic about it, but especially over the last year I've felt so powerless to myself and my own problematic state, how could I expect anything else to get better if I can't even help myself?

But I'm actually feeling kind of hopeful, which is a feeling I haven't had a lot of in the last several years. Wish me luck.

[youtube]: https://youtube.com
[xaslilac/sandbox]: https://github.com/xaslilac/sandbox
