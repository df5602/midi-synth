use std::cell::Cell;
use std::rc::Rc;

use synth::oscillator::Oscillator;
use synth::sample_stream::SampleStream;

pub struct Mixer {
    osc1: Rc<Oscillator>,
    osc1_enabled: Cell<bool>,
    osc1_volume: Cell<f32>,
}

impl Mixer {
    pub fn new(osc1: Rc<Oscillator>) -> Self {
        Self {
            osc1,
            osc1_enabled: Cell::new(false),
            osc1_volume: Cell::new(0.0),
        }
    }

    // TODO: take additional enum specifying which input and reuse set_enabled() for all inputs
    pub fn set_enabled(&self, enabled: bool) {
        self.osc1_enabled.set(enabled);
    }

    // TODO: take additional enum specifying which input and reuse set_volume() for all inputs
    pub fn set_volume(&self, volume: f32) {
        self.osc1_volume.set(volume);
    }
}

impl SampleStream for Mixer {
    fn next_sample(&self) -> f32 {
        if self.osc1_enabled.get() {
            self.osc1.next_sample() * self.osc1_volume.get()
        } else {
            0.0
        }
    }
}

iterator!(Mixer);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enable_oscillator1() {
        let osc1 = Rc::new(Oscillator::new(1.0, 0.0375));
        let ref_osc1 = Oscillator::new(1.0, 0.0375);
        let mut mixer = Mixer::new(osc1);
        mixer.set_volume(1.0);
        mixer.set_enabled(false);

        assert_float_eq!(0.0, mixer.next().unwrap(), 1e-6);
        assert_float_eq!(0.0, mixer.next().unwrap(), 1e-6);
        assert_float_eq!(0.0, mixer.next().unwrap(), 1e-6);

        mixer.set_enabled(true);

        let mut i = 0;
        println!();
        for (mixed, reference) in mixer.take(10).zip(ref_osc1) {
            println!("Mixer output: {}, Reference: {}", mixed, reference);
            assert_float_eq!(reference, mixed, 1e-6);
            i += 1;
        }
        assert_eq!(i, 10);
    }

    #[test]
    fn oscillator1_volume() {
        let osc1 = Rc::new(Oscillator::new(1.0, 0.0375));
        let ref_osc1 = Oscillator::new(1.0, 0.0375);
        let mixer = Mixer::new(osc1);
        mixer.set_enabled(true);
        mixer.set_volume(0.5);

        let mut i = 0;
        println!();
        for (mixed, reference) in mixer.take(10).zip(ref_osc1) {
            println!("Mixer output: {}, Reference: {}", mixed, reference);
            assert_float_eq!(reference * 0.5, mixed, 1e-6);
            i += 1;
        }
        assert_eq!(i, 10);
    }
}
