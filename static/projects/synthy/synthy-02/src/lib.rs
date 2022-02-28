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
