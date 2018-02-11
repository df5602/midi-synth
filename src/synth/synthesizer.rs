use std::sync::mpsc::Receiver;

use synth::oscillator::Oscillator;

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

impl Iterator for Synthesizer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(f) = self.freq_in.try_recv() {
            self.osc1.set_base_frequency(f);
        }

        self.osc1.next()
    }
}
