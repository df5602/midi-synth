use std::sync::mpsc::{Receiver, Sender};

use usb_midi::{ControlChange, MidiMessage};
use midi_controller::MidiControllerType;
use synth::audio_driver::SAMPLE_RATE;

use errors::Result;

#[derive(Debug, PartialEq)]
pub enum SynthControl {
    Oscillator1Range(f32),
}

#[derive(PartialEq)]
enum OscillatorRange {
    Low,
    Range32ft,
    Range16ft,
    Range8ft,
    Range4ft,
    Range2ft,
}

impl<'a> From<&'a OscillatorRange> for f64 {
    fn from(range: &OscillatorRange) -> f64 {
        let a = 440.0;
        // c'' (one octave above c', i.e. middle C) is 300 cents above a'
        let middle_c = 0.5 * a * 2.0_f64.powf(3.0 / 12.0);
        match *range {
            OscillatorRange::Low => 0.0625 * middle_c,
            OscillatorRange::Range32ft => 0.25 * middle_c,
            OscillatorRange::Range16ft => 0.5 * middle_c,
            OscillatorRange::Range8ft => middle_c,
            OscillatorRange::Range4ft => 2.0 * middle_c,
            OscillatorRange::Range2ft => 4.0 * middle_c,
        }
    }
}

pub struct Dispatcher {
    controls_rx: Receiver<(MidiMessage, MidiControllerType)>,
    controls_tx: Sender<MidiMessage>,
    synth_ctrl_tx: Sender<SynthControl>,
    osc1_range: OscillatorRange,
}

