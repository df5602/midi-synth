use std::sync::mpsc::Receiver;
use std::rc::Rc;

use synth::oscillator::Oscillator;
use synth::mixer::Mixer;
use synth::sample_stream::SampleStream;
use synth::dispatcher::SynthControl;

pub struct Synthesizer {
    osc1: Rc<Oscillator>,
    mixer: Mixer,
    ctrl_in: Receiver<SynthControl>,
}

impl Synthesizer {
    pub fn new(ctrl_in: Receiver<SynthControl>) -> Self {
        let osc1 = Rc::new(Oscillator::new(1.0, 0.0));
        Self {
            osc1: Rc::clone(&osc1),
            mixer: Mixer::new(osc1),
            ctrl_in: ctrl_in,
        }
    }
}

impl SampleStream for Synthesizer {
    type Sample = f32;

    fn next_sample(&self) -> Self::Sample {
        if let Ok(f) = self.ctrl_in.try_recv() {
            match f {
                SynthControl::MasterTune(frequency) => self.osc1.set_master_tune(frequency),
                SynthControl::Oscillator1Range(range) => self.osc1.set_range(range),
                SynthControl::Oscillator1Enable(enabled) => self.mixer.set_enabled(enabled),
                SynthControl::Oscillator1Volume(volume) => self.mixer.set_volume(volume),
                SynthControl::NoteOn(note) => self.osc1.set_note(note),
            }
        }

        self.mixer.next_sample()
    }
}

iterator!(Synthesizer);
