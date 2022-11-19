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
            amplitude: FloatParam::new("amplitude", 0.5, Range::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl Default for Synthy {
    fn default() -> Self {
        // ------------------------ //
        // 2. Create an audio graph //
        // ------------------------ //
        let params = Arc::pin(SynthyParams::default());

        let amplitude = || tag(0, params.amplitude.value as f64);

        #[allow(clippy::precedence)]
        let audio_graph = sine_hz(440.) * amplitude() >> split::<U2>();

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

    // ------------------------ //
    // 3. Process both channels //
    // ------------------------ //
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        for (_offset, mut block) in buffer.iter_blocks(MAX_BUFFER_SIZE) {
            self.audio
                .set(0, self.params.amplitude.plain_value() as f64);

            let mut left_tmp = [0f64; MAX_BUFFER_SIZE];
            let mut right_tmp = [0f64; MAX_BUFFER_SIZE];

            self.audio
                .process(MAX_BUFFER_SIZE, &[], &mut [&mut left_tmp, &mut right_tmp]);

            for (index, channel) in block.iter_mut().enumerate() {
                let new_channel = match index {
                    0 => left_tmp,
                    1 => right_tmp,
                    _ => return ProcessStatus::Error("unexpected number of channels"),
                };
                for (sample_index, sample) in channel.iter_mut().enumerate() {
                    *sample = new_channel[sample_index] as f32;
                }
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
        self.audio.reset(Some(buffer_config.sample_rate as f64));
        true
    }
}

impl Vst3Plugin for Synthy {
    const VST3_CLASS_ID: [u8; 16] = *b"1234567891234567";
    const VST3_CATEGORIES: &'static str = "Instrument|Synth";
}

nih_export_vst3!(Synthy);
