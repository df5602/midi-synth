pub struct Triangle {
    base_frequency: f32,
    next_value: f32,
    slope_direction: f32,
}

impl Triangle {
    pub fn new(base_frequency: f32) -> Triangle {
        Triangle {
            base_frequency: base_frequency,
            next_value: 0.0,
            slope_direction: 1.0,
        }
    }

    pub fn set_base_frequency(&mut self, base_frequency: f32) {
        self.base_frequency = base_frequency;
    }
}

impl Iterator for Triangle {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let output_value = self.next_value;

        self.next_value += self.base_frequency * self.slope_direction;
        if self.next_value >= 1.0 || self.next_value <= -1.0 {
            self.slope_direction *= -1.0;
            let mut diff = if self.next_value >= 1.0 {
                self.next_value - 1.0
            } else {
                -1.0 - self.next_value
            };

            if diff == 0.0 {
                diff += 1e-7;
            }

            self.next_value += 2.0 * diff * self.slope_direction;
        }

        Some(output_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_triangle() {
        let triangle = Triangle::new(0.15);

        let samples = [
            0.0, 0.15, 0.3, 0.45, 0.6, 0.75, 0.9, 0.95, 0.8, 0.65, 0.5, 0.35, 0.2, 0.05, -0.1,
            -0.25, -0.4, -0.55, -0.7, -0.85, -1.0, -0.85, -0.7, -0.55, -0.4, -0.25, -0.1, 0.05,
        ];

        let mut i = 0;
        println!();
        for sample in triangle {
            if i == samples.len() {
                break;
            }
            println!("Is: {}, should be: {}", sample, samples[i]);
            assert!((sample - samples[i]).abs() < 1e-6);
            i += 1;
        }
        assert_eq!(i, samples.len());
    }

    #[test]
    fn double_frequency() {
        let mut triangle = Triangle::new(0.15);
        triangle.set_base_frequency(0.3);

        let samples = [
            0.0, 0.3, 0.6, 0.9, 0.8, 0.5, 0.2, -0.1, -0.4, -0.7, -1.0, -0.7, -0.4, -0.1, 0.2
        ];

        let mut i = 0;
        println!();
        for sample in triangle {
            if i == samples.len() {
                break;
            }
            println!("Is: {}, should be: {}", sample, samples[i]);
            assert!((sample - samples[i]).abs() < 1e-6);
            assert!(sample < 1.0 && sample > -1.0);
            i += 1;
        }
        assert_eq!(i, samples.len());
    }

    #[test]
    fn limit_output_to_1() {
        let triangle = Triangle::new(0.2);

        let samples = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 0.8, 0.6, 0.4, 0.2, 0.0];

        let mut i = 0;
        println!();
        for sample in triangle {
            if i == samples.len() {
                break;
            }
            println!("Is: {}, should be: {}", sample, samples[i]);
            assert!((sample - samples[i]).abs() < 1e-6);
            assert!(sample < 1.0 && sample > -1.0);
            i += 1;
        }
        assert_eq!(i, samples.len());
    }
}
