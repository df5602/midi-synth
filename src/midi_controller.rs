use std::time::Duration;

use libusb;
use itertools::Itertools;

use errors::*;
use errors::ErrorKind::MidiControllerNotConnected;

use usb_midi::{MidiParseStatus, UsbMidiParser};

/// Utility function to discover all endpoints of a USB device.
#[allow(dead_code)]
pub fn describe_device(device: &libusb::Device) -> Result<()> {
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

fn open_device(
    context: &libusb::Context,
    vendor_id: u16,
    product_id: u16,
    interface: u8,
) -> Result<libusb::DeviceHandle> {
    let mut handle = None;
    for device in context
        .devices()
        .chain_err(|| "Failed to list USB devices")?
        .iter()
    {
        let device_desc = device.device_descriptor()?;

        if device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id {
            handle = Some(device.open().chain_err(|| "Failed to open USB device")?);
            break;
        }
    }

    let mut handle = match handle {
        Some(handle) => handle,
        None => return Err(MidiControllerNotConnected.into()),
    };

    handle
        .claim_interface(interface)
        .chain_err(|| format!("Failed to claim interface {}", interface))?;

    Ok(handle)
}

pub trait UsbMidiController {
    fn listen(&self) -> Result<()> {
        let mut buf: [u8; 256] = [0; 256];
        let mut usb_midi_parser = UsbMidiParser::new();

        let mut begin = 0;
        let mut end = 0;
        loop {
            let read = self.read_bulk(&mut buf[end..], Duration::from_secs(60))
                .chain_err(|| "Failed to read from USB device")?;
            end += read;

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
                        println!(
                            "Unknown MIDI message: 0x{:02x}",
                            buf[begin..end].iter().format(" 0x")
                        );
                        begin += n;
                    }
                    (MidiParseStatus::MalformedPacket, n) => {
                        println!(
                            "Malformed packet: 0x{:02x}",
                            buf[begin..end].iter().format(" 0x")
                        );
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

    fn read_bulk(&self, buf: &mut [u8], timeout: Duration) -> Result<usize>;
}

pub struct MAudioKeystation49e<'ctx> {
    device_handle: libusb::DeviceHandle<'ctx>,
}

impl<'ctx> MAudioKeystation49e<'ctx> {
    pub fn open(context: &'ctx libusb::Context) -> Result<Self> {
        Ok(Self {
            device_handle: open_device(context, 0xa4d, 0x90, 1)
                .chain_err(|| "Failed to open M-Audio Keystation 49e")?,
        })
    }
}

impl<'ctx> UsbMidiController for MAudioKeystation49e<'ctx> {
    fn read_bulk(&self, buf: &mut [u8], timeout: Duration) -> Result<usize> {
        Ok(self.device_handle.read_bulk(129, buf, timeout)?)
    }
}

pub struct AkaiAPC40MkII<'ctx> {
    device_handle: libusb::DeviceHandle<'ctx>,
}

impl<'ctx> AkaiAPC40MkII<'ctx> {
    pub fn open(context: &'ctx libusb::Context) -> Result<Self> {
        let handle =
            open_device(context, 0x9e8, 0x29, 1).chain_err(|| "Failed to open Akai APC40 MkII")?;

        let buf = [
            0x04, 0xF0, 0x47, 0x7F, 0x04, 0x29, 0x60, 0x00, 0x04, 0x04, 0x42, 0x00, 0x07, 0x00,
            0x00, 0xF7,
        ];
        handle.write_bulk(1, &buf, Duration::from_secs(5))?;

        Ok(Self {
            device_handle: handle,
        })
    }
}

impl<'ctx> UsbMidiController for AkaiAPC40MkII<'ctx> {
    fn read_bulk(&self, buf: &mut [u8], timeout: Duration) -> Result<usize> {
        Ok(self.device_handle.read_bulk(130, buf, timeout)?)
    }
}
