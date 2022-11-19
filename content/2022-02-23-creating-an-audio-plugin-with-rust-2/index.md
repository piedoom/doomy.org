+++
title = "Rust VST part 2: Generating a signal with fundsp"
author = "doomy" 
date = 2022-02-22
description = "Creating an audio plugin with the Rust programming language: Part 2"

[taxonomies] 
tags = ["rust", "audio"]

[extra]
prev="2022-02-22-creating-an-audio-plugin-with-rust/index.md"
next="2022-02-23-creating-an-audio-plugin-with-rust-3/index.md"
hidden=true
+++

### Project files

We'll be building off of code built in the previous chapter. I recommend you take a look, or [download the project files here](/projects/synthy/synthy-01.zip).

## What is fundsp?

We briefly covered what fundsp **is** last time. This time, we'll be using it to generate a basic signal for our Rust synthesizer [^fundsp]. Let's take a look at the kind of syntax we'll expect when using `fundsp`:

```rs
// Taken from: https://github.com/SamiPerttu/fundsp/blob/3d2696a1782c80799e23f027dbaaf22af47bda53/examples/beep.rs
let synth = lfo(|t| {
    let pitch = 110.0;
    let duty = lerp11(0.01, 0.99, sin_hz(0.05, t));
    (pitch, duty)
}) >> pulse();
let mix = synth 
    >> declick() 
    >> dcblock() 
    >> split::<U2>() 
    >> reverb_stereo(0.2, 5.0) 
    >> limiter_stereo((1.0, 10.0));
```

> It's important to note that `>>` is not a bit-shifting, but *piping* audio (also known as chaining). `fundsp` provides custom operators that allow us to succinctly describe an audio graph in pure Rust code [^rust].

So, what does this sound like?

{{ audio(src="sample") }}

There's a few moving parts to this synth, but it is relatively easy to understand. `fundsp` does a lot of heavy lifting for us. The first part, `lfo`, describes some inputs for the `pulse` function (`pitch` and `duty`). In the `mix` block, the pulse wave — modified by those inputs — is then processed further, split into stereo channels, and given a stereo reverb effect. 

With this simple syntax, we can achieve some interesting sounds! So, how can we take advantage of `fundsp` in our audio plugins?

## Using `fundsp` to process an audio buffer

Let's revisit our white noise generator, and replace the `rand` sample generation with `fundsp`. First, let's add `fundsp` to our `Cargo.toml` and remove the `rand` crate.

{{ filename(name="Cargo.toml") }}
```toml
[dependencies]
# ... remove rand = "*"
fundsp = "0.3.1" # need to be this version at least 
```

We use the `noise()` function of `fundsp` to generate some _bipolar_ white noise to get used to `fundsp`. After that, we'll generate some more interesting tones.

