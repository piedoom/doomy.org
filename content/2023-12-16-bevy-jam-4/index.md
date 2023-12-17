+++
title = "I submitted a game to Bevy Jam 4 and it was pretty cool"
author = "doomy" 
description = "Thoughts on working with Bevy in version 0.12 to make a 3D marble game"

[taxonomies] 
tags = ["rust", "gamedev", "bevy"]
+++

I submitted an entry to the 4th #bevy jam this December, with the theme of "
That's a LOT of Entities!". As a big fan of the [*Marble Blast
Ultra*](https://en.wikipedia.org/wiki/Marble_Blast_Ultra) game on the 360
arcade, I though I might try a stab at something similar. 

## Bevy Blast Ultra

With subtle acknowledgement of its inspiration, [Bevy Blast Ultra](https://github.com/piedoom/bevy_blast_ultra/releases/) brings several
minutes of non-stop MVP marble action to any ~~browser~~ Windows, Mac, or Linux
machine.
 

## How's Bevy 0.12?

There's a couple Epic Rust Moments and issues I ran into while creating this
game. Bevy has grown tremendously since I last wrote about it in version 0.6,
and this is an effort to document what I found of note.

### Bevy editor, pls

While Bevy is a
capable engine, its lack of an editor means procedural generation is much more
achievable in comparison to handcrafted levels. A game like MBU, however,
requires intentionally designed levels (or a wizardly knowledge of procedural
generation). 

![Marble Blast Ultra screenshot](MarbleBlastUltra_screenshot.png)

A few 3rd party [editors for
Bevy](https://bevy-cheatbook.github.io/setup/bevy-tools.html) exist at the
moment, but are understandably limited in comparison to established engines like
Unreal.

Fortunately, Bevy *is* able to import GLTFs, and a workflow for [exporting to
Bevy is already
available](https://github.com/kaosat-dev/Blender_bevy_components_workflow).
Using a single `.blend` file, I could edit all levels as different scenes, and
create reusable prefabs. Most importantly, I could create any kind of level
geometry that Blender could export.

### Assets are still kind of tricky, but not as much

Assets used to be among the more tedious game logic to implement. With the
advent of helpful crates like
[`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader), this isn't
much of an issue. 

Before I decided on going the Blender route, I considered writing a bunch of
`.ron` scene files by hand. I would have liked to specify a bunch of prefab
`.ron` scenes, and use them within level `.ron` scenes. Apparently, that still
isn't a possibility just yet, but I hear an overhaul of the way scenes are
handled in Bevy is under way. I imagine this might rely on some of the new asset
v2 preprocessing features released in `0.12`.

Annoyingly, it's still impossible to load folders with WASM (which isn't a
Bevy issue, but still a bummer when you have to hard-code filenames in your
project for web support). 

### Z will always be up to me

ok seriously though. "Up" is `(0.0,0.0,1.0)`, and I'm Tired of People trying to
Tell Me Otherwise! It is not `(0.0,1.0,0.0)`. I will die on this very tall
Z-aligned hill.

This brings me to **The worst moment of the Bevy Game Jam 2023**: figuring out
skyboxes.

I spent 6 very misplaced hours right before the due date attempting to figure
out how to convince my skybox image that Z is up.

This was, of course, completely unnecessary compared to fixing core issues with
the camera logic, but I persisted and brute-forced my way to figuring it out. 

### The wretched skybox journey

Bevy provides a few files in the example assets for showing a skybox image, but
it has some minor issues - mostly the fact that it is a little bit rotated.

![Game screenshot showing a skybox that is rotated 90 degrees
counter-clockwise](zup.png)

But that shouldn't be an issue, right? With the right editing, I could certainly
rotate it back in the right direction. But the files Bevy provides are `ktx2`
images. Most tools to convert to and from this format are CLI based (or need
compilation) which I wanted to avoid. NVIDIA *does* provide a GUI application
for viewing and exporting many of these filetypes, but it
claimed my precious `ktx2` files were corrupt. 

![Settings view of NVIDIA Texture Tools Exporter](texture-tools.png)

> This application has many, many settings, and I know what approximately 0 of
> them actually do.

Thankfully, the Bevy repo also provide a `.png` skybox example, I just needed to
map portions on the image in the correct sequence and rotation to match, which I
figured out slowly and arduously by changing the image until everything properly
tiled together.

![A cubemap from Bevy's examples](Ryfjallet_cubemap.png)

I'm sure this is a solved problem, and there's a document
somewhere specifying where to put which bit of image, but I instead spent hours
brute forcing this. But hey, it eventually worked!

> I skipped over a lot with this story. It also involved forking a cubemap
> generator javascript project so I could up the photo resolution, and lots of
> aggravated scribbling in Microsoft paint

#### I do not know how to compile a C project

One alternative route was to use Patrick Walton's [automatic
tool](https://github.com/pcwalton/gltf-ibl-sampler-egui), but this requires a
beginner level of C knowledge and some patience. After scouring the internet
for the right header files to download, and reading source code to figure out
where to place them, I gave up. But it looks like a great application for this
exact purpose, if you're able to get it running.

### The world's most difficult to remember acronym

Since I last used Bevy, its added another physics engine to the mainstream -
[`bevy_xpbd`](https://github.com/Jondolf/bevy_xpbd). The `xpbd` part stands for
*Extended Position Based Dynamics*. Without a bookmark, prepare to enter every
permutation of `xpbd`'s letters in your search engine.

XPBD to my understanding is a newer (but less battle-tested) way to calculate
physics. The YouTube channel [`Two Minute
Papers`](https://www.youtube.com/watch?v=F0QwAhUnpr4) does a pretty good job
summarizing the differences between XPBD and more traditional methods. One
interesting feature of XPBD is that it can simulate situations where more
traditional methods would fail, such as marbles on a track, which I incorporated
into my game.

![Marble on rails in game](rails.png)

I've used Rapier almost exclusively in the past, and while it's matured nicely,
it's still a library intended for general use. That means it needs the glue of
`bevy_rapier` to translate into our game world, and the Rapier and Bevy world
don't always play together without issues. `bevy_xpbd` on the other hand, is
designed specifically for use in Bevy, which (in my opinion) makes its resulting
API far friendlier and intuitive to use. I didn't encounter any bugs or strange
behavior and will use it as my physics engine of choice moving
forwards.

I only wish it had some built in forces (outside of gravity) like wind or
magnetism - but those are easy enough to implement yourself.

### Graphics are too good

As I alluded to earlier in this post, Bevy Blast Ultra is not playable
in-browser, because it runs like rotten garbage, clocking in a spectacular
~0.5FPS despite my desperate fumbling with `wasm-opt`. After playing a few other
submissions, I suspect the issue is the shadow graphics, which
were recently improved and look great, but might cost too much for a browser
game. But hey, it looks really nice!

<div style="padding:56.25% 0 0 0;position:relative;"><iframe src="https://player.vimeo.com/video/893312876?badge=0&amp;autopause=0&amp;player_id=0&amp;app_id=58479" frameborder="0" allow="autoplay; fullscreen; picture-in-picture" style="position:absolute;top:0;left:0;width:100%;height:100%;" title="Bevy Blast Ultra Gameplay"></iframe></div><script src="https://player.vimeo.com/api/player.js"></script>

## The best Bevy

Bevy is (objectively) a good game engine because Rust is (objectively) a good
programming language. The best parts of Bevy are when it leverages the best
parts of Rust, most notably in the incredibly satisfying generic `Query` API. I
find the more trying tasks within Bevy consist of working with dynamic data (like asset
loading). Still, I've found Bevy to be among the more enjoyable projects to hack
on in Rust, contrast to my web escapades in
[Tauri](https://github.com/piedoom/cedr) which left me feeling bitter and empty.
