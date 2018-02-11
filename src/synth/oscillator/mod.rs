mod triangle;

use self::triangle::Triangle;

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

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.triangle.next()
    }
}
