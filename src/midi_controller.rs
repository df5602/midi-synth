use std::time::Duration;
use std::sync::mpsc::Sender;
use std::sync::atomic::Ordering;

use libusb;
use itertools::Itertools;

use errors::*;
use errors::ErrorKind::{MidiControllerNotConnected, MidiOperationNotSupported};

use usb_midi::{MidiMessage, MidiParseStatus, SystemExclusive, SystemExlusiveId, UsbMidiParser};

#[derive(Debug, Copy, Clone)]
pub enum MidiControllerType {
    Keyboard,
    ControlPanel,
}

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

pub trait UsbMidiDevice {
    fn read_bulk(&self, buf: &mut [u8], timeout: Duration) -> Result<usize>;
    fn write_bulk(&self, buf: &[u8], timeout: Duration) -> Result<usize>;
}

pub struct UsbMidiController<T: UsbMidiDevice> {
    device: T,
}

impl<T: UsbMidiDevice> UsbMidiController<T> {
    pub fn new(device: T) -> UsbMidiController<T> {
        UsbMidiController { device }
    }

    pub fn listen(
        &self,
        tx: &Sender<(MidiMessage, MidiControllerType)>,
        source: MidiControllerType,
    ) -> Result<()> {
        let mut buf: [u8; 256] = [0; 256];
        let mut usb_midi_parser = UsbMidiParser::new();

        let mut begin = 0;
        let mut end = 0;
        loop {
            if ::TERMINATION_REQUEST.load(Ordering::Acquire) {
                return Ok(());
            }

            let read = match self.device
                .read_bulk(&mut buf[end..], Duration::from_millis(100))
            {
                Ok(read) => read,
                Err(e) => {
                    match *e.kind() {
                        ErrorKind::Usb(::libusb::Error::Timeout) => continue,
                        _ => 0, // Hack: return value that typechecks, so that we reach return statement
                                // and can properly return the error (will possibly be fixed with NLL)
                    };
                    return Err(e.chain_err(|| "Failed to read from USB device"));
                }
            };
            end += read;

            while begin < end {
                match usb_midi_parser.parse(&buf[begin..end]) {
                    (MidiParseStatus::Complete(packet), n) => {
                        tx.send((packet.into_midi_message(), source))?;
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

    pub fn send_message(&self, msg: MidiMessage) -> Result<usize> {
        let mut buf = [0u8; 32];

        let mut sent = 0;

        let mut i = 0;
        for byte in msg.serialize() {
            buf[i] = byte;
            i += 1;

            if i == buf.len() {
                sent += self.device.write_bulk(&buf, Duration::from_secs(5))?;
                i = 0;
            }
        }

        if i > 0 {
            sent += self.device.write_bulk(&buf[..i], Duration::from_secs(5))?;
        }

        Ok(sent)
    }
}

pub struct MAudioKeystation49e<'ctx> {
    device_handle: libusb::DeviceHandle<'ctx>,
}

impl<'ctx> MAudioKeystation49e<'ctx> {
    pub fn open(context: &'ctx libusb::Context) -> Result<Self> {
        Ok(Self {
            device_handle: open_device(context, 0xa4d, 0x90, 1)?,
        })
    }
}

impl<'ctx> UsbMidiDevice for MAudioKeystation49e<'ctx> {
    fn read_bulk(&self, buf: &mut [u8], timeout: Duration) -> Result<usize> {
        Ok(self.device_handle.read_bulk(129, buf, timeout)?)
    }

    fn write_bulk(&self, _buf: &[u8], _timeout: Duration) -> Result<usize> {
        Err(MidiOperationNotSupported.into())
    }
}

pub struct AkaiAPC40MkII<'ctx> {
    device_handle: libusb::DeviceHandle<'ctx>,
}

impl<'ctx> AkaiAPC40MkII<'ctx> {
    pub fn open(context: &'ctx libusb::Context) -> Result<Self> {
        let handle = open_device(context, 0x9e8, 0x29, 1)?;

        let init_message = SystemExclusive::create(
            SystemExlusiveId::OneByte(0x47),
            vec![0x7F, 0x29, 0x60, 0x00, 0x04, 0x42, 0x00, 0x00, 0x00],
        );

        let mut buf = [0u8; 16];
        for (i, byte) in init_message.serialize().enumerate() {
            buf[i] = byte;
        }
        handle.write_bulk(1, &buf, Duration::from_secs(5))?;

        Ok(Self {
            device_handle: handle,
        })
    }
}

impl<'ctx> UsbMidiDevice for AkaiAPC40MkII<'ctx> {
    fn read_bulk(&self, buf: &mut [u8], timeout: Duration) -> Result<usize> {
        Ok(self.device_handle.read_bulk(130, buf, timeout)?)
    }

    fn write_bulk(&self, buf: &[u8], timeout: Duration) -> Result<usize> {
        Ok(self.device_handle.write_bulk(1, buf, timeout)?)
    }
}