{{ filename(name="src/lib.rs") }}
```rust
// ---------------- //
// 0. Hacker import //
// ---------------- //
use fundsp::hacker::*;
use nih_plug::*;
use std::{pin::Pin, sync::Arc};

struct Synthy {
    // ------------- //
    // 1. New fields //
    // ------------- //
    audio: Box<dyn AudioUnit64 + Send + Sync>,
    sample_rate: f32,
    params: Pin<Arc<SynthyParams>>,
}

#[derive(Params)]
pub struct SynthyParams {
    #[id = "amplitude"]
    pub amplitude: FloatParam,
}

impl Default for SynthyParams {
    fn default() -> Self {
        Self {
            amplitude: FloatParam::new("amplitude", 0.1, Range::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl Default for Synthy {
    fn default() -> Self {
        // -------------------------- //
        // 2. Creating an audio graph //
        // -------------------------- //
        let params = Arc::pin(SynthyParams::default());

        let amplitude = || tag(0, params.amplitude.value as f64);
        let audio_graph = noise() * amplitude() >> split::<U2>();

        Self {
            audio: Box::new(audio_graph) as Box<dyn AudioUnit64 + Send + Sync>,
            sample_rate: Default::default(),
            params,
        }
    }
}

impl Plugin for Synthy {
    const NAME: &'static str = "synthy";
    const VENDOR: &'static str = "rust audio";
    const URL: &'static str = "https://vaporsoft.net";
    const EMAIL: &'static str = "myemail@example.com";
    const VERSION: &'static str = "0.0.1";
    const DEFAULT_NUM_INPUTS: u32 = 0;
    const DEFAULT_NUM_OUTPUTS: u32 = 2;
    const ACCEPTS_MIDI: bool = true;

    fn params(&self) -> Pin<&dyn Params> {
        self.params.as_ref()
    }

    // --------------------------- //
    // 3. Processing both channels //
    // --------------------------- //
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        if buffer.as_raw().len() != 2 {
            return ProcessStatus::Error("unexpected number of buffers");
        }
        let buffer_raw = buffer.as_raw();
        let (left_buffer, right_buffer) = buffer_raw.split_at_mut(1);
        for (left_chunk, right_chunk) in left_buffer[0]
            .chunks_mut(MAX_BUFFER_SIZE)
            .zip(right_buffer[0].chunks_mut(MAX_BUFFER_SIZE))
        {
            let mut left_tmp = [0f64; MAX_BUFFER_SIZE];
            let mut right_tmp = [0f64; MAX_BUFFER_SIZE];

            self.audio.set(0, self.params.amplitude.value as f64);

            self.audio
                .process(MAX_BUFFER_SIZE, &[], &mut [&mut left_tmp, &mut right_tmp]);

            for (sample, output) in left_chunk.iter_mut().zip(left_tmp.iter()) {
                *sample = *output as f32;
            }
            for (chunk, output) in right_chunk.iter_mut().zip(right_tmp.iter()) {
                *chunk = *output as f32;
            }
        }

        ProcessStatus::Normal
    }

    // ------------------ //
    // 4. Set sample rate //
    // ------------------ //
    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        buffer_config: &BufferConfig,
        _context: &mut impl ProcessContext,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        true
    }
}

impl Vst3Plugin for Synthy {
    const VST3_CLASS_ID: [u8; 16] = *b"1234567891234567";
    const VST3_CATEGORIES: &'static str = "Instrument|Synth";
}

nih_export_vst3!(Synthy);
```

Now, let's explain what is going on here.

### 0. Hacker import

`fundsp` specifies not one, but *two* preludes — `fundsp::prelude::*`, and `fundsp::hacker::*`. The hacker environment uses 64-bit precision internally, which is what we want. Because the precision of our VST is `f32`, we will need to cast values when processing later.

### 1. New fields

a. `audio_graph`
    Our audio "graph" is hardly a graph, it's one `noise()` node controlled by amplitude, split into two channels (left and right). As the graph increases in complexity, its concrete type will become difficult or impossible to manually provide. We instead store the graph as a boxed `dyn AudioUnit64` for access. We further restrict the type to require `Send` and `Sync`, which is a bound defined by the `Plugin` trait.
b. `sample_rate`
    We store the plugin's sample rate, which is useful for when we calculate the time per sample later on.

### 2. New audio graph description

```rs
let params = Arc::pin(SynthyParams::default());

let amplitude = || tag(0, params.amplitude.value as f64);
let audio_graph = noise() * amplitude() >> split::<U2>();

Self {
    audio: Box::new(audio_graph) as Box<dyn AudioUnit64 + Send + Sync>,
    sample_rate: Default::default(),
    params,
}
```

In `fundsp`, `tag(..)`s are used to control parameters of the audio graph.

We use the `new` method to construct a new audio graph. In this case, it is one `noise()` node, split into a left and right track using the `split::<U2>()` function of `fundsp`. (Without the `split`, we would only hear audio on the left side.)

### 3. Processing both channels
 
