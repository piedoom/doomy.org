+++
title = "My Week In Amethyst"
author = "doomy"
date = 2019-05-28T01:20:22.892Z
description = "N/A"

[taxonomies]
categories = []

[extra]
tags = []
+++
It's been a while! I haven't been not-busy, but writing is effort, u kno?

Well to make up for that, I'm going to be documenting progress using & learning the Amethyst game engine.

## Amethyst

So, what is Amethyst? [It's a game engine](https://amethyst.rs/) written with the Rust programming language. I've been involved as a volunteer since winter, and I've been learning the engine itself more and more each day.

Hopefully by documenting my progress here, I can document early issues I ran into, and how to solve them.

## Game

To be honest, I don't really have that perfectly nailed down right now. My goal is to just make _something_ that I can build off of. At first, I tried a simple DDR/FFR style rhythm game. This was a great exercise, but it was so messy by the time I was done with it (probably a good sign I learned a lot).

## Starting out

Amethyst is built on `specs`, an ECS (Entity Component System). If you've used something like Unity in the past, you might be partially familiar.

I thought it'd be smooth sailing, since I _had_ used Unity in the past, and ECS was a term I was comfortable with. For the most part it was - however, Unity's "ECS" has a few distinct differences.

In Unity, when you want to write a new script, you often use `MonoBehaviour`. This file would contain a definition of the component, as well as how that individual component should behave. 

In Amethyst, we see the "S" in "ECS" more often. Instead of having a component describe its own implementation, we use Systems. While I won't go into depth about the topic since the [Amethyst book covers these topics much better than I could](https://amethyst.rs/doc), let me give you a general gist of systems.

Whereas some engines like Unity define individual implementation, systems define how _all_ things of a certain type should behave. This makes it super easy to benefit from certain CPU performance improvements later on.

## Hurdles

Okay. When I said my first game attempt in Amethyst was an FFR clone, I lied. It was actually going to be a text-based visual novel. As is often the case with game development, I planned a huuuge scope while I was still learning. Turns out this is usually not a good idea! My first lesson is don't even _pretend_ like the first few games you make while learning an engine will last. It's important to be messy, make mistakes, learn, and move on.

So that brings us to our FFR style game. The idea was to create something _much_ simpler. The game would start with a song at a constant BPM, a file would describe what "arrows" would show, and input was limited to keyboard movements. While I never attained this goal either, I actually got kind of far! If you'd like to take a look, I have the [hideous messy source code right here](https://github.com/piedoom/r). So, I got the arrows spawning according to a timer, and moving down at a constant speed. Input is also captured, although it has no outward effect on the "game".

So, why did I move on, and what did I learn? My biggest issue was figuring out which code should be put in a `Component`, and which code should be put in a `System`. For instance, if I want to move an entity with a component, I also need the entity's transform. Should a function be supplied in the component that takes a reference to a transform, or should everything be handled in the system?

To illustrate the difference, imagine the following pseudo-code:

```rs
// Component 
struct MyComponent
    speed: 1

impl MyComponent
    fn move_up(self, transform) 
        // move up the transform here using the component speed
        transform.move_up(self.speed)

// System
struct MyComponentSystem

impl MyComponentSystem
    fn run(component, transform)
        component.move_up(transform)
```

Here, we define data *and* implementation in a component. After playing around with my code, I found similarities to ECS and other common patters like MVC. Compared to MVC, components are almost like models, in that they contain data (and possibly some helper functions), but do not implement their own functionality. That is left up to the system (which is similar ((sort of)) to a controller).

So if we were to rewrite the above pseudo-code, it might look like this:

```rs
// Component 
struct MyComponent
    speed: 1

// System
struct MyComponentSystem

impl MyComponentSystem
    fn run(component, transform)
        // imagine the transform component has a translate_up function
        transform.translate_up(component.speed)
```

It does the same thing, but implementing most of the logic in a system makes it easier to keep track my project's code.

## What's Next

Now that I had working knowledge of how to use specs, I thought it was a good idea to start fresh and new. I wanted to come back to the idea of basing a game around a narrative. Unfortunately, text-based graphics - while not difficult - are not relatively popular, which means I'd have less resources from which to learn. Instead, I thought a top-down JRPG-like game might be a bit more attainable. Okay, actually JRPGs are super hard, but the movement aspect is easy.

For JRPG/Pokemon style movement, a character needs to be able to move in at least the four cardinal directions, align itself to a grid, and not run through walls.  This is *much* simpler than attempting something like a platformer which gets hairy for someone with a BFA, especially when slopes are involved. I'm a horror fan, and I think there's a criminal lack of 2D horror - so I've decided to make an RPG-like horror game. Fun stuff!

## Movement

Okay, so lets remember the 3 goals from earlier:

1. Move in four directions
2. Align to a grid
3. Don't run through walls

I have a solid 2 / 3 so far, which I think is pretty good! My character runs around in an endless cornflower-blue void, but it's a start.

### 1. Move in four directions

To start, I needed a way to move the character. Sometimes, just modifying a transform in a system would suffice, but in my case, I knew I had more advanced movement in the future, so I opted for custom components. I created two, actually - `Movement`, and `Player`. This way, I can re-use my movement component with NPCs and other game entities without it being tied to game input. Instead, my `Player` component and system capture user input, and modify a `Movement` state. The movement system *then* moves everything based on the component's state. 

Again, as someone whose degree includes the word "Art", math is not my forte. I actually regret not paying attention in my classes. Ever hear people say "when will I ever use this?" in class? Well, you will use it. Probably a lot. At least if you are an aspiring game developer.

However, I watched a few videos on vectors and all that cool stuff, so I got a #solid grasp on some concepts. This helped a lot when figuring out how to keep the direction state in my `Movement` component.

Usually, I would do something like this to store direction:

```rs
struct Movement
    up: bool
    down: bool
    left: bool
    right: bool
```

While not totally awful (maybe a little awful), it is not ideal, especially when dealing with vector math later down the road. Instead, the nalgebra math crate provides me with a `Vector3`, and a `Unit` type.

From my light reading, I realized I could store the direction of my `Movement` in a unit vector.

```rs
struct Movment
    direction: Unit<Vector3>
```

This makes it super-duper simple to actually use this value later on in tandem with the entity transform.

### 2. Align to a grid

This one was much tougher for me! I had a few ideas on how to implement grid-based movement. Initially, I just `lerp`ed between two values via the `Movement` component. This is great for entities that only move one grid at a time. However, for entities that move multiple tiles in one direction, this method looked unstable and jittery. Controls also felt unresponsive sometimes.

Instead, I figured that I only really care if the entity is aligned to a grid *before* and *after* it stops moving. In-between, it can just y'know, go. I haven't implemented this yet, but it should allow for smoother movement and tighter-feeling controls. 

### 3. Don't run through walls

This is the hardest part, in my mind. It should be decently simple - since all items are aligned to a grid, all we would need to do is check if the tile next to the player is empty. However, this (possibly) gets into ray-casting which I'm unfamiliar with using Amethyst. But I hope to learn!

## End

All said, while I did struggle a bit with the initial complexity, Amethyst is an insanely fun engine to work with, even if all you're doing is making sprites move around. Specs is very powerful, and the "composition over inheritance" style of coding makes development go much quicker than I expected.

I hope to have more stuff like this in the future! Please consider [donating](https://amethyst.rs/donate/) to the Amethyst Foundation to support development.
