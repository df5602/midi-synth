use std::sync::mpsc::Receiver;

use portaudio;
use portaudio::{OutputStreamCallbackArgs, PortAudio, Stream};

use errors::Result;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

pub struct AudioDriver {
    portaudio: PortAudio,
    stream: Option<Stream<portaudio::NonBlocking, portaudio::Output<f32>>>,
}

impl AudioDriver {
    pub fn new() -> Result<AudioDriver> {
        let portaudio = PortAudio::new()?;

        Ok(AudioDriver {
            portaudio: portaudio,
            stream: None,
        })
    }

    pub fn start(&mut self, freq_in: Receiver<f32>) -> Result<()> {
        let mut settings = self.portaudio.default_output_stream_settings(
            CHANNELS,
            SAMPLE_RATE,
            FRAMES_PER_BUFFER,
        )?;
        settings.flags = portaudio::stream_flags::CLIP_OFF;

        let mut output_value = 0.0;
        let mut output_direction = 1.0;
        let mut freq = 0.04; // approx. 440 Hz

        let mut stream = self.portaudio.open_non_blocking_stream(
            settings,
            move |OutputStreamCallbackArgs { buffer, frames, .. }| {
                let mut idx = 0;

                if let Ok(f) = freq_in.try_recv() {
                    freq = f;
                };

                for _ in 0..frames {
                    buffer[idx] = output_value;
                    buffer[idx + 1] = output_value;

                    output_value += freq * output_direction;
                    if output_value >= 1.0 || output_value <= -1.0 {
                        output_direction *= -1.0;
                        let diff = if output_value >= 1.0 {
                            output_value - 1.0
                        } else {
                            -1.0 - output_value
                        };
                        output_value += 2.0 * diff * output_direction;
                        assert!(
                            output_value < 1.0 && output_value > -1.0,
                            "{}",
                            output_value,
                        );
                    }
                    idx += 2;
                }
                portaudio::Continue
            },
        )?;

        stream.start()?;
        self.stream = Some(stream);

        Ok(())
    }
}

impl Drop for AudioDriver {
    fn drop(&mut self) {
        if let Some(ref mut stream) = self.stream {
            let _ = stream.stop();
            let _ = stream.close();
        }
    }
}
