use std::fmt;
use std::mem;
use itertools::Itertools;

#[derive(Clone, PartialEq, Debug)]
pub struct NoteOn {
    channel: u8,
    note_number: u8,
    key_velocity: u8,
}

impl NoteOn {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note_number(&self) -> u8 {
        self.note_number
    }

    pub fn key_velocity(&self) -> u8 {
        self.key_velocity
    }
}

impl fmt::Display for NoteOn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Note On, Channel: {}, Note number: {}, Key velocity: {}",
            self.channel(),
            self.note_number(),
            self.key_velocity()
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct NoteOff {
    channel: u8,
    note_number: u8,
    off_velocity: u8,
}

impl NoteOff {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note_number(&self) -> u8 {
        self.note_number
    }

    pub fn off_velocity(&self) -> u8 {
        self.off_velocity
    }
}

impl fmt::Display for NoteOff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Note Off, Channel: {}, Note number: {}, Key velocity: {}",
            self.channel(),
            self.note_number(),
            self.off_velocity()
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PitchBend {
    channel: u8,
    pitch_bend_change: u16,
}

impl PitchBend {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn pitch_bend_change(&self) -> u16 {
        self.pitch_bend_change
    }
}

impl fmt::Display for PitchBend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pitch Bend, Channel: {}, Pitch bend change: {}",
            self.channel(),
            self.pitch_bend_change()
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ControlChange {
    channel: u8,
    control_number: u8,
    control_value: u8,
}

impl ControlChange {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn control_number(&self) -> u8 {
        self.control_number
    }

    pub fn control_value(&self) -> u8 {
        self.control_value
    }
}

impl fmt::Display for ControlChange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Control Change, Channel: {}, Control number: {}, Control value: {}",
            self.channel(),
            self.control_number(),
            self.control_value()
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProgramChange {
    channel: u8,
    program_number: u8,
}

impl ProgramChange {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn program_number(&self) -> u8 {
        self.program_number
    }
}

impl fmt::Display for ProgramChange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Program Change, Channel: {}, Program number: {}",
            self.channel(),
            self.program_number(),
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SystemExlusiveId {
    OneByte(u8),
    TwoByte(u8, u8),
}

impl fmt::Display for SystemExlusiveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SystemExlusiveId::OneByte(a) => write!(f, "0x{:02x}", a),
            SystemExlusiveId::TwoByte(a, b) => write!(f, "0x00 0x{:02x} 0x{:02x}", a, b),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SystemExclusive {
    id: SystemExlusiveId,
    payload: Vec<u8>,
}

impl SystemExclusive {
    pub fn id(&self) -> SystemExlusiveId {
        self.id.clone()
    }

    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }
}

impl fmt::Display for SystemExclusive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "System Exclusive, ID: {}, Payload: 0x{:02x}",
            self.id(),
            self.payload().iter().format(" 0x")
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum MidiMessage {
    NoteOn(NoteOn),
    NoteOff(NoteOff),
    PitchBend(PitchBend),
    ControlChange(ControlChange),
    ProgramChange(ProgramChange),
    SystemExlusive(SystemExclusive),
}

impl fmt::Display for MidiMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MidiMessage::NoteOn(ref note_on) => write!(f, "{}", note_on),
            MidiMessage::NoteOff(ref note_off) => write!(f, "{}", note_off),
            MidiMessage::PitchBend(ref pitch_bend) => write!(f, "{}", pitch_bend),
            MidiMessage::ControlChange(ref control_change) => write!(f, "{}", control_change),
            MidiMessage::ProgramChange(ref program_change) => write!(f, "{}", program_change),
            MidiMessage::SystemExlusive(ref system_exclusive) => write!(f, "{}", system_exclusive),
        }
    }
}