The trickiest part is getting our arbitrarily-sized output buffer of `f32` precision to work with `fundsp`'s `process` method. `fundsp` provides the `MAX_BUFFER_SIZE` constant which defines the number of samples the given audio unit can handle. At the time of writing, that number is 64. If we attempt to give the `process` method more than `MAX_BUFFER_SIZE` samples, the plugin will panic. To resolve this, we `chunk` our buffer into a `MAX_BUFFER_SIZE` length using a temporary left and right buffer. We pass those buffers as "out" parameters to our `process` method, and then assign the values of that buffer to our actual output audio buffer. 

Whew, that was a lot... tl;dr, We split `&mut [f32]` into chunks of length 64, so it can be processed by `fundsp`[^style].

## What does it sound like?

White noise! That's it. But it's done properly this time, with values between `-1.` and `1.`

{{ audio(src="sample-2") }}

![Oscilloscope plugin showing our now bipolar white noise](visualizer.png)

## Creating something tonal

Now let's create a signal that actually _sounds_ like something. How about an "A" key (440Hz)? With `fundsp`, it is trivial: modify the `audio_graph` defined in your `new` block.

```rs
// ...
let audio_graph = sine_hz(440.0) >> split::<U2>();
// ...
```

{{ audio(src="sample-sine") }}

Let's take it a step further, and create an FM oscillator as described in `fundsp`'s README.

```rs
// ...
let pitch = 440.;
let modulation = 1.;

// #[allow(clippy::precedence)] // if you use clippy
let audio_graph = sine_hz(pitch) * pitch * modulation + pitch >> sine() >> split::<U2>();
// ...
```

{{ audio(src="sample-fm") }}

## Adding parameters

Now we have a simple FM synthesizer to play around with... sort of. Our synth doesn't turn off, let alone play different notes with different timbres. Let's add a couple parameters, so we can start to adjust the synth in real time.

> Because our VST doesn't have a UI yet, we need to ensure our plugin host provides some way to tweak parameters[^compatibility].

We're going to add two parameters, one for `pitch`, and another for `modulation`. We need to specify this in our info struct, as well as other parts of our application. Before this, we're going to add a couple dependencies to our `Cargo.toml`. We'll talk about these dependencies more later.

{{ filename(name="Cargo.toml") }}
```toml
[dependencies]
# ...
num-derive = "0.3"
num-traits = "0.2"
# ...
```

Next, create a new file in the `src` directory called `params.rs`. It should look like this:

{{ filename(name="src/params.rs") }}
```rs
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt::Display;
use vst::prelude::*;

// ----------------------------------- //
// 1. Creating the `Parameters` struct //
// ----------------------------------- //
pub struct Parameters {
    pub freq: AtomicFloat,
    pub modulation: AtomicFloat,
}

impl Default for Parameters {
    // ------------------- //
    // 2. Setting defaults //
    // ------------------- //
    fn default() -> Self {
        Self {
            freq: AtomicFloat::new(0.44),
            modulation: AtomicFloat::new(1.),
        }
    }
}

impl PluginParameters for Parameters {
    // ----------------------------------------- //
    // 3. Required get and set parameter methods //
    // ----------------------------------------- //

    fn get_parameter(&self, index: i32) -> f32 {
        match FromPrimitive::from_i32(index) {
            Some(Parameter::Freq) => self.freq.get(),
            Some(Parameter::Modulation) => self.modulation.get(),
            _ => 0f32,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match FromPrimitive::from_i32(index) {
            Some(Parameter::Freq) => self.freq.set(value),
            Some(Parameter::Modulation) => self.modulation.set(value),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        let param: Option<Parameter> = FromPrimitive::from_i32(index);
        param
            .map(|f| f.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }
}

// ---------------------------------------- //
// 4. Tagged enum instead of magic numbers  //
// ---------------------------------------- //
#[derive(FromPrimitive, Clone, Copy)]
pub enum Parameter {
    Freq = 0,
    Modulation = 1,
}

// --------------------------------------------- //
// 5. Optional display to make things look nice  //
// --------------------------------------------- //
impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Parameter::Freq => "frequency",
                Parameter::Modulation => "modulation",
            }
        )
    }
}
```

