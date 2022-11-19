+++
title = "Creating an audio plugin with Rust in 2022: Part 1 — Getting started"
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

This series follows my own progress exploring the Rust Audio ecosystem in 2022. At the end, we should have a basic polyphonic synthesizer with a GUI. This series focuses at developers who are comfortable using Rust. Basic familiarity with digital audio tools like DAWs or plugin hosts is a plus.

### Will this show me how to make a VST2?

Unfortunately, VST2 (and thus the `vst` crate), are now considered "unlicensed". We will no longer rely on the `vst` crate, and instead opt for a generic interface that can build a VST3. Please note: due to the GPLv3 community license of the VST3 SDK, VST3 plugins built with it will require the same license.

### Tool belt

In the entire series, we'll use several crates. I've highlighted a few notable libraries below.

### 1. [egui](https://github.com/emilk/egui)

egui is an excellent immediate-mode GUI written in pure Rust. (In fact, all listed crates are pure Rust and require only the Rust tool chain). [^immediate-vs-retained][^excessive]

### 2. [nih-plug](https://github.com/RustAudio/vst-rs)

`nih-plug` helps us construct our audio plugin in a generic way, building to multiple plugin formats like VST3 and [CLAP](https://github.com/free-audio/clap).

### 3. [fundsp](https://github.com/SamiPerttu/fundsp)

`fundsp` provides a novel way to construct audio graphs using pure Rust. We will look at using fundsp to generate the samples for our synthesizer. You may find using `fundsp` is simpler (and more fun) than implementing your own math.

### 4. [baseview](https://github.com/RustAudio/baseview)

`baseview` manages windowing with our plugin UI. If you're familiar with winit, it is similar, but designed around the needs of audio plugins. We won't interact much with `baseview` [^baseview-egui] directly.

## Setting up

Initialize a new Rust project with `cargo new --lib synthy`. You may choose whichever name you like, but I will refer to `synthy` in code examples. Note the `--lib` flag, which may seem counter-intuitive. VST plugins are a dynamic library, even if they seem more like standalone applications. Specifically, we use the `cdylib` `crate-type` which creates `*.so` files on Linux, `*.dylib` files on macOS, and `*.dll` files on Windows [^linkage].

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
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug" }
rand = "0.8"
```

> Note the `edition = "2021"` line. Some imports in the following code will error without the new resolver, which is implied by this edition.

To start, we're going to build a white noise generator as a VST plugin. It won't react to MIDI yet, but it'll help us understand how we create and modify audio buffers. As we progress, we'll add more complex components.

Remove everything in your `lib.rs` file, and replace it with the following

{{ filename(name="src/lib.rs") }}
```rs
// ---------- //
// 0. Imports //
// ---------- //
use nih_plug::*;
use rand::Rng;
use std::{pin::Pin, sync::Arc};

// ----------------------------- //
// 1. Defining plugin and params //
// ----------------------------- //
struct Synthy {
    params: Pin<Arc<SynthyParams>>,
}

#[derive(Params)]
struct SynthyParams {
    #[id = "amplitude"]
    amplitude: FloatParam,
}

// ------------------------------ //
// 1. Param defaults and settings //
// ------------------------------ //
impl Default for Synthy {
    fn default() -> Self {
        Self {
            params: Arc::pin(SynthyParams {
                amplitude: FloatParam::new("amplitude", 0.1, Range::Linear { min: 0.0, max: 1.0 }),
            }),
        }
    }
}

// ------------------------ //
// 2. Plugin implementation //
// ------------------------ //
impl Plugin for Synthy {
    const NAME: &'static str = "synthy";
    const VENDOR: &'static str = "rust audio";
    const URL: &'static str = "https://vaporsoft.net";
    const EMAIL: &'static str = "myemail@example.com";
    const VERSION: &'static str = "0.0.1";
    const DEFAULT_NUM_INPUTS: u32 = 0;
    const DEFAULT_NUM_OUTPUTS: u32 = 2;
    const ACCEPTS_MIDI: bool = true;

    // ----------------------------- //
    // 3. Modifying the audio buffer //
    // ----------------------------- //
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        for buffer in buffer.as_raw() {
            // a. Filling the buffer with random numbers
            rand::thread_rng().fill(*buffer);
            // b. Adjusting amplitude
            for sample in buffer.iter_mut() {
                *sample *= self.params.amplitude.value;
            }
        }
        // c. Returning a status
        ProcessStatus::Normal
    }

    // --------------------------- //
    // 4. Getting the params field //
    // --------------------------- //
    fn params(&self) -> std::pin::Pin<&dyn Params> {
        self.params.as_ref()
    }
}

