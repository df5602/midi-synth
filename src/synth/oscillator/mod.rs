mod triangle;

use synth::oscillator::triangle::Triangle;
use synth::sample_stream::SampleStream;

pub struct Oscillator {
    triangle: Triangle,
}

impl Oscillator {
    pub fn new(master_tune: f32, range: f32) -> Oscillator {
        Oscillator {
            triangle: Triangle::new(master_tune, range),
        }
    }

    pub fn set_master_tune(&self, master_tune: f32) {
        self.triangle.set_master_tune(master_tune);
    }

    pub fn set_range(&self, range: f32) {
        self.triangle.set_range(range);
    }
}

impl SampleStream for Oscillator {
    type Sample = f32;

    fn next_sample(&self) -> Self::Sample {
        self.triangle.next_sample()
    }
}

iterator!(Oscillator);