### 1. Creating Parameters

First, we define a `Parameters` struct. Later, we will add this as a field in our synth, and enable real-time modulation through our host. This struct has two parameters: `pitch`, and `modulation`[^mod]. We use the thread-safe `vst::util::AtomicFloat` type, as parameter values are shared across both audio and UI threads. 

### 2. Setting defaults

Keep in mind that float parameters in VSTs are always between `0.0` and `1.0`. If we want to create a 440Hz sine wave, we will need to scale our normalized `pitch` value by some factor. Here, we set the `pitch` to `0.44`, which will allow us to scale it by `1000.` later for a final number of `440Hz`.

We set `modulation` to `1.` here, but could set it to whatever we like within that range.

### 3. Getting and setting

Time for boilerplate. For our parameters to work correctly, we need to implement `get_parameter` and `set_parameter`.

### 4. Tagged enum

We use a tagged enum to provide a friendlier interface when accessing parameters. Without this, we would need to get and set parameters by an arbitrary ID (0, 1, 2, ...). This is where our `num-derive` crate can generate code that will let us match `i32`s as our `Parameter` enum.

### 5. Display

At this point, our plugin host does not know the name of any parameters. To resolve this, we implement `Display` on `Parameter` and ` PluginParameters::get_parameter_name`.

## Adding parameters to `fundsp`

Let's modify our `lib.rs` file to match the following. (Don't replace your `process` block!):

```rs
// ------------------------------ //
// 0. The params module & imports //
// ------------------------------ //
mod params;

use fundsp::hacker::*;
use params::{Parameter, Parameters};
use std::sync::Arc;
use vst::prelude::*;

const FREQ_SCALAR: f64 = 1000.;

struct Synthy {
    audio: Box<dyn AudioUnit64 + Send>,
    // ---------------------------------------- //
    // 1. Adding a thread-safe parameters field //
    // ---------------------------------------- //
    parameters: Arc<Parameters>,
}

impl Plugin for Synthy {
    #[allow(clippy::precedence)]
    fn new(_host: HostCallback) -> Self {
        // --------------------------------------- //
        // 2. Adding parameters to our audio graph //
        // --------------------------------------- //
        let Parameters { freq, modulation } = Parameters::default();
        let hz = freq.get() as f64 * FREQ_SCALAR;

        let freq = || tag(Parameter::Freq as i64, hz);
        let modulation = || tag(Parameter::Modulation as i64, modulation.get() as f64);

        let audio_graph =
            freq() >> sine() * freq() * modulation() + freq() >> sine() >> split::<U2>();

        Self {
            audio: Box::new(audio_graph) as Box<dyn AudioUnit64 + Send>,
            parameters: Default::default(),
        }
    }

    // ---------------------------------- //
    // 3. Revealing parameters to our DAW //
    // ---------------------------------- //
    fn get_info(&self) -> Info {
        Info {
            name: "synthy".into(),
            vendor: "rusty".into(),
            unique_id: 128956,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: 2,
            ..Info::default()
        }
    }

    // --------------- //
    // 4. Housekeeping //
    // --------------- //
    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.parameters) as Arc<dyn PluginParameters>
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, mut outputs) = buffer.split();
        if outputs.len() == 2 {
            let (left, right) = (outputs.get_mut(0), outputs.get_mut(1));

            // ---------------------------- //
            // 5. Modifying the audio graph //
            // ---------------------------- //
            for (left_chunk, right_chunk) in left
                .chunks_mut(MAX_BUFFER_SIZE)
                .zip(right.chunks_mut(MAX_BUFFER_SIZE))
            {
                let mut left_buffer = [0f64; MAX_BUFFER_SIZE];
                let mut right_buffer = [0f64; MAX_BUFFER_SIZE];

                self.audio.set(
                    Parameter::Modulation as i64,
                    self.parameters.get_parameter(Parameter::Modulation as i32) as f64,
                );

                self.audio.set(
                    Parameter::Freq as i64,
                    self.parameters.get_parameter(Parameter::Freq as i32) as f64 * FREQ_SCALAR,
                );

                self.audio.process(
                    MAX_BUFFER_SIZE,
                    &[],
                    &mut [&mut left_buffer, &mut right_buffer],
                );

                for (chunk, output) in left_chunk.iter_mut().zip(left_buffer.iter()) {
                    *chunk = *output as f32;
                }

                for (chunk, output) in right_chunk.iter_mut().zip(right_buffer.iter()) {
                    *chunk = *output as f32;
                }
            }
        }
    }
}

vst::plugin_main!(Synthy);
```

