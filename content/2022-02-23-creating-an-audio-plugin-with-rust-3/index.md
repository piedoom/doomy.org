+++
title = "Rust VST part 3: reacting to MIDI"
author = "doomy" 
date = 2022-02-23
summary = "Creating an audio plugin with the Rust programming language: Part 3"
draft = true

[taxonomies] 
tags = ["rust", "audio"]

[extra]
prev="2022-02-23-creating-an-audio-plugin-with-rust-2/index.md"
next="2022-02-27-creating-an-audio-plugin-with-rust-4/index.md"
hidden=true
+++

### Project files

We'll be building off of code built in the previous chapter. I recommend you take a look, or [download the project files here](/projects/synthy/synthy-02.zip).

## Upgrading our synthesizer

So far, we've built a simple FM synthesizer. It can change pitch and modulation, but generally we want to react to MIDI when using a synthesizer. In this chapter, we upgrade our monophonic synth to respond to MIDI events [^monophonic].

To do this, we save the latest MIDI note-on event in our `Synthy` struct, and adjust our pitch to match the current note. We'll also need to include some logic for note-off, as well as an envelope to control our audio graph.

To improve MIDI ergonomics, pull the `wmidi` crate into your `Cargo.toml` dependencies:

{{ filename(name="Cargo.toml") }}
```ini
[dependencies]
# ...
wmidi = "4"
# ...
```

Implement the `Plugin::process_events` trait method and see how it converts and destructures MIDI data. 

```rs
fn process_events(&mut self, events: &vst::api::Events) {
    for event in events.events() {
        if let vst::event::Event::Midi(midi) = event {
            if let Ok(midi) = wmidi::MidiMessage::try_from(midi.data.as_slice()) {
                match midi {
                    wmidi::MidiMessage::NoteOff(channel, note, velocity) => todo!(),
                    wmidi::MidiMessage::NoteOn(channel, note, velocity) => todo!(),
                    _ => (), // A ton more hidden here that we don't care about right now
                }
            }
        }
    }
}
```

In this example, we iterate through `events` with the `vst-rs` method `.events()`. We then get the MIDI data and try to parse it using the `wmidi` crate. If successful, we can then respond to `NoteOff` and `NoteOn` events [^sysex]. To save the current note, we use an option to store a note after a `NoteOn` event, and then remove it on a `NoteOff`.

All together, the code in our `lib.rs` should look like this:

```rs
mod params;

use fundsp::hacker::*;
use params::{Parameter, Parameters};
use std::{convert::TryFrom, sync::Arc};
use vst::prelude::*;
use wmidi::{Note, Velocity};

const FREQ_SCALAR: f64 = 1000.;

struct Synthy {
    audio: Box<dyn AudioUnit64 + Send>,
    parameters: Arc<Parameters>,
    // -------------------------------- //
    // 1. Storing the note as an option //
    // -------------------------------- //
    note: Option<(Note, Velocity)>,
}

impl Plugin for Synthy {
    #[allow(clippy::precedence)]
    fn new(_host: HostCallback) -> Self {
        let Parameters { freq, modulation } = Parameters::default();
        let hz = freq.get() as f64 * FREQ_SCALAR;

        let freq = || tag(Parameter::Freq as i64, hz);
        let modulation = || tag(Parameter::Modulation as i64, modulation.get() as f64);

        let audio_graph =
            freq() >> sine() * freq() * modulation() + freq() >> sine() >> split::<U2>();

        Self {
            audio: Box::new(audio_graph) as Box<dyn AudioUnit64 + Send>,
            parameters: Default::default(),
            note: None,
        }
    }

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

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.parameters) as Arc<dyn PluginParameters>
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, mut outputs) = buffer.split();
        if outputs.len() == 2 {
            let (left, right) = (outputs.get_mut(0), outputs.get_mut(1));
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

                // ------------------------ //
                // 2. Setting the frequency //
                // ------------------------ //
                self.audio.set(
                    Parameter::Freq as i64,
                    self.note.map(|(n, ..)| n.to_freq_f64()).unwrap_or(0.),
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

    fn process_events(&mut self, events: &vst::api::Events) {
        for event in events.events() {
            if let vst::event::Event::Midi(midi) = event {
                if let Ok(midi) = wmidi::MidiMessage::try_from(midi.data.as_slice()) {
                    // ------------------------- //
                    // 3. Processing MIDI events //
                    // ------------------------- //
                    match midi {
                        wmidi::MidiMessage::NoteOn(_channel, note, velocity) => {
                            self.note = Some((note, velocity));
                        }
                        wmidi::MidiMessage::NoteOff(_channel, note, _velocity) => {
                            if let Some((current_note, ..)) = self.note {
                                if current_note == note {
                                    self.note = None;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

vst::plugin_main!(Synthy);
```