impl MidiMessage {
    fn from_bytes(input: &[u8]) -> Self {
        assert!(input.len() >= 3);

        let message_type = (input[0] & 0xF0) >> 4;
        let channel = input[0] & 0x0F;

        match message_type {
            0x8 => MidiMessage::NoteOff(NoteOff {
                channel: channel,
                note_number: input[1] & 0x7F,
                off_velocity: input[2] & 0x7F,
            }),
            0x9 => {
                let key_velocity = input[2] & 0x7F;
                if key_velocity == 0 {
                    MidiMessage::NoteOff(NoteOff {
                        channel: channel,
                        note_number: input[1] & 0x7F,
                        off_velocity: 64,
                    })
                } else {
                    MidiMessage::NoteOn(NoteOn {
                        channel: channel,
                        note_number: input[1] & 0x7F,
                        key_velocity: key_velocity,
                    })
                }
            }
            0xb => MidiMessage::ControlChange(ControlChange {
                channel: channel,
                control_number: input[1] & 0x7F,
                control_value: input[2] & 0x7F,
            }),
            0xc => MidiMessage::ProgramChange(ProgramChange {
                channel: channel,
                program_number: input[1] & 0x7F,
            }),
            0xe => MidiMessage::PitchBend(PitchBend {
                channel: channel,
                pitch_bend_change: u16::from(input[2] & 0x7F) << 8 | u16::from(input[1] & 0x7F),
            }),
            _ => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct EventPacket {
    cable_number: u8,
    midi_message: MidiMessage,
}

impl EventPacket {
    #[allow(dead_code)]
    pub fn cable_number(&self) -> u8 {
        self.cable_number
    }

    pub fn midi_message(&self) -> MidiMessage {
        self.midi_message.clone()
    }
}

#[derive(PartialEq, Debug)]
pub enum MidiParseStatus {
    Complete(EventPacket),
    Incomplete,
    Unknown,
    MalformedPacket,
}

#[derive(Copy, Clone)]
enum SysExIdStatus {
    Empty,
    OneByte(u8),
    NeedTwoMore,
    NeedOneMore(u8),
    TwoByte(u8, u8),
}

impl SysExIdStatus {
    fn give_byte(&mut self, byte: u8) -> bool {
        match *self {
            SysExIdStatus::OneByte(_) | SysExIdStatus::TwoByte(_, _) => return false,
            _ => {}
        }

        *self = match *self {
            SysExIdStatus::Empty => if byte == 0 {
                SysExIdStatus::NeedTwoMore
            } else {
                SysExIdStatus::OneByte(byte)
            },
            SysExIdStatus::NeedTwoMore => SysExIdStatus::NeedOneMore(byte),
            SysExIdStatus::NeedOneMore(a) => SysExIdStatus::TwoByte(a, byte),
            _ => unreachable!(),
        };

        true
    }

    fn get_id_and_reset(&mut self) -> SystemExlusiveId {
        let id = match *self {
            SysExIdStatus::Empty | SysExIdStatus::NeedTwoMore | SysExIdStatus::NeedOneMore(_) => {
                SystemExlusiveId::OneByte(0)
            }
            SysExIdStatus::OneByte(id) => SystemExlusiveId::OneByte(id),
            SysExIdStatus::TwoByte(a, b) => SystemExlusiveId::TwoByte(a, b),
        };

        *self = SysExIdStatus::Empty;

        id
    }
}

pub struct UsbMidiParser {
    system_exclusive_ids: [SysExIdStatus; 16],
    system_exclusive_messages: [Vec<u8>; 16],
    system_exclusive_started: [bool; 16],
}

impl UsbMidiParser {
    pub fn new() -> Self {
        Self {
            system_exclusive_ids: [SysExIdStatus::Empty; 16],
            system_exclusive_messages: [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            system_exclusive_started: [false; 16],
        }
    }

    pub fn parse(&mut self, input: &[u8]) -> (MidiParseStatus, usize) {
        if input.len() < 4 {
            return (MidiParseStatus::Incomplete, 0);
        }

        let mut n = 0;
        let mut result = MidiParseStatus::Unknown;

        while input.len() - n >= 4 {
            let cable_number = (input[n] & 0xF0) >> 4;
            let code_index_number = input[n] & 0x0F;
            result = match code_index_number {
                0x8 | 0x9 | 0xb | 0xc | 0xe => MidiParseStatus::Complete(EventPacket {
                    cable_number: cable_number,
                    midi_message: MidiMessage::from_bytes(&input[1..]),
                }),
                0x4 => self.system_exclusive(&input[n + 1..n + 4], cable_number, false),
                0x5 => self.system_exclusive(&input[n + 1..n + 2], cable_number, true),
                0x6 => self.system_exclusive(&input[n + 1..n + 3], cable_number, true),
                0x7 => self.system_exclusive(&input[n + 1..n + 4], cable_number, true),
                _ => return (MidiParseStatus::Unknown, 1),
            };

            n += 4;

            match result {
                MidiParseStatus::Incomplete => {}
                _ => break,
            }
        }

        (result, n)
    }

    fn system_exclusive(
        &mut self,
        input: &[u8],
        cable_number: u8,
        terminate: bool,
    ) -> MidiParseStatus {
        for byte in input.iter().filter(|&byte| *byte <= 0x7F) {
            if !self.system_exclusive_ids[cable_number as usize].give_byte(*byte) {
                self.system_exclusive_messages[cable_number as usize].push(*byte);
            }
        }

        if terminate {
            let payload = mem::replace(
                &mut self.system_exclusive_messages[cable_number as usize],
                Vec::new(),
            );
            let id = self.system_exclusive_ids[cable_number as usize].get_id_and_reset();
            let result = if self.system_exclusive_started[cable_number as usize] {
                MidiParseStatus::Complete(EventPacket {
                    cable_number: cable_number,
                    midi_message: MidiMessage::SystemExlusive(SystemExclusive {
                        id: id,
                        payload: payload,
                    }),
                })
            } else {
                MidiParseStatus::MalformedPacket
            };

            self.system_exclusive_started[cable_number as usize] = false;

            result
        } else {
            self.system_exclusive_started[cable_number as usize] = true;
            MidiParseStatus::Incomplete
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_returns_incomplete_when_given_less_than_4_bytes() {
        let buf: [u8; 2] = [0; 2];
        let mut usb_midi_parser = UsbMidiParser::new();
        assert_eq!(
            (MidiParseStatus::Incomplete, 0),
            usb_midi_parser.parse(&buf)
        );
    }

    #[test]
    fn parse_returns_unknown_if_bytes_unknown_midi_message() {
        let buf: [u8; 4] = [0; 4];
        let mut usb_midi_parser = UsbMidiParser::new();
        assert_eq!((MidiParseStatus::Unknown, 1), usb_midi_parser.parse(&buf));
    }

    #[test]
    fn parse_returns_note_on_message() {
        let buf: [u8; 4] = [0x29, 0x94, 0x60, 0x65];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let note_on = match midi_message {
            MidiMessage::NoteOn(note_on) => note_on,
            _ => panic!("wrong variant"),
        };

        assert_eq!(4, note_on.channel());
        assert_eq!(0x60, note_on.note_number());
        assert_eq!(0x65, note_on.key_velocity());
    }

    #[test]
    fn parse_returns_note_off_message() {
        let buf: [u8; 4] = [0x28, 0x84, 0x60, 0x50];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let note_off = match midi_message {
            MidiMessage::NoteOff(note_off) => note_off,
            _ => panic!("wrong variant"),
        };

        assert_eq!(4, note_off.channel());
        assert_eq!(0x60, note_off.note_number());
        assert_eq!(0x50, note_off.off_velocity());
    }

    #[test]
    fn parse_returns_note_off_message_when_note_on_with_velocity_0() {
        let buf: [u8; 4] = [0x29, 0x94, 0x60, 0x00];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let note_off = match midi_message {
            MidiMessage::NoteOff(note_off) => note_off,
            _ => panic!("wrong variant"),
        };

        assert_eq!(4, note_off.channel());
        assert_eq!(0x60, note_off.note_number());
        assert_eq!(64, note_off.off_velocity());
    }

    #[test]
    fn parse_returns_pitch_bend_message() {
        let buf: [u8; 4] = [0x2e, 0xe5, 0x50, 0x40];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let pitch_bend = match midi_message {
            MidiMessage::PitchBend(pitch_bend) => pitch_bend,
            _ => panic!("wrong variant"),
        };

        assert_eq!(5, pitch_bend.channel());
        assert_eq!(0x4050, pitch_bend.pitch_bend_change());
    }

    #[test]
    fn parse_returns_control_change_message() {
        let buf: [u8; 4] = [0x2b, 0xb3, 0x01, 0x50];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let control_change = match midi_message {
            MidiMessage::ControlChange(control_change) => control_change,
            _ => panic!("wrong variant"),
        };

        assert_eq!(3, control_change.channel());
        assert_eq!(1, control_change.control_number());
        assert_eq!(0x50, control_change.control_value());
    }

    #[test]
    fn parse_returns_program_change_message() {
        let buf: [u8; 4] = [0x6c, 0xc5, 0x4, 0x0];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(6, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let program_change = match midi_message {
            MidiMessage::ProgramChange(program_change) => program_change,
            _ => panic!("wrong variant"),
        };

        assert_eq!(5, program_change.channel());
        assert_eq!(4, program_change.program_number());
    }

    #[test]
    fn parse_starts_system_exclusive_message() {
        let buf: [u8; 4] = [0x24, 0xf0, 0x7e, 0x1];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        match midi_message {
            (MidiParseStatus::Incomplete, n) => {
                assert_eq!(4, n);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_ends_system_exclusive_message_with_single_byte() {
        let buf: [u8; 8] = [0x24, 0xf0, 0x7e, 0x1, 0x25, 0xf7, 0x0, 0x0];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(8, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());
    }

    #[test]
    fn parse_ends_system_exclusive_message_with_two_bytes() {
        let buf: [u8; 8] = [0x24, 0xf0, 0x7e, 0x1, 0x26, 0x2, 0xf7, 0x0];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(8, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1, 0x2];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());
    }

    #[test]
    fn parse_ends_system_exclusive_message_with_three_bytes() {
        let buf: [u8; 8] = [0x24, 0xf0, 0x7e, 0x1, 0x27, 0x2, 0x3, 0xf7];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(8, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1, 0x2, 0x3];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());
    }

    #[test]
    fn parse_ends_system_exclusive_message_with_three_packets() {
        let buf: [u8; 12] = [
            0x24,
            0xf0,
            0x7e,
            0x1,
            0x24,
            0x2,
            0x3,
            0x4,
            0x25,
            0xf7,
            0x00,
            0x00,
        ];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(12, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());
    }

    #[test]
    fn parse_separates_interleaved_system_exclusive_message() {
        let buf: [u8; 16] = [
            0x14, // Cable #1
            0xf0,
            0x7e,
            0x1,
            0x24, // Cable #2
            0xf0,
            0x7e,
            0x5,
            0x15, // Cable #1
            0xf7,
            0x00,
            0x00,
            0x26, // Cable #2
            0x6,
            0xf7,
            0x0,
        ];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(1, packet.cable_number());
                assert_eq!(12, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());

        let midi_message = usb_midi_parser.parse(&buf[12..]);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x5, 0x6];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());
    }

    #[test]
    fn parse_id_of_system_exclusive_message() {
        let buf: [u8; 8] = [0x24, 0xf0, 0x00, 0x10, 0x27, 0x11, 0x3, 0xf7];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(8, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x3];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::TwoByte(0x10, 0x11), system_exclusive.id());
    }

    #[test]
    fn parse_ids_of_interleaved_system_exclusive_messages() {
        let buf: [u8; 16] = [
            0x24,
            0xf0,
            0x00,
            0x10,
            0x44,
            0xf0,
            0x00,
            0x12,
            0x27,
            0x11,
            0x3,
            0xf7,
            0x47,
            0x34,
            0x5,
            0xf7,
        ];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(12, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x3];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::TwoByte(0x10, 0x11), system_exclusive.id());

        let midi_message = usb_midi_parser.parse(&buf[12..]);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(4, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x5];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::TwoByte(0x12, 0x34), system_exclusive.id());
    }

    #[test]
    fn parse_ids_of_successive_system_exclusive_messages() {
        let buf: [u8; 16] = [
            0x24,
            0xf0,
            0x00,
            0x10,
            0x27,
            0x11,
            0x3,
            0xf7,
            0x24,
            0xf0,
            0x7e,
            0x42,
            0x25,
            0xf7,
            0x0,
            0x0,
        ];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(8, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x3];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::TwoByte(0x10, 0x11), system_exclusive.id());

        let midi_message = usb_midi_parser.parse(&buf[8..]);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(2, packet.cable_number());
                assert_eq!(8, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let system_exclusive = match midi_message {
            MidiMessage::SystemExlusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x42];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());
    }

    #[test]
    fn parse_returns_malformed_if_system_exclusive_message_was_not_started() {
        let buf: [u8; 4] = [0x26, 0x12, 0xf7, 0x0];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        match midi_message {
            (MidiParseStatus::MalformedPacket, n) => {
                assert_eq!(4, n);
            }
            _ => panic!("wrong variant"),
        }
    }
}
