use std::sync::mpsc::Receiver;

use synth::oscillator::Oscillator;
use synth::sample_stream::SampleStream;

pub struct Synthesizer {
    osc1: Oscillator,
    freq_in: Receiver<f32>,
}

impl Synthesizer {
    pub fn new(freq_in: Receiver<f32>) -> Synthesizer {
        Synthesizer {
            osc1: Oscillator::new(0.04),
            freq_in: freq_in,
        }
    }
}

impl SampleStream for Synthesizer {
    type Sample = f32;

    fn next_sample(&mut self) -> Self::Sample {
        if let Ok(f) = self.freq_in.try_recv() {
            self.osc1.set_base_frequency(f);
        }

        self.osc1.next_sample()
    }
}

iterator!(Synthesizer);
