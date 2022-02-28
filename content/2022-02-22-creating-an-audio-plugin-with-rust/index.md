+++
title = "Creating a Rust VST plugin in 2022 part 1: Getting started"
author = "doomy" 
date = 2022-02-21
description = "Creating an audio plugin with the Rust programming language"

[taxonomies] 
tags = ["rust", "audio"]

[extra]
next="2022-02-23-creating-an-audio-plugin-with-rust-2/index.md"
hidden=true
+++

The Rust audio ecosystem has changed much 4 years since the last time [I've written about this subject](/creating-an-audio-plugin-with-rust-vst). I recommend checking out the official [Rust Audio Discord](https://discord.gg/PQXjcWdUYM) chat for more info or help as you progress.

## What will I learn here?

This series follows my own progress exploring the Rust Audio ecosystem in 2022. At the end, we should have a basic polyphonic VST 2 synthesizer with a GUI [^vst]. This series focuses at developers who are comfortable using Rust. Basic familiarity with digital audio tools like DAWs or plugin hosts is a plus.

## Tool belt

Creating an audio plugin is difficult from scratch, so we'll need to pull in a few crates.

### 1. [egui](https://github.com/emilk/egui)

egui is an excellent immediate-mode GUI written in pure Rust. (In fact, all listed crates are pure Rust and require only the Rust tool chain). [^immediate-vs-retained][^excessive]

### 2. [vst](https://github.com/RustAudio/vst-rs)

Our good friend, `vst-rs` is still around. Not much has changed, which is unsurprising as VST 2 spec development is no longer active. This crate helps us construct VST plugins by surfacing several traits and utilities.

### 3. [baseview](https://github.com/RustAudio/baseview)

`baseview` manages windowing with our plugin UI. If you're familiar with winit, it is similar, but designed around the needs of audio plugins. We won't interact much with `baseview` [^baseview-egui] directly.

### 4. [fundsp](https://github.com/SamiPerttu/fundsp)

`fundsp` provides a novel way to construct audio graphs using pure Rust. We will look at using fundsp to generate the samples for our synthesizer. You may find using `fundsp` is simpler (and more fun) than implementing your own math.

## Setting up

Initialize a new Rust project with `cargo new --lib synthy`. You may choose whichever name you like, but I will refer to `synthy` in code examples. Note the `--lib` flag, which may seem counter-intuitive to the average rustacean. VST plugins build as a `dylib`, even if they seem more like applications.

Let's add a few dependencies to our `Cargo.toml` to get started.

{{ filename(name="Cargo.toml") }}
```toml
[package]
name = "synthy"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
# add some dependencies here
vst = "0.3"
rand = "0.8"
```

To start, we're going to build a white noise generator as a VST plugin. It won't react to MIDI yet, but it'll help us understand how we create and modify audio buffers. As we progress, we'll add more complex components.

Remove everything in your `lib.rs` file, and replace it with the following

{{ filename(name="src/lib.rs") }}
```rs
// ---------- //
// 0. Imports //
// ---------- //
use rand::Rng;
use std::borrow::BorrowMut;
use vst::prelude::*;

// ----------------------------- //
// 1. Define the plugin struct //
// ----------------------------- //
struct Synthy;

impl Plugin for Synthy {
    fn new(_host: HostCallback) -> Self {
        Synthy
    }

    // -------------- //
    // 2. Plugin info //
    // -------------- //
    fn get_info(&self) -> Info {
        Info {
            name: "synthy".into(),
            vendor: "rusty".into(),
            unique_id: 128956,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: 0,
            ..Info::default()
        }
    }

    // -------------------------- //
    // 3. Modify the audio buffer //
    // -------------------------- //
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, mut outputs) = buffer.split();
        for output in outputs.borrow_mut() {
            rand::thread_rng().fill(output);
        }
    }
}

// ------------------- //
// 4. Build the plugin //
// ------------------- //
vst::plugin_main!(Synthy);
```

This is possibly the simplest synthesizer plugin you can make with Rust. Thankfully, it doesn't need _too_ much code to get started. Let's explain what's going on, section by section:

### 0. Imports

Here, we import several crate items to take advantage of `vst-rs`. We also import a trait from the `rand` crate to assist us in filling the buffer with random values.

### 1. Defining the plugin struct

Because there are no parameters or state to our synth yet, we can create a struct with zero fields.

### 2. Plugin info

`get_info` is a required implementation on the `Plugin` trait. We provide info that is later read by our DAW to show basic details about our plugin. There are a few items of note.

1. `unique_id` - some random number that should be unique. Technically this is something you'd register, but... is anyone doing that?
2. `category` - this is where we'll specify that our plugin is a synth, as opposed to an audio effect.
3. `inputs` and `outputs` - these are the amount of audio inputs and outputs our synth supports. Because we are generating audio, we give our synth 0 inputs and 2 outputs for stereo.
4. `parameters` - indicates the number of modifiable parameters (dials, switches, and everything else interactive) available. Right now, we have 0.
5. `f64_precision` - we won't use this one today, but it's worth mentioning in case you stumble across it in your own experiments. [^f64-precision]
6. everything else - not important to us right now!

### 3. Modifying the audio buffer

This is the main part of our synthesizer, where we modify the output audio buffers. `buffer.split()` returns a tuple of mutable input (which we discard) and output buffers. We then use the `fill` method provided by the `rand` crate to fill our entire buffer with random values.

## Compiling and loading

> Warning: this is **loud** and **uncontrollable**. Please turn your volume down before loading this VST into your DAW. It will produce sound immediately and incessantly.

To compile, use the following command:

```
cargo build --release
```

We will always tend to build in release as it offers superior performance, especially when dealing with time-sensitive operations like filling audio buffers.

When finished building, you should see a `synthy.dll` file in your `target/release/` directory. If you're on Windows, can now load this into the VST host of your choice and try it out. If you're on Mac, there's an extra [script to run](https://github.com/RustAudio/vst-rs/blob/b5f342c67535071f65824d1b4a056d378ca03548/osx_vst_bundler.sh). If you're on Linux, it may be a different story too[^linux].

You can take a look at [my older article](/creating-an-audio-plugin-with-rust-vst#testing-our-bare-bones-plugin) if you need help setting up a VST host.

## Listening to our synth

After loading the plugin, it should begin playing white noise. If you haven't made the connection yet, random data as an audio stream sounds like white noise! That's what we did with `rand`'s `fill` method.

### Audio sample

(amplitude adjusted to be less loud)

{{ audio(src="01-sample") }}

### Analyzing

Let's take a closer look at what we're listening to by using some visualization. In Bitwig, the `oscilloscope` effect allows us to visualize the produced audio wave.

![An oscilloscope reading of the white noise shows a wave pattern only visible with positive values.](01-visualizer.png)

We can see that it's not *exactly* what we'd like to see - but it's not surprising. Our signal is stuck on the top (green) with no values on the bottom (red). Our `rand` method generates values from `0.0` to `1.0`. With audio signals, we generally want to produce values within `-1.0` to `1.0`. While we could solve this, it won't matter much as we'll be discarding our white noise generator for something more interesting in later chapters.

If you're a pro at DSP, this might be enough for you to get started. But I hope you'll stick around, regardless of your expertise, to explore more of the Rust ecosystem.

## Next: Generating a signal with `fundsp`

We'll be taking a look at using the `fundsp` crate to easily generate audio graphs and process audio samples.

---

## Footnotes

[^vst]: Steinberg has been trying to [kill VST 2 forever](https://forums.steinberg.net/t/vst-2-discontinued/761383), unsuccessfully. Rust Audio also provides an LV2 format among others like VST 3, which I will (possibly) be exploring. It remains that VST/VST 2 is the most common term when referring to audio plugins, especially while searching.

[^immediate-vs-retained]: Well, some would not be inclined to agree. What even is immediate-mode? Let's steal egui's excellent documentation as a quick explainer: 
> egui is an immediate mode GUI library, as opposed to a retained mode GUI library. The difference between retained mode and immediate mode is best illustrated with the example of a button: In a retained GUI you create a button, add it to some UI and install some on-click handler (callback). The button is retained in the UI, and to change the text on it you need to store some sort of reference to it. By contrast, in immediate mode you show the button and interact with it immediately, and you do so every frame (e.g. 60 times per second). This means there is no need for any on-click handler, nor to store any reference to it. In egui this looks like this: 
> 
> ```rs
> if ui.button("Save file").clicked() { 
>     save(file); 
> }
> ```

[^excessive]: Yes! I have figured out that markdown has annotations. This is bad news for everyone who thought excessive parenthesis were bad enough.

[^f64-precision]: The important thing to note here is that enabling `f64_precision` and implementing the `process_f64` function of the `Plugin` trait does not necessarily work, as it's up to the DAW to support those. Be aware you might not hear anything if you try and use `f64` precision. 

[^baseview-egui]: We're actually going to be using [egui-baseview](https://github.com/BillyDM/egui-baseview) and not just baseview to help us use egui specifically. In any case, the crate does the heavy lifting, and we just use egui as usual after some initialization.

[^linux]: I don't know how to help with Linux yet. Maybe I'll cover Rust Audio's LV2 crate in the future.
