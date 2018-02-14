mod triangle;

use synth::oscillator::triangle::Triangle;
use synth::sample_stream::SampleStream;

pub struct Oscillator {
    triangle: Triangle,
}

impl Oscillator {
    pub fn new(base_frequency: f32) -> Oscillator {
        Oscillator {
            triangle: Triangle::new(base_frequency),
        }
    }

    pub fn set_base_frequency(&mut self, base_frequency: f32) {
        self.triangle.set_base_frequency(base_frequency);
    }
}

impl SampleStream for Oscillator {
    type Sample = f32;

    fn next_sample(&mut self) -> Self::Sample {
        self.triangle.next_sample()
    }
}

iterator!(Oscillator);