### 1. Storing the note as an option

We use the `Note` and `Velocity` types from the `wmidi` crate (see new `use` imports) to store an optional note. This code doesn't react to `Velocity` yet, but we save it here for later use.

### 2. Setting the frequency

We use `wmidi`'s lovely `to_freq_f64` method to easily get the correct pitch to play. If the note is `None`, we provide a frequency of `0.`, which is inaudible. In other words, the synth stops playing when the note is `None`. (This is _not_ the proper way to start and stop playing notes, but it will work for this step.) Note that we also moved the parameter-setting part into the more predictably sized buffer used for `fundsp`.

### 3. Processing MIDI events

The `NoteOn` branch is simple to understand: we set the `note` to new values (ignoring the channel for now). The `NoteOff` branch is slightly more complex. We don't want to remove the note on every `NoteOff` event. Imagine playing two notes in succession, without stopping the first note. Now, imagine that we stop the first note after the second note began playing. The second note will stop playing too! This would be very frustrating to musicians [^hmm], so we fix that by checking that the `NoteOff` note is the same as the currently playing note. We ignore all other events for now.

## Monophonic FM synth

Compile your plugin with `cargo build --release`, and open in your host. It took 3 chapters, but we finally figured out how to generate silence! Try playing a note, and see how the synth reacts.

{{ audio(src="sample-pitch") }}

## Generating an envelope

You may see a problem with this naive implementation: the `0.` Hz signal is not necessarily `0.` amplitude. Additionally, we have no way to adjust the attack or release - it's either on or off. This results in a sound riddled with clicks and pops. Luckily, `fundsp` also comes with envelope generation functions. For a practical example, consider the following:

```rs
let offset = || tag(Parameter::SomeTag, 0.);
let env = || offset() >> envelope2(|t, offset| downarc((t - offset) * 2.));
```

Here, we create a new tag that controls the offset of an envelope. This is useful when responding to a note on event, as we can control exactly when to "trigger" the envelope. `downarc` is a simple curve function that eases in and out. The output of this function applied as amplitude to our FM signal looks like this:

![Oscilloscope for the `downarc` function applied to amplitude looks like a circle (or also kind of like the Pepsi logo)](visualizer.png)

