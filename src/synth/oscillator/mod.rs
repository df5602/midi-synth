mod triangle;

use synth::oscillator::triangle::Triangle;
use synth::sample_stream::SampleStream;

pub struct Oscillator {
    enabled: bool,
    triangle: Triangle,
}

impl Oscillator {
    pub fn new(master_tune: f32, range: f32, enabled: bool) -> Oscillator {
        Oscillator {
            enabled: enabled,
            triangle: Triangle::new(master_tune, range),
        }
    }

    pub fn set_master_tune(&mut self, master_tune: f32) {
        self.triangle.set_master_tune(master_tune);
    }

    pub fn set_range(&mut self, range: f32) {
        self.triangle.set_range(range);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl SampleStream for Oscillator {
    type Sample = f32;

    fn next_sample(&mut self) -> Self::Sample {
        if self.enabled {
            self.triangle.next_sample()
        } else {
            0.0
        }
    }
}

iterator!(Oscillator);
