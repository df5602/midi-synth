extern crate itertools;
extern crate libusb;

#[macro_use]
extern crate error_chain;

mod errors;
mod midi_controller;
mod usb_midi;

use midi_controller::{AkaiAPC40MkII, MAudioKeystation49e, UsbMidiController};

use errors::*;
use error_chain::ChainedError;

fn run() -> Result<()> {
    let context = libusb::Context::new().chain_err(|| "Failed to initialize libusb")?;

    //let keystation = MAudioKeystation49e::open(&context)?;
    //keystation.listen()?;

    let apc40 = AkaiAPC40MkII::open(&context)?;
    apc40.listen()?;

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}", e.display_chain().to_string());

        ::std::process::exit(1);
    }
}