// ---------------------- //
// 5. Building the plugin //
// ---------------------- //
impl Vst3Plugin for Synthy {
    const VST3_CLASS_ID: [u8; 16] = *b"1234567891234567";
    const VST3_CATEGORIES: &'static str = "Instrument|Synth";
}

nih_export_vst3!(Synthy);
```

This is possibly the simplest synthesizer plugin you can make with Rust. Thankfully, it doesn't need _too_ much code to get started. Let's explain what's going on, section by section:

### 0. Imports

```rs
use nih_plug::*;
use rand::Rng;
use std::{pin::Pin, sync::Arc};
```

Here, we import several crate items to take advantage of `nih_plug`. We also import a trait from the `rand` crate to assist us in filling the buffer with random values. Lastly, we import some items from the standard library that allows us to use parameters across threads.

### 1. Defining the plugin and params

```rs
struct Synthy {
    params: Pin<Arc<SynthyParams>>,
}

#[derive(Params)]
struct SynthyParams {
    #[id = "amplitude"]
    amplitude: FloatParam,
}
```

First, we create a base `Synthy` struct that will represent our plugin. Next, we create a new struct to hold any adjustable parameters. In this case, we add one `amplitude` field. Make sure you remember to derive from `Params`.

### 2. Plugin implementation

```rs
impl Plugin for Synthy {
    const NAME: &'static str = "synthy";
    const VENDOR: &'static str = "rust audio";
    const URL: &'static str = "https://vaporsoft.net";
    const EMAIL: &'static str = "myemail@example.com";
    const VERSION: &'static str = "0.0.1";
    const DEFAULT_NUM_INPUTS: u32 = 0;
    const DEFAULT_NUM_OUTPUTS: u32 = 2;
    const ACCEPTS_MIDI: bool = true;
    // ...
```

Here, we implement the `Plugin` trait and various associated constants that provide information about our plugin to the plugin host. Because our synth does not process any incoming audio, and generates stereo, we set `DEFAULT_NUM_INPUTS` and `DEFAULT_NUM_OUTPUTS` to 0 and 2 respectively.

### 3. Modifying the audio buffer

```rs
fn process(
    &mut self,
    buffer: &mut Buffer,
    _context: &mut impl ProcessContext,
) -> ProcessStatus {
    for buffer in buffer.as_raw() {
        // a. Filling the buffer with random numbers
        rand::thread_rng().fill(*buffer);
        // b. Adjusting amplitude
        for sample in buffer.iter_mut() {
            *sample *= self.params.amplitude.value;
        }
    }
    // 3c. Returning a status
    ProcessStatus::Normal
}
```

This is the main part of our synthesizer, where we modify the output audio buffers. `Buffer` is a type that contains all channels, inputs and outputs. 

a. As we only have outputs, we loop through each channel's buffer and `fill` it with noise using the `rand`.
b. We get the parameter `amplitude` and multiply it by each individual sample to adjust the volume
c. The `Plugin::process` method must return a `ProcessStatus`. As there were no errors, we return `ProcessStatus::Normal`.

## Compiling and loading

> Warning: while the initial amplitude is set to be very low, always take caution when loading a new sound-generating application. There is always the potential that your plugin will be **louder than you expect** or **uncontrollable**. Please turn your volume down before loading any plugin to reduce the risk of hurting your ears.

To compile, use the following command:

```
cargo build --release
```

We will always tend to build in release as it offers superior performance, especially when dealing with time-sensitive operations like real-time audio. When finished building, you should see a `synthy.dll` file in your `target/release/` directory.

After compilation, we need to bundle our plugin [^tasks]. The specifics of this bundle depend on our output format and the target system. To skip all the system specifics, we can download another Rust tool to automatically bundle our plugins. In a terminal, run the following:

```sh
cargo install --git https://github.com/piedoom/plugit
```

This installs the [`plugit`](https://github.com/piedoom/plugit) executable with cargo from GitHub. We can use it to bundle our `synthy.dll` into a VST3 that our host will recognize. In the same directory as your project's `Cargo.toml`, run `plugit` with no parameters. You should receive a success message, along with a printout of the directory of your newly bundled plugin directory [^directory-structure].

You can take a look at [my older article](/creating-an-audio-plugin-with-rust-vst#testing-our-bare-bones-plugin) if you need help setting up a VST host on Windows.

## Listening to our synth

After loading the plugin, it should begin playing white noise. If you haven't made the connection yet, random data as an audio stream sounds like white noise! That's what we did with `rand`'s `fill` method.

### Audio sample

{{ audio(src="01-sample") }}

### Analyzing

Let's take a closer look at what we're listening to by using some visualization. In Bitwig, the oscilloscope allows us to visualize the produced audio wave. The following is an oscilloscope applied to our plugin with an amplitude of 1. 

![An oscilloscope reading of the white noise shows a wave pattern only visible with positive values.](01-visualizer.png)

We can see that it's not *exactly* what we'd like to see - but it's not surprising. Our signal is stuck on the top (green) with no values on the bottom (red). Our `rand` method generates values from `0.0` to `1.0`. With audio signals, we generally want to produce values within `-1.0` to `1.0`. While we could solve this, it won't matter much as we'll be discarding our white noise generator for something more interesting in later chapters.

If you're a pro at DSP, this might be enough for you to get started. But I hope you'll stick around, regardless of your expertise, to explore more of the Rust ecosystem.

## Next: Generating a signal with `fundsp`

We'll be taking a look at using the `fundsp` crate to easily generate audio graphs and process audio samples.

---

## Footnotes

[^immediate-vs-retained]: Well, some would not be inclined to agree. What even is immediate-mode? Let's steal egui's excellent documentation as a quick explainer: 
> egui is an immediate mode GUI library, as opposed to a retained mode GUI library. The difference between retained mode and immediate mode is best illustrated with the example of a button: In a retained GUI you create a button, add it to some UI and install some on-click handler (callback). The button is retained in the UI, and to change the text on it you need to store some sort of reference to it. By contrast, in immediate mode you show the button and interact with it immediately, and you do so every frame (e.g. 60 times per second). This means there is no need for any on-click handler, nor to store any reference to it. In egui this looks like this: 
> 
> ```rs
> if ui.button("Save file").clicked() { 
>     save(file); 
> }
> ```

[^excessive]: Yes! I have figured out that markdown has annotations. This is bad news for everyone who thought excessive parenthesis were bad enough.

[^baseview-egui]: We're actually going to be using [egui-baseview](https://github.com/BillyDM/egui-baseview) and not just baseview to help us use egui specifically. In any case, the crate does the heavy lifting, and we just use egui as usual after some initialization.

[^linkage]: To see more about `cdylib` and other `crate-type`s, read [the "Linkage" page of Rust's reference docs](https://doc.rust-lang.org/reference/linkage.html).

[^tasks]: There is no easy way to execute post-build steps with cargo yet. While the [`xtask`](https://github.com/matklad/cargo-xtask) pattern exists, it's not as easy to get started with.

[^directory-structure]: This directory _is_ the VST3. Do not try to remove the library file from within the directory structure. If you move the plugin to a different location, move the entire directory. To view more about how VST3 plugins are structured, [visit Steinberg's site](https://developer.steinberg.help/pages/viewpage.action?pageId=9798275).