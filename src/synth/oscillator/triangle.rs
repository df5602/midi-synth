use synth::sample_stream::SampleStream;

pub struct Triangle {
    base_frequency: f32,
    master_tune: f32,
    range: f32,
    sample_counter: f32,
    phase_offset: f32,
}

impl Triangle {
    pub fn new(master_tune: f32, range: f32) -> Triangle {
        Triangle {
            base_frequency: master_tune * range,
            master_tune: master_tune,
            range: range,
            sample_counter: 0.0,
            phase_offset: 0.25,
        }
    }

    pub fn set_range(&mut self, range: f32) {
        self.phase_offset += self.sample_counter * self.base_frequency;
        self.base_frequency = self.master_tune * range;
        self.range = range;
        self.sample_counter = 0.0;
    }

    pub fn set_master_tune(&mut self, master_tune: f32) {
        self.phase_offset += self.sample_counter * self.base_frequency;
        self.base_frequency = master_tune * self.range;
        self.master_tune = master_tune;
        self.sample_counter = 0.0;
    }
}

impl SampleStream for Triangle {
    type Sample = f32;

    fn next_sample(&mut self) -> Self::Sample {
        // Calculate phase angle
        // (Do it this seemingly more complicated than necessary way, since this seems to minimize
        // floating point errors)
        let mut phase_angle = self.phase_offset + self.sample_counter * self.base_frequency;

        if phase_angle >= 1.0 {
            phase_angle -= 1.0;
            self.sample_counter = 0.0;
            self.phase_offset = phase_angle;
        }

        self.sample_counter += 1.0;

        // Calculate output value
        let output_value = if phase_angle < 0.5 {
            4.0 * phase_angle - 1.0
        } else {
            1.0 - 4.0 * (phase_angle - 0.5)
        };

        assert!(output_value <= 1.0 && output_value >= -1.0);

        output_value
    }
}

iterator!(Triangle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_triangle() {
        let triangle = Triangle::new(1.0, 0.0375);

        let samples = [
            0.0, 0.15, 0.3, 0.45, 0.6, 0.75, 0.9, 0.95, 0.8, 0.65, 0.5, 0.35, 0.2, 0.05, -0.1,
            -0.25, -0.4, -0.55, -0.7, -0.85, -1.0, -0.85, -0.7, -0.55, -0.4, -0.25, -0.1, 0.05,
            0.2, 0.35, 0.5, 0.65, 0.8, 0.95,
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
        let mut triangle = Triangle::new(1.0, 0.0375);

        let samples = [
            0.3, 0.6, 0.9, 0.8, 0.5, 0.2, -0.1, -0.4, -0.7, -1.0, -0.7, -0.4, -0.1, 0.2
        ];

        assert!((triangle.next().unwrap() - 0.0).abs() < 1e-6);
        assert!((triangle.next().unwrap() - 0.15).abs() < 1e-6);

        triangle.set_range(0.075);

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
    fn master_tune() {
        let mut triangle = Triangle::new(1.0, 0.05);

        let samples = [
            0.4, 0.7, 1.0, 0.7, 0.4, 0.1, -0.2, -0.5, -0.8, -0.9, -0.6, -0.3, 0.0
        ];

        assert!((triangle.next().unwrap() - 0.0).abs() < 1e-6);
        assert!((triangle.next().unwrap() - 0.2).abs() < 1e-6);

        triangle.set_master_tune(1.5);

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
    fn limit_output_to_1() {
        let triangle = Triangle::new(1.0, 0.05);

        let samples = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 0.8, 0.6, 0.4, 0.2, 0.0];

        let mut i = 0;
        println!();
        for sample in triangle {
            if i == samples.len() {
                break;
            }
            println!("Is: {}, should be: {}", sample, samples[i]);
            assert!((sample - samples[i]).abs() < 1e-6);
            assert!(sample <= 1.0 && sample >= -1.0);
            i += 1;
        }
        assert_eq!(i, samples.len());
    }
}
