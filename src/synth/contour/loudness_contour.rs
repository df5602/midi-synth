use std::cell::Cell;

use synth::sample_stream::SampleStream;

pub struct LoudnessContour<T: SampleStream> {
    input: T,
    on: Cell<bool>,
}

impl<T: SampleStream> LoudnessContour<T> {
    pub fn new(input: T) -> Self {
        Self {
            input,
            on: Cell::new(false),
        }
    }

    pub fn trigger_on(&self) {
        self.on.set(true);
    }

    pub fn trigger_off(&self) {
        self.on.set(false);
    }
}

impl<T: SampleStream> SampleStream for LoudnessContour<T> {
    fn next_sample(&self) -> f32 {
        if self.on.get() {
            self.input.next_sample()
        } else {
            0.0
        }
    }
}

#[cfg(test)]
impl<'a, T: SampleStream> Iterator for &'a LoudnessContour<T> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::rc::Rc;

    use synth::oscillator::Oscillator;

    #[test]
    fn sound_is_off_by_default() {
        let osc = Rc::new(Oscillator::new(1.0, 0.0375));
        let mut contour = &LoudnessContour::new(osc);

        assert_float_eq!(0.0, contour.next().unwrap(), 1e-6);
        assert_float_eq!(0.0, contour.next().unwrap(), 1e-6);
        assert_float_eq!(0.0, contour.next().unwrap(), 1e-6);
    }

    #[test]
    fn trigger_on_starts_envelope_generator() {
        let osc = Rc::new(Oscillator::new(1.0, 0.0375));
        let ref_osc = Oscillator::new(1.0, 0.0375);
        let contour = LoudnessContour::new(osc);

        contour.trigger_on();

        let mut i = 0;
        println!();
        for (with_contour, reference) in contour.take(10).zip(ref_osc) {
            println!("Contour output: {}, Reference: {}", with_contour, reference);
            assert_float_eq!(reference, with_contour, 1e-6);
            i += 1;
        }
        assert_eq!(i, 10);
    }

    #[test]
    fn trigger_off_stops_envelope_generator() {
        let osc = Rc::new(Oscillator::new(1.0, 0.0375));
        let ref_osc = Oscillator::new(1.0, 0.0375);
        let mut contour = &LoudnessContour::new(osc);

        contour.trigger_on();

        let mut i = 0;
        println!();
        for (with_contour, reference) in contour.take(10).zip(ref_osc) {
            println!("Contour output: {}, Reference: {}", with_contour, reference);
            assert_float_eq!(reference, with_contour, 1e-6);
            i += 1;
        }
        assert_eq!(i, 10);

        contour.trigger_off();

        assert_float_eq!(0.0, contour.next().unwrap(), 1e-6);
        assert_float_eq!(0.0, contour.next().unwrap(), 1e-6);
        assert_float_eq!(0.0, contour.next().unwrap(), 1e-6);
    }
}
