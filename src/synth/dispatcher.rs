use std::sync::mpsc::{Receiver, Sender};

use usb_midi::{ControlChange, MidiMessage};

use errors::Result;

pub struct Dispatcher {
    keyboard_rx: Receiver<MidiMessage>,
    controls_rx: Receiver<MidiMessage>,
    controls_tx: Sender<MidiMessage>,
    synth_ctrl_tx: Sender<f32>,
}

impl Dispatcher {
    pub fn new(
        keyboard_rx: Receiver<MidiMessage>,
        controls_rx: Receiver<MidiMessage>,
        controls_tx: Sender<MidiMessage>,
        synth_ctrl_tx: Sender<f32>,
    ) -> Dispatcher {
        Dispatcher {
            keyboard_rx: keyboard_rx,
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

        // Receive MIDI events from controller
        while let Ok(midi_message) = self.controls_rx.recv() {
            match midi_message {
                MidiMessage::ControlChange(control_change) => {
                    if control_change.control_number() == 0x30 {
                        let (value, f) = match control_change.control_value() {
                            0...21 => (21, 0.005),
                            val @ 35...38 => (val, 0.01),
                            val @ 53...56 => (val, 0.02),
                            val @ 70...73 => (val, 0.04),
                            val @ 88...91 => (val, 0.08),
                            105...127 => (105, 0.16),
                            _ => continue,
                        };

                        self.synth_ctrl_tx.send(f)?;

                        let response = ControlChange::create(0, 0x30, value);
                        self.controls_tx.send(response)?;
                    }
                }
                _ => continue,
            }
        }

        Ok(())
    }
}
