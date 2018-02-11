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

use midi_controller::{AkaiAPC40MkII, MAudioKeystation49e, UsbMidiController};

use synth::dispatcher::Dispatcher;
use synth::audio_driver::AudioDriver;

use errors::*;
use error_chain::ChainedError;

lazy_static! {
    static ref USB_CONTEXT: libusb::Context = match libusb::Context::new() {
        Ok(context) => context,
        Err(e) => panic!("Failed to initialize libusb: {}", e)
    };
}

fn run() -> Result<()> {
    let mut threads = vec![];

    let (keyboard2host_tx, keyboard2host_rx) = mpsc::channel();
    let (controls2host_tx, controls2host_rx) = mpsc::channel();
    let (host2controls_tx, host2controls_rx) = mpsc::channel();
    let (synth_ctrl_tx, synth_ctrl_rx) = mpsc::channel();

    let keyboard = false;

    // Setup MIDI controllers
    let keystation = if keyboard {
        Some(UsbMidiController::new(MAudioKeystation49e::open(
            &USB_CONTEXT,
        )?))
    } else {
        None
    };
    let apc40 = Arc::new(UsbMidiController::new(AkaiAPC40MkII::open(&USB_CONTEXT)?));

    // Setup Portaudio
    let mut audio = AudioDriver::new()?;
    audio.start(synth_ctrl_rx)?;

    // Setup threads that listen to MIDI events from the controllers
    if keystation.is_some() {
        let keystation = keystation.unwrap();
        let keyboard_thread = thread::spawn(move || keystation.listen(&keyboard2host_tx));
        threads.push(keyboard_thread);
    }

    let apc40_cloned = apc40.clone();
    let controls_rx_thread = thread::spawn(move || apc40_cloned.listen(&controls2host_tx));
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
    let mut dispatcher = Dispatcher::new(
        keyboard2host_rx,
        controls2host_rx,
        host2controls_tx,
        synth_ctrl_tx,
    );
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
