use portaudio;
use portaudio::{OutputStreamCallbackArgs, PortAudio, Stream};

use synth::synthesizer::Synthesizer;

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

    pub fn start(&mut self, mut synthesizer: Synthesizer) -> Result<()> {
        let mut settings = self.portaudio.default_output_stream_settings(
            CHANNELS,
            SAMPLE_RATE,
            FRAMES_PER_BUFFER,
        )?;
        settings.flags = portaudio::stream_flags::CLIP_OFF;

        let mut stream = self.portaudio.open_non_blocking_stream(
            settings,
            move |OutputStreamCallbackArgs { buffer, frames, .. }| {
                let mut idx = 0;

                for _ in 0..frames {
                    let output_value = synthesizer.next().unwrap();

                    buffer[idx] = output_value;
                    buffer[idx + 1] = output_value;

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
