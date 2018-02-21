use std::rc::Rc;
use std::cell::Cell;

use synth::sample_stream::SampleStream;
use synth::oscillator::Oscillator;

pub struct Mixer {
    osc1: Rc<Oscillator>,
    osc1_enabled: Cell<bool>,
}

impl Mixer {
    pub fn new(osc1: Rc<Oscillator>) -> Self {
        Self {
            osc1: osc1,
            osc1_enabled: Cell::new(false),
        }
    }

    // TODO: take additional enum specifying which input and reuse set_enabled() for all inputs
    pub fn set_enabled(&self, enabled: bool) {
        self.osc1_enabled.set(enabled);
    }
}

impl SampleStream for Mixer {
    type Sample = f32;

    fn next_sample(&self) -> Self::Sample {
        if self.osc1_enabled.get() {
            self.osc1.next_sample()
        } else {
            0.0
        }
    }
}

iterator!(Mixer);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_float_eq {
        ($left:expr, $right:expr, $eps:expr) => {{
            let left = $left;
            let right = $right;
            assert!((left - right).abs() < $eps, "Expected: {}, got: {}", left, right);
        }};
    }

    #[test]
    fn enable_oscillator1() {
        let osc1 = Rc::new(Oscillator::new(1.0, 0.0375));
        let ref_osc1 = Oscillator::new(1.0, 0.0375);
        let mut mixer = Mixer::new(osc1);
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
}
