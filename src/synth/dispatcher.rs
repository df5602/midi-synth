use std::sync::mpsc::{Receiver, Sender};
use std::f32;

use usb_midi::{ControlChange, MidiMessage, NoteOn};
use midi_controller::MidiControllerType;
use synth::audio_driver::SAMPLE_RATE;

use errors::Result;

const COLOR_UNSELECTED: u8 = 38;
const COLOR_SELECTED: u8 = 124;

#[derive(Debug, PartialEq)]
pub enum SynthControl {
    MasterTune(f32),
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
    master_tune: u8,
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
            master_tune: 64,
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
                            0x31 => self.update_master_tune(control_change.control_value())?,
                            _ => {}
                        }
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }

        Ok(())
    }

    fn initialize(&mut self) -> Result<()> {
        // Master Tune
        // Set knob to single style
        self.controls_tx.send(ControlChange::create(0, 0x39, 1))?;

        // Set knob to position 0
        self.controls_tx.send(ControlChange::create(0, 0x31, 64))?;

        // Set master tune to 0
        self.synth_ctrl_tx.send(SynthControl::MasterTune(1.0))?;
        self.master_tune = 64;

        // Oscillator 1
        // Set knob to single style
        self.controls_tx.send(ControlChange::create(0, 0x38, 1))?;

        // Set knob to 8' position
        self.controls_tx.send(ControlChange::create(0, 0x30, 72))?;

        // Set range of oscillator 1 to 8' (440 Hz)
        self.osc1_range = OscillatorRange::Range8ft;
        self.synth_ctrl_tx.send(SynthControl::Oscillator1Range(
            (f64::from(&self.osc1_range) / SAMPLE_RATE) as f32,
        ))?;

        // Set LEDs of unselected waveforms to unselected (except first one)
        self.controls_tx
            .send(NoteOn::create(0, 0, COLOR_UNSELECTED))?;
        self.controls_tx
            .send(NoteOn::create(0, 8, COLOR_UNSELECTED))?;
        self.controls_tx
            .send(NoteOn::create(0, 16, COLOR_UNSELECTED))?;
        self.controls_tx
            .send(NoteOn::create(0, 24, COLOR_UNSELECTED))?;
        self.controls_tx
            .send(NoteOn::create(0, 32, COLOR_SELECTED))?;
        self.controls_tx
            .send(NoteOn::create(0, 33, COLOR_UNSELECTED))?;

        // Set oscillator 1 to on
        self.controls_tx.send(NoteOn::create(0, 0x33, 127))?;

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

    fn update_master_tune(&mut self, value: u8) -> Result<()> {
        if value != self.master_tune {
            let tune = (f32::from(value) - 64.0) * 5.0 / 128.0;

            self.synth_ctrl_tx
                .send(SynthControl::MasterTune(2.0_f32.powf(tune / 12.0)))?;

            self.controls_tx
                .send(ControlChange::create(0, 0x31, value))?;

            self.master_tune = value;
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

    macro_rules! get_resp {
        ($rx:ident) => {{
            $rx.recv_timeout(Duration::from_millis(100)).unwrap()
        }};
    }

    macro_rules! expect_no_resp {
        ($rx:ident) => {
            assert!($rx.recv_timeout(Duration::from_millis(100)).is_err());
        };
    }

    macro_rules! assert_float_eq {
        ($left:expr, $right:expr, $eps:expr) => {{
            let left = $left;
            let right = $right;
            assert!((left - right).abs() < $eps, "Expected: {}, got: {}", left, right);
        }};
    }

    #[test]
    fn master_tune() {
        macro_rules! send_and_check {
            ($tx:ident, $rx_midi:ident, $val:expr, $rx_synth:ident, $tune:expr, $eps:expr) => {
                send_cmd!($tx, ControlChange::create(0, 0x31, $val), MidiControllerType::ControlPanel);
                expect_resp!($rx_midi, ControlChange::create(0, 0x31, $val));

                let tune = get_resp!($rx_synth);
                let tune = match tune {
                    SynthControl::MasterTune(tune) => tune,
                    _ => panic!("wrong variant!"),
                };
                assert_float_eq!($tune, tune, $eps);
            };
        }

        let (midi_cmd_tx, midi_rsp_rx, synth_ctrl_rx) = setup_dispatcher!();

        send_and_check!(midi_cmd_tx, midi_rsp_rx, 0, synth_ctrl_rx, 0.865537, 1e-6);
        send_and_check!(midi_cmd_tx, midi_rsp_rx, 32, synth_ctrl_rx, 0.930342, 1e-6);
        send_and_check!(midi_cmd_tx, midi_rsp_rx, 64, synth_ctrl_rx, 1.0, 1e-6);
        send_and_check!(midi_cmd_tx, midi_rsp_rx, 96, synth_ctrl_rx, 1.074873, 1e-6);
        send_and_check!(midi_cmd_tx, midi_rsp_rx, 127, synth_ctrl_rx, 1.152749, 1e-6);
    }

    #[test]
    fn master_tune_update_only_when_different() {
        let (midi_cmd_tx, midi_resp_rx, synth_ctrl_rx) = setup_dispatcher!();

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x31, 32),
            MidiControllerType::ControlPanel
        );

        expect_resp!(midi_resp_rx, ControlChange::create(0, 0x31, 32));

        let tune = get_resp!(synth_ctrl_rx);
        let tune = match tune {
            SynthControl::MasterTune(tune) => tune,
            _ => panic!("wrong variant!"),
        };
        assert_float_eq!(tune, 0.930342, 1e-6);

        send_cmd!(
            midi_cmd_tx,
            ControlChange::create(0, 0x31, 32),
            MidiControllerType::ControlPanel
        );

        expect_no_resp!(midi_resp_rx);
        expect_no_resp!(synth_ctrl_rx);
    }

    #[test]
    fn oscillator1_range_valid_positions() {
        macro_rules! send_and_check {
            ($tx:ident, $tx_val:expr, $rx_midi:ident, $rx_val:expr, $rx_synth:ident, $range:expr) => {
                send_cmd!($tx, ControlChange::create(0, 0x30, $tx_val), MidiControllerType::ControlPanel);
                expect_resp!($rx_midi, ControlChange::create(0, 0x30, $rx_val));
                expect_resp!(
                    $rx_synth,
                    SynthControl::Oscillator1Range((f64::from($range) / SAMPLE_RATE) as f32));
            };
        }

        let (cmd, rsp, synth_rx) = setup_dispatcher!();

        send_and_check!(cmd, 20, rsp, 21, synth_rx, &OscillatorRange::Low);
        send_and_check!(cmd, 54, rsp, 54, synth_rx, &OscillatorRange::Range16ft);
        send_and_check!(cmd, 72, rsp, 72, synth_rx, &OscillatorRange::Range8ft);
        send_and_check!(cmd, 90, rsp, 90, synth_rx, &OscillatorRange::Range4ft);
        send_and_check!(cmd, 110, rsp, 105, synth_rx, &OscillatorRange::Range2ft);
    }

    #[test]
    fn oscillator1_range_invalid_positions() {
        macro_rules! send_and_check_no_resp {
            ($tx:ident, $val:expr, $rx_midi:ident, $rx_synth:ident) => {
                send_cmd!($tx, ControlChange::create(0, 0x30, $val), MidiControllerType::ControlPanel);
                expect_no_resp!($rx_midi);
                expect_no_resp!($rx_synth);
            };
        }

        let (midi_cmd_tx, midi_resp_rx, synth_ctrl_rx) = setup_dispatcher!();

        send_and_check_no_resp!(midi_cmd_tx, 30, midi_resp_rx, synth_ctrl_rx);
        send_and_check_no_resp!(midi_cmd_tx, 40, midi_resp_rx, synth_ctrl_rx);
        send_and_check_no_resp!(midi_cmd_tx, 60, midi_resp_rx, synth_ctrl_rx);
        send_and_check_no_resp!(midi_cmd_tx, 80, midi_resp_rx, synth_ctrl_rx);
        send_and_check_no_resp!(midi_cmd_tx, 100, midi_resp_rx, synth_ctrl_rx);
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
