+++
title = "My Week In Amethyst"
author = "doomy"
date = 2019-05-28T01:20:22.892Z
description = "N/A"
+++
It's been a while! I haven't been not-busy, but writing is effort, u kno?

Well to make up for that, I'm going to be documenting progress using & learning the Amethyst game engine.

## Amethyst
So, what is Amethyst? [It's a game engine](https://amethyst.rs/) written with the Rust programming language. I've been involved as a volunteer since winter, and I've been learning the engine itself more and more each day.

Hopefully by documenting my progress here, I can document early issues I ran into, and how to solve them.

## Game
To be honest, I don't really have that perfectly nailed down right now. My goal is to just make *something* that I can build off of. At first, I tried a simple DDR/FFR style rhythm game. This was a great exercise, but it was so messy by the time I was done with it (probably a good sign I learned a lot).

## Starting out 
Amethyst is built on `specs`, an ECS (Entity Component System). If you've used something like Unity in the past, you might be partially familiar.

I thought it'd be smooth sailing, since I *had* used Unity in the past, and ECS was a term I was comfortable with. For the most part it was - however, Unity's "ECS" has a few distinct differences.

In Unity, when you want to write a new script, you often use `MonoBehaviour`. This file would contain a definition of the component, as well as how that individual component should behave. 

In Amethyst, we see the "S" in "ECS" more often. Instead of having a component describe its own implementation, we use Systems. While I won't go into depth about the topic since the [Amethyst book covers these topics much better than I could](https://amethyst.rs/doc), let me give you a general gist of systems.

Whereas some engines like Unity define individual implementation, systems define how *all* things of a certain type should behave. This makes it super easy to benefit from certain CPU performance improvements later on.

## Hurdles
Okay. When I said my first game attempt in Amethyst was an FFR clone, I lied. It was actually going to be a text-based visual novel. As is often the case with game development, I planned a huuuge scope while I was still learning an engine. Turns out this usually is not a good idea! My first lesson here is don't even *pretend* like the first few games you make to learn an engine will last long. It's important to be messy, make mistakes, learn and move on.

So that brings us to our FFR style game. The idea was to create something *much* simpler. The game would start with a song at a constant BPM, a file would describe what "arrows" would show, and input was limited to keyboard movements. While I never attained this goal either, I actually got kind of far!