### 0. Modules and imports

Here we add our recently created `params` module, and import some required items.

### 1. Adding a parameters field

We add a new field to our `Synthy` struct titled `params` with the type of `Arc<Parameters>`. Audio and UI threads share `Parameters`, which explains the use of `Arc<_>`.

### 2. Adding parameters to our audio graph

In `fundsp`, `tag(..)` nodes define queryable variables. A `tag(..)` takes an identifier used to query the value, as well as a starting value. For easier composition, we create a couple closures to create any tag nodes. This is especially useful when composing the audio graph with repeated tags.

Consider the following syntax:

```rs
let freq = || tag(Parameter::Freq as i64, hz);
let modulation = || tag(Parameter::Modulation as i64, modulation.get() as f64);

let audio_graph = freq() >> sine() * freq() * modulation() + freq() >> sine() >> split::<U2>();
```

Without closures, the equivalent code reads:

```rs
let audio_graph = tag(Parameter::Freq as i64, hz) >> sine() * tag(Parameter::Freq as i64, hz) * tag(Parameter::Modulation as i64, modulation.get() as f64) + tag(Parameter::Freq as i64, hz) >> sine() >> split::<U2>();
```

The first version is much more legible.

We also destructure a default `Parameters` struct to provide default values for our audio graph. Lots of defaults. 

### 3. Revealing parameters to our DAW

Our host still needs to know _how many_ parameters are available. We specify this in our `Info` struct with the `parameters: 2,` line.

### 4. Housekeeping

Now that we have parameters, we also need to implement this function. All it does is clone our parameters object for access.

### 5. Modifying the audio graph

Before processing audio, we get our parameter values and apply them to our audio graph using the `set` method of our `AudioUnit64`. Sharp eyes might spot an issue: parameters are set every 64 samples. That shouldn't be an issue here.

## Summary

With this code, we should now have a functioning FM synthesizer. Let's open it up in our host or DAW of choice, and see what it sounds like. You'll hopefully notice two new parameters: "frequency" and "modulation". If you tweak these parameters, you should be able to hear the difference in pitch and timbre

### What does it sound like?

The following is a sample of the synth playing while the host automates its `pitch` and `modulation` . With little code, we're able to achieve something that actually sounds somewhat interesting. 

{{ audio(src="modulation") }}

## Project files

[Download the project files so far here](/projects/synthy/synthy-02.zip).

---

## Footnotes

[^fundsp]: Note that `fundsp` is in active development, and it's possible by the time you read this that some parts of the API have changed or moved. For example, the tagging described later in this article literally came out the day I wrote this. I love Rust's community.

[^rust]: This is an excellent display of both the ingenuity of the Rust community (specifically [Sami Perttu](https://github.com/SamiPerttu), the crate's author) and the extensibility and expressiveness that the Rust language provides. For a full list of operators that `fundsp` provides, please check out the excellent documentation provided in its [README.md](https://github.com/SamiPerttu/fundsp).

[^style]: Experiment with these lines to make them feel more comfortable with your style of Rust. Note that this method only handles stereo audio, which is fine for our purposes.

[^compatibility]: Your host probably supports natively tweaking parameters, but it might not. It depends on what you're using. 

[^mod]: In case you like to shorten variable names, remember that `mod` is a reserved keyword in Rust.