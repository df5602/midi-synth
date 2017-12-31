extern crate itertools;
extern crate libusb;

#[macro_use]
extern crate error_chain;

mod errors;
mod usb_midi;

use itertools::Itertools;

use std::time::Duration;

use errors::*;
use error_chain::ChainedError;

use usb_midi::{MidiParseStatus, UsbMidiParser};

#[allow(dead_code)]
fn describe_device(device: &libusb::Device) -> Result<()> {
    let config_desc = device.active_config_descriptor()?;

    println!("Found keyboard:");
    println!("Active Configuration: {}", config_desc.number());
    println!("Number of Interfaces: {}", config_desc.num_interfaces());

    for interface in config_desc.interfaces() {
        println!();
        println!("Interface {}", interface.number());

        for interface_desc in interface.descriptors() {
            println!();
            println!("Number of endpoints: {}", interface_desc.num_endpoints());

            for endpoint in interface_desc.endpoint_descriptors() {
                println!();
                println!("Endpoint {}", endpoint.number());
                println!("Address {}", endpoint.address());
                println!("Direction: {:?}", endpoint.direction());
                println!("Transfer Type: {:?}", endpoint.transfer_type());
                println!("Sync Type: {:?}", endpoint.sync_type());
                println!("Usage Type: {:?}", endpoint.usage_type());
            }
        }
    }

    Ok(())
}

fn run() -> Result<()> {
    let context = libusb::Context::new().chain_err(|| "Failed to initialize libusb")?;

    let mut handle = None;
    for device in context
        .devices()
        .chain_err(|| "Failed to list USB devices")?
        .iter()
    {
        let device_desc = device.device_descriptor()?;

        if device_desc.vendor_id() == 0xa4d && device_desc.product_id() == 0x90 {
            handle = Some(device.open().chain_err(|| "Failed to open USB device")?);
            break;
        }
    }

    let mut handle = match handle {
        Some(handle) => handle,
        None => {
            println!("No keyboard found.");
            return Ok(());
        }
    };

    handle
        .claim_interface(1)
        .chain_err(|| "Failed to claim interface 1")?;

    let mut buf: [u8; 256] = [0; 256];
    let mut usb_midi_parser = UsbMidiParser::new();

    let mut begin = 0;
    let mut end = 0;
    loop {
        let read = handle
            .read_bulk(129, &mut buf[end..], Duration::from_secs(60))
            .chain_err(|| "Failed to read from USB device")?;
        end += read;

        println!("Read {} bytes:", read);
        println!("0x{:02x}", buf[begin..end].iter().format(""));

        while begin < end {
            match usb_midi_parser.parse(&buf[begin..end]) {
                (MidiParseStatus::Complete(packet), n) => {
                    println!("{}", packet.midi_message());
                    begin += n;
                }
                (MidiParseStatus::Incomplete, n) => {
                    begin += n;
                    break;
                }
                (MidiParseStatus::Unknown, n) => {
                    println!("Unknown MIDI message");
                    begin += n;
                }
                (MidiParseStatus::MalformedPacket, n) => {
                    println!("Malformed packet");
                    begin += n;
                }
            }
        }

        if begin >= end {
            begin = 0;
            end = 0;
        }
    }
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}", e.display_chain().to_string());

        ::std::process::exit(1);
    }
}
