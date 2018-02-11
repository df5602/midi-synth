extern crate itertools;
extern crate libusb;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

mod errors;
mod midi_controller;
mod usb_midi;

use std::sync::mpsc;
use std::thread;

use midi_controller::{AkaiAPC40MkII, MAudioKeystation49e, UsbMidiController};
use usb_midi::MidiMessage;

use errors::*;
use error_chain::ChainedError;

lazy_static! {
    static ref USB_CONTEXT: libusb::Context = match libusb::Context::new() {
        Ok(context) => context,
        Err(e) => panic!("Failed to initialize libusb: {}", e)
    };
}

fn run() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let keystation = UsbMidiController::new(MAudioKeystation49e::open(&USB_CONTEXT)?);
    let keyboard_thread = thread::spawn(move || keystation.listen(&tx));

    let apc40 = UsbMidiController::new(AkaiAPC40MkII::open(&USB_CONTEXT)?);

    while let Ok(midi_message) = rx.recv() {
        let midi_message = match midi_message {
            note_on @ MidiMessage::NoteOn(_) => note_on,
            note_off @ MidiMessage::NoteOff(_) => note_off,
            _ => continue,
        };

        apc40.send_message(midi_message)?;
    }

    let res = keyboard_thread.join();
    match res {
        Ok(res) => res?,
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        eprintln!("{}", e.display_chain().to_string());

        ::std::process::exit(1);
    }
}