While not as versatile as a proper [ASDR](https://en.wikipedia.org/wiki/Envelope_(music)), this solution will get us acquainted with envelope generation and eliminate clicking artifacts created by instantaneously starting or stopping a signal.

### Differentiating parameters and tags

You might start to realize that we overloaded the functionality of our `Parameter` enum. We use it in both our `fundsp` audio graph tags and our `Plugin` parameters. Additionally, we no longer use the `Parameter::Freq` tag, as MIDI notes now determine pitch. This code rot is a consequence of building on our initial naive design that assumes `Tags` and `Parameters` are the same. We need separate enums. 

Replace your `params.rs` file with the following to remove all references to `Parameter::Freq`. Note that this will break our code for the next few blocks. We'll fix everything later.

{{ filename(name="src/params.rs") }}
```rs
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt::Display;
use vst::{plugin::PluginParameters, util::AtomicFloat};

pub struct Parameters {
    pub modulation: AtomicFloat,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            modulation: AtomicFloat::new(1.),
        }
    }
}

impl PluginParameters for Parameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match FromPrimitive::from_i32(index) {
            Some(Parameter::Modulation) => self.modulation.get(),
            _ => 0f32,
        }
    }

    #[allow(clippy::single_match)]
    fn set_parameter(&self, index: i32, value: f32) {
        match FromPrimitive::from_i32(index) {
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

#[derive(FromPrimitive, Clone, Copy)]
pub enum Parameter {
    Modulation = 0,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Parameter::Modulation => "modulation",
            }
        )
    }
}
```

Now, let's move back to our main `lib.rs` file and create a new tagged enum for our `fundsp` `tag` nodes:

{{ filename(name="src/lib.rs") }}
```rs
use num_derive::FromPrimitive;

#[derive(FromPrimitive, Clone, Copy)]
pub enum Tag {
    Freq = 0,
    Modulation = 1,
    NoteOn = 2,
}
```

With a new `Tag` enum, we now define audio graph labels independently of our `Plugin` parameters. One flaw with this design is that integer values represent both tags and plugin parameters. This means we could mistakenly refer to an audio graph tag with a `Parameter` enum cast as an integer, or vice versa. To reduce the likelihood of this error, we implement some helper methods on `Synthy`:

{{ filename(name="src/lib.rs") }}
```rs
impl Synthy {
    #[inline(always)]
    fn set_tag(&mut self, tag: Tag, value: f64) {
        self.audio.set(tag as i64, value);
    }

    #[inline(always)]
    fn set_tag_with_param(&mut self, tag: Tag, param: Parameter) {
        self.set_tag(tag, self.parameters.get_parameter(param as i32) as f64);
    }
}
```

This way, we leave casting to an integer up to the function and ensure the enum type provided is correct. Instead of:

```rs
self.audio.set(
    // Could accidentally be written with a parameter instead of a tag!
    // Parameter::Modulation as i64 would also work but could have 
    // unintended effects if the numbers did not match
    Tag::Modulation as i64,  
    self.parameters.get_parameter(Parameter::Modulation as i32) as f64,
);
```

We can now write:

```rs
// We always specify a `Tag` and the function ensures it is used correctly
self.set_tag_with_param(Tag::Modulation, Parameter::Modulation);
```

## Putting it all together

We now have the building blocks needed to understand how things work in context. Replace your `lib.rs` file with the following:


{{ filename(name="src/lib.rs") }}
```rs
mod params;

use fundsp::hacker::*;
use num_derive::FromPrimitive;
use params::{Parameter, Parameters};
use std::{convert::TryFrom, sync::Arc, time::Duration};
use vst::prelude::*;
use wmidi::{Note, Velocity};

struct Synthy {
    audio: Box<dyn AudioUnit64 + Send>,
    note: Option<(Note, Velocity)>,
    parameters: Arc<Parameters>,
    // ------------- //
    // 1. New fields //
    // ------------- //
    enabled: bool,
    sample_rate: f32,
    time: Duration,
}

impl Plugin for Synthy {
    #[allow(clippy::precedence)]
    fn new(_host: HostCallback) -> Self {
        // ------------------------------ //
        // 2. Removal of Parameters::Freq //
        // ------------------------------ //
        let Parameters { modulation } = Parameters::default();

        let freq = || tag(Tag::Freq as i64, 440.);
        let modulation = || tag(Tag::Modulation as i64, modulation.get() as f64);

        // ---------------------- //
        // 3. Envelope generation //
        // ---------------------- //
        let offset = || tag(Tag::NoteOn as i64, 0.);
        let env = || offset() >> envelope2(|t, offset| downarc((t - offset) * 2.));

        let audio_graph =
            freq() >> sine() * freq() * modulation() + freq() >> env() * sine() >> split::<U2>();

        Self {
            audio: Box::new(audio_graph) as Box<dyn AudioUnit64 + Send>,
            parameters: Default::default(),
            note: None,
            time: Duration::default(),
            sample_rate: 41_000f32,
            enabled: false,
        }
    }

    // --------------------------- //
    // 4. Changing to 1 parameters //
    // --------------------------- //
    fn get_info(&self) -> Info {
        Info {
            name: "synthy".into(),
            vendor: "rusty".into(),
            unique_id: 128956,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: 1,
            ..Info::default()
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.parameters) as Arc<dyn PluginParameters>
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, mut outputs) = buffer.split();
        if outputs.len() == 2 {
            let (left, right) = (outputs.get_mut(0), outputs.get_mut(1));
            for (left_chunk, right_chunk) in left
                .chunks_mut(MAX_BUFFER_SIZE)
                .zip(right.chunks_mut(MAX_BUFFER_SIZE))
            {
                let mut left_buffer = [0f64; MAX_BUFFER_SIZE];
                let mut right_buffer = [0f64; MAX_BUFFER_SIZE];

                self.set_tag_with_param(Tag::Modulation, Parameter::Modulation);

                if let Some((note, ..)) = self.note {
                    self.set_tag(Tag::Freq, note.to_freq_f64())
                }

                if self.enabled {
                    // -------------- //
                    // 5. Timekeeping //
                    // -------------- //
                    self.time += Duration::from_secs_f32(MAX_BUFFER_SIZE as f32 / self.sample_rate);
                    self.audio.process(
                        MAX_BUFFER_SIZE,
                        &[],
                        &mut [&mut left_buffer, &mut right_buffer],
                    );
                }

                for (chunk, output) in left_chunk.iter_mut().zip(left_buffer.iter()) {
                    *chunk = *output as f32;
                }

                for (chunk, output) in right_chunk.iter_mut().zip(right_buffer.iter()) {
                    *chunk = *output as f32;
                }
            }
        }
    }

    fn process_events(&mut self, events: &vst::api::Events) {
        for event in events.events() {
            if let vst::event::Event::Midi(midi) = event {
                if let Ok(midi) = wmidi::MidiMessage::try_from(midi.data.as_slice()) {
                    match midi {
                        wmidi::MidiMessage::NoteOn(_channel, note, velocity) => {
                            // ----------------------------------------- //
                            // 6. Set `NoteOn` time tag and enable synth //
                            // ----------------------------------------- //
                            self.set_tag(Tag::NoteOn, self.time.as_secs_f64());
                            self.note = Some((note, velocity));
                            self.enabled = true;
                        }
                        wmidi::MidiMessage::NoteOff(_channel, note, _velocity) => {
                            if let Some((current_note, ..)) = self.note {
                                if current_note == note {
                                    self.note = None;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    // ------------------------------ //
    // 7. Implement `set_sample_rate` //
    // ------------------------------ //
    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
        self.time = Duration::default();
        self.audio.reset(Some(rate as f64));
    }
}

impl Synthy {
    #[inline(always)]
    fn set_tag(&mut self, tag: Tag, value: f64) {
        self.audio.set(tag as i64, value);
    }

    #[inline(always)]
    fn set_tag_with_param(&mut self, tag: Tag, param: Parameter) {
        self.set_tag(tag, self.parameters.get_parameter(param as i32) as f64);
    }
}

#[derive(FromPrimitive, Clone, Copy)]
pub enum Tag {
    Freq = 0,
    Modulation = 1,
    NoteOn = 2,
}

vst::plugin_main!(Synthy);
```

### 1. New fields

We add `enabled`, `sample_rate`, and `time` as fields to `Synthy`. These are all related to envelope generation. 

1. The `time` field keeps track of how much time has passed since `fundsp` began processing. It _should_ be identical to the internal `t` parameter accessible within the `envelope2` function.
2. `sample_rate` is necessary for timekeeping, as it allows us to calculate time passed based on number of samples. At #7 we implement the `set_sample_rate` method of the `Plugin` trait that controls this field.
3. `enabled` is initially set to `false`. When playing a note, it is set to `true` for the remainder of the plugin's run time. This is to prevent playing an initial noise.

### 2. Removal of `Parameters::Freq`

Just as in `params.rs`, remove references to `Parameters::Freq` as MIDI now controls frequency.

### 3. Envelope generation

This is the fun part. Let's dive in and see what's happening in-depth

```rs
// 1.
let offset = || tag(Tag::NoteOn as i64, 0.);
// 2.
let env = || offset() >> envelope2(|t, offset| downarc((t - offset) * 2.));
// 3.
let audio_graph = freq()
            >> sine() * freq() * modulation() + freq()
            >> env() * sine()
            >> declick()
            >> split::<U2>();
```

1. We create a new `tag` with the ID of a new enum variant called `Tag::NoteOn` and the initial value of `0f64`. This represents the amount of seconds that have passed since our `AudioUnit64` began processing. We will later set this tag to the time that a note was pressed. This time offsets the envelope.
2. We use `envelope2`, which is like the `envelope` function, but takes an input. With `envelope2`, we can pipe our `offset` tag into the function with `downarc((t - offset)..`. The constant `2.` at the end of the line scales the speed of the envelope. Try changing this yourself to see how it affects the output, or add another parameter and tag to control it.
3. Finally, we apply the envelope to our carrier `sine()`. We also add a `delick()` node to help with audio popping.

### 4. Changing to 1 parameters

Because we eliminated `Parameters::Freq`, we should change the amount of parameters advertised in `Plugin::get_info` to `1` [^macros].

### 5. Timekeeping

In this section, we increment our `self.time` clock by calculating the `Duration` in seconds of a `MAX_BUFFER_SIZE` block [^save]. Note that none of this happens if the synth is not `enabled`. See #6 for an explanation.

### 6. Set `NoteOn` time tag and enable synth

When processing a `wmidi::MidiMessage::NoteOn` event, we now set our `Tag::NoteOn` tag with the current time. Additionally, `enabled` is set to true, indicating our synth has received input and is ready to create sound.

### 7. Implement `set_sample_rate`

To ensure our time calculations remain accurate, we implement ` Plugin ::set_sample_rate`. We also invoke the `reset` method on our `fundsp` graph, which allows us to specify a specific sample rate.

## Listening

We now have a pleasant sounding monophonic FM synth that is actually usable in music composition.

{{ audio(src="sample-env") }}

## Extending range


While simple, opportunity exists to make our synth more expressive. For example, there's no reason `Tag::modulation` can't go above `1.0`. It is only limited to that range because we directly read a parameter. We will modify our `set_tag_with_param` function later to re-map the normalized value to an arbitrary range.

A [quick search](https://stackoverflow.com/a/43045004) tells us how to map values to a range, where `x` is the input value:

```rs
fn(x) 
    = (x - input_range_start) 
    / (input_range_end - input_range_start) 
    * (output_range_end - output_range_start) 
    + output_range_start
```

Because our `input_range_start` - `input_range_end` is just `1`, we can simplify the function to:

```rs
fn(x)
    = (x - input_range_start) 
    * (output_range_end - output_range_start) 
    + output_range_start
```

We use the `RangeInclusive` type to create a nice API to remap values. Change the `set_tag_with_params` method to the following:

```rs
#[inline(always)]
fn set_tag_with_param(&mut self, tag: Tag, param: Parameter, range: RangeInclusive<f64>) {
    let value = self.parameters.get_parameter(param as i32) as f64;
    let mapped_value = (value - range.start()) * (range.end() - range.start()) + range.start();
    self.set_tag(tag, mapped_value);
}
```

Now, update the usage of this function in our `process` method to increase the maximum modulation amount from `1` to `10`:

```rs
// Plugin::process
// ...
self.set_tag_with_param(Tag::Modulation, Parameter::Modulation, 0f64..=10f64);
// ...
```

Now, a sample using our new modulation range to automate an FM bass line:

{{ audio(src="sample-mod") }}


## Project files

[Download the project files so far here](/projects/synthy/synthy-03-0.0.1.zip).

---

## Footnotes

[^monophonic]: `fundsp` will help us a ton later when we want to add polyphony. For now its easiest to understand a monophonic implementation, as polyphony requires additional considerations like [voice stealing](https://web.archive.org/web/20220224012013/https://electronicmusic.fandom.com/wiki/Voice_stealing).

[^sysex]: `wmidi` can handle a variety of messages like Sysex. We discard those messages when we match `vst::event::Event::Midi`, so keep this in mind if your application requires that functionality. If you're not sure, you likely do not need it!

[^hmm]: It could be fun to make a synth that refuses to respond properly and pretend its broken or has its own personality.

[^macros]: If you want to be really slick, you could probably write or import a macro to count the number of enum variants and provide that to the `Info` struct.

[^save]: It might be a better idea to calculate this value once in `set_sample_rate` and save it to a new `Synthy` field