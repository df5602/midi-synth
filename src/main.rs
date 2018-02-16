extern crate itertools;
extern crate libusb;
extern crate portaudio;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

mod errors;
mod midi_controller;
mod usb_midi;
mod synth;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use midi_controller::{AkaiAPC40MkII, MAudioKeystation49e, MidiControllerType, UsbMidiController};

use synth::dispatcher::Dispatcher;
use synth::audio_driver::AudioDriver;
use synth::synthesizer::Synthesizer;

use errors::*;
use errors::ErrorKind::*;
use error_chain::ChainedError;

lazy_static! {
    static ref USB_CONTEXT: libusb::Context = match libusb::Context::new() {
        Ok(context) => context,
        Err(e) => panic!("Failed to initialize libusb: {}", e)
    };
}

fn run() -> Result<()> {
    let mut threads = vec![];

    let (device2host_tx, device2host_rx) = mpsc::channel();
    let (host2controls_tx, host2controls_rx) = mpsc::channel();
    let (synth_ctrl_tx, synth_ctrl_rx) = mpsc::channel();

    // Setup MIDI controllers
    let keystation = match MAudioKeystation49e::open(&USB_CONTEXT) {
        Ok(keystation) => Some(UsbMidiController::new(keystation)),
        Err(e) => match *e.kind() {
            MidiControllerNotConnected => {
                println!("Keyboard not connected, continue in continuous mode...");
                None
            }
            _ => return Err(e).chain_err(|| "Could not open M-Audio Keystation 49e"),
        },
    };

    let apc40 = Arc::new(UsbMidiController::new(AkaiAPC40MkII::open(&USB_CONTEXT)
        .chain_err(|| "Could not open Akai APC40 MkII")?));

    // Create Synthesizer
    let synthesizer = Synthesizer::new(synth_ctrl_rx);

    // Setup Portaudio
    let mut audio = AudioDriver::new()?;
    audio.start(synthesizer)?;

    // Setup threads that listen to MIDI events from the controllers
    if let Some(keystation) = keystation {
        let keyboard_tx = device2host_tx.clone();
        let keyboard_thread =
            thread::spawn(move || keystation.listen(&keyboard_tx, MidiControllerType::Keyboard));
        threads.push(keyboard_thread);
    }

    let apc40_cloned = apc40.clone();
    let controls_rx_thread = thread::spawn(move || {
        apc40_cloned.listen(&device2host_tx, MidiControllerType::ControlPanel)
    });
    threads.push(controls_rx_thread);

    // Setup thread that transmits MIDI events to APC controller
    let controls_tx_thread = thread::spawn(move || {
        while let Ok(midi_message) = host2controls_rx.recv() {
            match apc40.send_message(midi_message) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    });
    threads.push(controls_tx_thread);

    // Create dispatcher
    let mut dispatcher = Dispatcher::new(device2host_rx, host2controls_tx, synth_ctrl_tx);
    let dispatcher_thread = thread::spawn(move || dispatcher.start());
    threads.push(dispatcher_thread);

    for thread in threads {
        match thread.join() {
            Ok(res) => res?,
            Err(e) => eprintln!("{:?}", e),
        }
    }

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        eprintln!("{}", e.display_chain().to_string());

        ::std::process::exit(1);
    }
}