impl Dispatcher {
    pub fn new(
        controls_rx: Receiver<(MidiMessage, MidiControllerType)>,
        controls_tx: Sender<MidiMessage>,
        synth_ctrl_tx: Sender<SynthControl>,
    ) -> Dispatcher {
        Dispatcher {
            controls_rx: controls_rx,
            controls_tx: controls_tx,
            synth_ctrl_tx: synth_ctrl_tx,
            osc1_range: OscillatorRange::Range8ft,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.initialize()?;

        // Receive MIDI events from controller
        while let Ok((midi_message, source)) = self.controls_rx.recv() {
            match (source, midi_message) {
                (MidiControllerType::ControlPanel, midi_message) => match midi_message {
                    MidiMessage::ControlChange(control_change) => {
                        match control_change.control_number() {
                            0x30 => self.update_oscillator_range(control_change.control_value())?,
                            _ => {}
                        }
                    }
                    _ => {}
                },
                (MidiControllerType::Keyboard, _midi_message) => {}
            }
        }

        Ok(())
    }

    fn initialize(&mut self) -> Result<()> {
        // Oscillator 1
        // Set knob to single style
        let message = ControlChange::create(0, 0x38, 1);
        self.controls_tx.send(message)?;

        // Set knob to 8' position
        let message = ControlChange::create(0, 0x30, 72);
        self.controls_tx.send(message)?;

        // Set range of oscillator 1 to 8' (440 Hz)
        self.osc1_range = OscillatorRange::Range8ft;
        self.synth_ctrl_tx.send(SynthControl::Oscillator1Range(
            (f64::from(&self.osc1_range) / SAMPLE_RATE) as f32,
        ))?;

        Ok(())
    }

    fn update_oscillator_range(&mut self, value: u8) -> Result<()> {
        let (value, range) = match value {
            0...21 => (21, OscillatorRange::Low),
            val @ 35...38 => (val, OscillatorRange::Range32ft),
            val @ 53...56 => (val, OscillatorRange::Range16ft),
            val @ 70...73 => (val, OscillatorRange::Range8ft),
            val @ 88...91 => (val, OscillatorRange::Range4ft),
            105...127 => (105, OscillatorRange::Range2ft),
            _ => return Ok(()),
        };

        if range != self.osc1_range {
            self.synth_ctrl_tx.send(SynthControl::Oscillator1Range(
                (f64::from(&range) / SAMPLE_RATE) as f32,
            ))?;

            self.controls_tx
                .send(ControlChange::create(0, 0x30, value))?;

            self.osc1_range = range;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    macro_rules! setup_dispatcher {
        () => {{
            let (midi_cmd_tx, midi_cmd_rx) = mpsc::channel();
            let (midi_resp_tx, midi_resp_rx) = mpsc::channel();
            let (synth_ctrl_tx, synth_ctrl_rx) = mpsc::channel();

            let mut dispatcher = Dispatcher::new(midi_cmd_rx, midi_resp_tx, synth_ctrl_tx);
            let _dispatcher_thread = thread::spawn(move || dispatcher.start());

            // Clear initialization messages
            while let Ok(_) = midi_resp_rx.recv_timeout(Duration::from_millis(100)) {}
            while let Ok(_) = synth_ctrl_rx.recv_timeout(Duration::from_millis(100)) {}

            (midi_cmd_tx, midi_resp_rx, synth_ctrl_rx)
        }};
    }

    macro_rules! send_cmd {
        ($tx:ident, $cmd:expr, $src:expr) => {
            $tx.send(($cmd, $src)).unwrap();
        };
    }

    macro_rules! expect_resp {
        ($rx:ident, $resp:expr) => {
            assert_eq!($rx.recv_timeout(Duration::from_millis(100)).unwrap(), $resp);
        };
    }

    macro_rules! expect_no_resp {
        ($rx:ident) => {
            assert!($rx.recv_timeout(Duration::from_millis(100)).is_err());
        };
    }

    #[test]
    fn oscillator1_range_valid_positions() {
        let (midi_cmd_tx, midi_resp_rx, synth_ctrl_rx) = setup_dispatcher!();

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 20),
            MidiControllerType::ControlPanel
        );
        expect_resp!(midi_resp_rx, ControlChange::create(0, 0x30, 21));
        expect_resp!(
            synth_ctrl_rx,
            SynthControl::Oscillator1Range((f64::from(&OscillatorRange::Low) / SAMPLE_RATE) as f32)
        );

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 36),
            MidiControllerType::ControlPanel
        );
        expect_resp!(midi_resp_rx, ControlChange::create(0, 0x30, 36));
        expect_resp!(
            synth_ctrl_rx,
            SynthControl::Oscillator1Range(
                (f64::from(&OscillatorRange::Range32ft) / SAMPLE_RATE) as f32
            )
        );

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 54),
            MidiControllerType::ControlPanel
        );
        expect_resp!(midi_resp_rx, ControlChange::create(0, 0x30, 54));
        expect_resp!(
            synth_ctrl_rx,
            SynthControl::Oscillator1Range(
                (f64::from(&OscillatorRange::Range16ft) / SAMPLE_RATE) as f32
            )
        );

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 72),
            MidiControllerType::ControlPanel
        );
        expect_resp!(midi_resp_rx, ControlChange::create(0, 0x30, 72));
        expect_resp!(
            synth_ctrl_rx,
            SynthControl::Oscillator1Range(
                (f64::from(&OscillatorRange::Range8ft) / SAMPLE_RATE) as f32
            )
        );

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 90),
            MidiControllerType::ControlPanel
        );
        expect_resp!(midi_resp_rx, ControlChange::create(0, 0x30, 90));
        expect_resp!(
            synth_ctrl_rx,
            SynthControl::Oscillator1Range(
                (f64::from(&OscillatorRange::Range4ft) / SAMPLE_RATE) as f32
            )
        );

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 110),
            MidiControllerType::ControlPanel
        );
        expect_resp!(midi_resp_rx, ControlChange::create(0, 0x30, 105));
        expect_resp!(
            synth_ctrl_rx,
            SynthControl::Oscillator1Range(
                (f64::from(&OscillatorRange::Range2ft) / SAMPLE_RATE) as f32
            )
        );
    }

    #[test]
    fn oscillator1_range_invalid_positions() {
        let (midi_cmd_tx, midi_resp_rx, synth_ctrl_rx) = setup_dispatcher!();

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 30),
            MidiControllerType::ControlPanel
        );
        expect_no_resp!(midi_resp_rx);
        expect_no_resp!(synth_ctrl_rx);

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 40),
            MidiControllerType::ControlPanel
        );
        expect_no_resp!(midi_resp_rx);
        expect_no_resp!(synth_ctrl_rx);

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 60),
            MidiControllerType::ControlPanel
        );
        expect_no_resp!(midi_resp_rx);
        expect_no_resp!(synth_ctrl_rx);

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 80),
            MidiControllerType::ControlPanel
        );
        expect_no_resp!(midi_resp_rx);
        expect_no_resp!(synth_ctrl_rx);

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 100),
            MidiControllerType::ControlPanel
        );
        expect_no_resp!(midi_resp_rx);
        expect_no_resp!(synth_ctrl_rx);
    }

    #[test]
    fn oscillator1_range_update_only_if_different() {
        let (midi_cmd_tx, midi_resp_rx, synth_ctrl_rx) = setup_dispatcher!();

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 54),
            MidiControllerType::ControlPanel
        );
        expect_resp!(midi_resp_rx, ControlChange::create(0, 0x30, 54));
        expect_resp!(
            synth_ctrl_rx,
            SynthControl::Oscillator1Range(
                (f64::from(&OscillatorRange::Range16ft) / SAMPLE_RATE) as f32
            )
        );

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x30, 55),
            MidiControllerType::ControlPanel
        );
        expect_no_resp!(midi_resp_rx);
        expect_no_resp!(synth_ctrl_rx);
    }
}
