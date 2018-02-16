use std::sync::mpsc::{Receiver, Sender};

use usb_midi::{ControlChange, MidiMessage};
use midi_controller::MidiControllerType;
use synth::audio_driver::SAMPLE_RATE;

use errors::Result;

pub struct Dispatcher {
    controls_rx: Receiver<(MidiMessage, MidiControllerType)>,
    controls_tx: Sender<MidiMessage>,
    synth_ctrl_tx: Sender<f32>,
}

impl Dispatcher {
    pub fn new(
        controls_rx: Receiver<(MidiMessage, MidiControllerType)>,
        controls_tx: Sender<MidiMessage>,
        synth_ctrl_tx: Sender<f32>,
    ) -> Dispatcher {
        Dispatcher {
            controls_rx: controls_rx,
            controls_tx: controls_tx,
            synth_ctrl_tx: synth_ctrl_tx,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        // Set track knob to "single style" and initialize with 8' position
        let message = ControlChange::create(0, 0x38, 1);
        self.controls_tx.send(message)?;

        let message = ControlChange::create(0, 0x30, 72);
        self.controls_tx.send(message)?;

        self.synth_ctrl_tx.send(440.0 / 44100.0)?;

        // Receive MIDI events from controller
        while let Ok((midi_message, source)) = self.controls_rx.recv() {
            match (source, midi_message) {
                (MidiControllerType::ControlPanel, midi_message) => match midi_message {
                    MidiMessage::ControlChange(control_change) => {
                        if control_change.control_number() == 0x30 {
                            let (value, f) = match control_change.control_value() {
                                0...21 => (21, 13.75 / SAMPLE_RATE),
                                val @ 35...38 => (val, 110.0 / SAMPLE_RATE),
                                val @ 53...56 => (val, 220.0 / SAMPLE_RATE),
                                val @ 70...73 => (val, 440.0 / SAMPLE_RATE),
                                val @ 88...91 => (val, 880.0 / SAMPLE_RATE),
                                105...127 => (105, 1760.0 / SAMPLE_RATE),
                                _ => continue,
                            };

                            self.synth_ctrl_tx.send(f as f32)?;

                            let response = ControlChange::create(0, 0x30, value);
                            self.controls_tx.send(response)?;
                        }
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }

        Ok(())
    }
}
