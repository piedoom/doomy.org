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

impl Plugin for Synthy {
    // -------------- //
    // 2. Plugin info //
    // -------------- //
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
            // 3a. Filling the buffer with random numbers
            rand::thread_rng().fill(*buffer);
            // 3b. Adjusting amplitude
            for sample in buffer.iter_mut() {
                *sample *= self.params.amplitude.value;
            }
        }
        // 3c. Returning a status
        ProcessStatus::Normal
    }

    fn params(&self) -> std::pin::Pin<&dyn Params> {
        self.params.as_ref()
    }
}

// ---------------------- //
// 4. Building the plugin //
// ---------------------- //
impl Vst3Plugin for Synthy {
    const VST3_CLASS_ID: [u8; 16] = *b"1234567891234567";
    const VST3_CATEGORIES: &'static str = "Instrument|Synth";
}

nih_export_vst3!(Synthy);
