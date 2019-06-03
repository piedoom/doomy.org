+++
title = "My Week In Amethyst 2 - Using Tiled"
author = "doomy"
template = "page.html"
date = 2019-06-03T14:27:23.927Z
description = " "

[taxonomies]
categories = []

[extra]
tags = []
+++
[Last week](/mwia-1) I took a deep dive into Amethyst, and learned as much as I could in a week. I figured out systems, got a player moving with input, and created a component for grid-aligned movement. As a quick update, let's talk about the movement system:

## Review

In the previous issue, I created both a `GridMovement` component, as well as a `Movement` component. The latter was free-er and not aligned to a grid like the former. In the future if I were to redesign this, I would create one single movement component, as well as a `GridAligned` component. Because grid calculations are significantly different than simple free-moving calculations, this separate component would keep the necessary data to feed into the regular movement component. Alternatively, an "align to grid" boolean option on a `Movement` component could prove to be a simpler solution.

## This week

This week was all about maps! Specifically, [Tiled](https://www.mapeditor.org/) - a 2D tilemap editor with a wide array of supported game engines. Tiled encodes its data in XML, which is convenient since it means a custom parser doesn't need to be implemented. 

## Tiled

Rust actually has a crate for Tiled, called [`rs-tiled`](https://github.com/mattyhall/rs-tiled). It's somewhat barebones, but provides everything I needed to get started. (Well, almost everything, but more on that later.) There's also a great beginner example of how to use `rs-tiled` within Amethyst, [which you can view here](https://github.com/Temeez/Tiled-Amethyst-Example).  

There's a few limitations with the above:

1. The tilemap is not loaded as an asset, as a spritesheet or model would be. This means it's quick and easy to load via our world, but can pose some issues later on if we want to read our map data in other systems. 
2. The example only loads a single layer, which is too basic for a JRPG-style game. We will eventually want not only a background layer, but a scenery layer.

I sought out to create an asset type from `rs-tiled` according to the guide [available here](https://book.amethyst.rs/master/assets/how_to_define_custom_assets.html). It's relatively straightforward, especially since `rs-tiled` provides a method to read from bytes. 

Not exactly Amethyst related, but I dislike when Rust projects try to put too much in a single file.While I'm grateful that `rs-tiled` exists, all of the code was in a single `lib.rs` file which made understanding it rather difficult. Additionally, the code was not Rust 2018 idiomatic, mostly because it used the `try!` macro instead of the preferred `?` for `Result`s.

[To remedy this, I created my own fork of `rs-tiled`](https://github.com/piedoom/rs-tiled), which updates the code so newer `rustc` won't complain. I also added an `amethyst` feature gate that enables `Map` to be an asset. This worked pretty great, actually! I'm not sure if including the asset type within the crate is idiomatic, but once that is reviewed I might muster up the courage to PR into master - though depending on the original author's intentions they may not like my very subjective splitting up of source files.

## Building the Map

This was the _really_ tough part for me. Now that I had a `Map` asset, how do I use it? My first thought was "Oh, well, a handle is just like a reference, right? All I have to do is dereference that handle somehow to get the underlying asset! Easy."

Okay, so let's try it. In Amethyst, we get the underlying asset of a `Handle` like this:

```rs
asset_storage.get(&self.my_handle.clone()).unwrap();
```

Annnnd... it panics! But why?

`Handle`s are returned _immediately_ from a `load` method, as to not block the game thread. This means that the `Handle` could point to data that is not yet fully loaded. When we try to get this incomplete data, the program panics.

While I struggled for a while to understand this concept, it's relatively easy in retrospect. The solution to make sure that everything is loaded before continuing. We can do this with `ProgressCounter`s and states. 

From a high level, our program flow looks like this:

```
Game Start ->
Begin Loading State ->
    Load Map
    Every Update, check if Map is loaded
    If loaded, push new state with Map handle
Push Main Game State ->
    Main Game State has Map handle field
    This handle is guaranteed to be fully loaded
```

Actually not too bad! This has the benefit of accidentally creating a nice loading state for us to use in our game. 

If you want to see the full details of how I actually did this, [take a look at my code here](https://github.com/piedoom/j/blob/master/src/states/load.rs#L20). Note that I actually use a couple loading states, since my map file contains references to textures that need to be loaded.

Awesome. Our map is loaded and ready to use. All that's left is to build it. To do this, I used Temeez's aforementioned [Amethyst-Tiled](https://github.com/Temeez/Tiled-Amethyst-Example) example as a base. Every tile in this case is a separate entity. While that might seem like a lot, entities in Amethyst are extremely cheap, so there's not much to worry about performance wise.

### System Data

Systems, as we know, are designed to manage the behavior of a lot of _things_ every game update. While we need functionality similar to this, our map building function only needs to run _once_, which makes running the code in the `on_start` of a game state more fitting. We can get the best of both worlds by using `world.exec` in our game state with system data. 

I'm by no means an expert `borrowck` charmer, so I may have made newbie errors, but using `SystemData` instead of a bunch of method calls on `world` helped me avoid a lot of borrow check issues. I won't get into the weeds, [but here's the code again if you're interested](https://github.com/piedoom/j/blob/master/src/states/main.rs#L35). Take note that building the map would probably be more suitable to be done in yet another loading state.

## Wrapping Up

Great! We have a way to read our tiled file, a state to load all its resources, and then a main state to actually build the tilemap and run the game. And just like that, my blobby red player sprite is no longer drifting in an endless blue void.

![](/static/uploads/mwia2.png)

## Next Up

I'll be honest, this learning experience took a lot out of me. I might take a small break to a small non-tilemap-based program for a bit, but after that my next goal is collision. There's already an nphysics integration for Amethyst, and I'm excited to try it out! Until next time.
