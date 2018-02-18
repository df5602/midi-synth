use std::sync::mpsc::Receiver;

use synth::oscillator::Oscillator;
use synth::sample_stream::SampleStream;
use synth::dispatcher::SynthControl;

pub struct Synthesizer {
    osc1: Oscillator,
    ctrl_in: Receiver<SynthControl>,
}

impl Synthesizer {
    pub fn new(ctrl_in: Receiver<SynthControl>) -> Synthesizer {
        Synthesizer {
            osc1: Oscillator::new(1.0, 0.0),
            ctrl_in: ctrl_in,
        }
    }
}

impl SampleStream for Synthesizer {
    type Sample = f32;

    fn next_sample(&mut self) -> Self::Sample {
        if let Ok(f) = self.ctrl_in.try_recv() {
            match f {
                SynthControl::Oscillator1Range(range) => self.osc1.set_range(range),
                SynthControl::MasterTune(frequency) => self.osc1.set_master_tune(frequency),
            }
        }

        self.osc1.next_sample()
    }
}

iterator!(Synthesizer);
