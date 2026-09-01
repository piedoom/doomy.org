use fundsp::hacker::*;
use nih_plug::*;
use num_derive::FromPrimitive;
use std::{pin::Pin, sync::Arc};

struct Synthy {
    audio: Box<dyn AudioUnit64 + Send + Sync>,
    sample_rate: f32,
    params: Pin<Arc<SynthyParams>>,
}

#[derive(Params)]
pub struct SynthyParams {
    #[id = "modulation"]
    pub modulation: FloatParam,
    #[id = "frequency"]
    pub freq: FloatParam,
}

#[derive(FromPrimitive, Clone, Copy)]
pub enum Tag {
    Freq = 0,
    Modulation = 1,
    NoteOn = 2,
}

impl Default for SynthyParams {
    fn default() -> Self {
        Self {
            modulation: FloatParam::new(
                "modulation",
                0.0,
                Range::Linear {
                    min: 0.0,
                    max: 10.0,
                },
            ),
            freq: FloatParam::new(
                "frequency",
                440.0,
                Range::Linear {
                    min: 0.0,
                    max: 1000.0,
                },
            ),
        }
    }
}

impl Default for Synthy {
    fn default() -> Self {
        let params = Arc::pin(SynthyParams::default());

        let freq = || tag(Tag::Freq as i64, params.freq.value as f64);
        let modulation = || tag(Tag::Modulation as i64, params.modulation.value as f64);
        let offset = || tag(Tag::NoteOn as i64, 0.);
        let env = || offset() >> envelope2(|t, offset| downarc((t - offset) * 2.));

        #[allow(clippy::precedence)]
        let audio_graph = freq()
            >> sine() * freq() * modulation() + freq()
            >> env() * sine()
            >> declick()
            >> split::<U2>();

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

            self.audio
                .set(Tag::Modulation as i64, self.params.modulation.value as f64);
            self.audio
                .set(Tag::Freq as i64, self.params.freq.value as f64);

            // process buffers here
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
