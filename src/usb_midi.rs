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
pub struct AllSoundOff {
    channel: u8,
}

impl AllSoundOff {
    pub fn channel(&self) -> u8 {
        self.channel
    }
}

impl fmt::Display for AllSoundOff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Channel Mode, Channel: {}, All Sound Off",
            self.channel(),
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ResetAllControllers {
    channel: u8,
}

impl ResetAllControllers {
    pub fn channel(&self) -> u8 {
        self.channel
    }
}

impl fmt::Display for ResetAllControllers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Channel Mode, Channel: {}, Reset All Controllers",
            self.channel(),
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LocalControl {
    channel: u8,
    on: bool,
}

impl LocalControl {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn on(&self) -> bool {
        self.on
    }
}

impl fmt::Display for LocalControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Channel Mode, Channel: {}, Local Control {}",
            self.channel(),
            if self.on() { "On" } else { "Off" }
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AllNotesOff {
    channel: u8,
}

impl AllNotesOff {
    pub fn channel(&self) -> u8 {
        self.channel
    }
}

impl fmt::Display for AllNotesOff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Channel Mode, Channel: {}, All Notes Off",
            self.channel(),
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct OmniModeOff {
    channel: u8,
}

impl OmniModeOff {
    pub fn channel(&self) -> u8 {
        self.channel
    }
}

impl fmt::Display for OmniModeOff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Channel Mode, Channel: {}, Omni Mode Off",
            self.channel(),
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct OmniModeOn {
    channel: u8,
}

impl OmniModeOn {
    pub fn channel(&self) -> u8 {
        self.channel
    }
}

impl fmt::Display for OmniModeOn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Channel Mode, Channel: {}, Omni Mode On", self.channel())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MonoModeOn {
    channel: u8,
    number_of_channels: u8,
}

impl MonoModeOn {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn number_of_channels(&self) -> u8 {
        self.number_of_channels
    }
}

impl fmt::Display for MonoModeOn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Channel Mode, Channel: {}, Mono Mode On, Number of channels: {}",
            self.channel(),
            self.number_of_channels()
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PolyModeOn {
    channel: u8,
}

impl PolyModeOn {
    pub fn channel(&self) -> u8 {
        self.channel
    }
}

impl fmt::Display for PolyModeOn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Channel Mode, Channel: {}, Poly Mode On", self.channel())
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
pub struct ChannelPressure {
    channel: u8,
    pressure: u8,
}

impl ChannelPressure {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn pressure(&self) -> u8 {
        self.pressure
    }
}

impl fmt::Display for ChannelPressure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Channel Pressure, Channel: {}, Pressure value: {}",
            self.channel(),
            self.pressure(),
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PolyphonicKeyPressure {
    channel: u8,
    note_number: u8,
    pressure: u8,
}

impl PolyphonicKeyPressure {
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note_number(&self) -> u8 {
        self.note_number
    }

    pub fn pressure(&self) -> u8 {
        self.pressure
    }
}

impl fmt::Display for PolyphonicKeyPressure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Polyphonic Key Pressure, Channel: {}, Note number: {}, Pressure value: {}",
            self.channel(),
            self.note_number(),
            self.pressure(),
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
    AllSoundOff(AllSoundOff),
    ResetAllControllers(ResetAllControllers),
    LocalControl(LocalControl),
    AllNotesOff(AllNotesOff),
    OmniModeOff(OmniModeOff),
    OmniModeOn(OmniModeOn),
    MonoModeOn(MonoModeOn),
    PolyModeOn(PolyModeOn),
    ProgramChange(ProgramChange),
    ChannelPressure(ChannelPressure),
    PolyphonicKeyPressure(PolyphonicKeyPressure),
    SystemExlusive(SystemExclusive),
}

impl fmt::Display for MidiMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MidiMessage::NoteOn(ref inner) => write!(f, "{}", inner),
            MidiMessage::NoteOff(ref inner) => write!(f, "{}", inner),
            MidiMessage::PitchBend(ref inner) => write!(f, "{}", inner),
            MidiMessage::ControlChange(ref inner) => write!(f, "{}", inner),
            MidiMessage::AllSoundOff(ref inner) => write!(f, "{}", inner),
            MidiMessage::ResetAllControllers(ref inner) => write!(f, "{}", inner),
            MidiMessage::LocalControl(ref inner) => write!(f, "{}", inner),
            MidiMessage::AllNotesOff(ref inner) => write!(f, "{}", inner),
            MidiMessage::OmniModeOff(ref inner) => write!(f, "{}", inner),
            MidiMessage::OmniModeOn(ref inner) => write!(f, "{}", inner),
            MidiMessage::MonoModeOn(ref inner) => write!(f, "{}", inner),
            MidiMessage::PolyModeOn(ref inner) => write!(f, "{}", inner),
            MidiMessage::ProgramChange(ref inner) => write!(f, "{}", inner),
            MidiMessage::ChannelPressure(ref inner) => write!(f, "{}", inner),
            MidiMessage::PolyphonicKeyPressure(ref inner) => write!(f, "{}", inner),
            MidiMessage::SystemExlusive(ref inner) => write!(f, "{}", inner),
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
            0xa => MidiMessage::PolyphonicKeyPressure(PolyphonicKeyPressure {
                channel: channel,
                note_number: input[1] & 0x7F,
                pressure: input[2] & 0x7F,
            }),
            0xb => {
                let control_number = input[1] & 0x7F;
                match control_number {
                    120 => MidiMessage::AllSoundOff(AllSoundOff { channel: channel }),
                    121 => {
                        MidiMessage::ResetAllControllers(ResetAllControllers { channel: channel })
                    }
                    122 => MidiMessage::LocalControl(LocalControl {
                        channel: channel,
                        on: input[2] & 0x7F >= 64,
                    }),
                    123 => MidiMessage::AllNotesOff(AllNotesOff { channel: channel }),
                    124 => MidiMessage::OmniModeOff(OmniModeOff { channel: channel }),
                    125 => MidiMessage::OmniModeOn(OmniModeOn { channel: channel }),
                    126 => MidiMessage::MonoModeOn(MonoModeOn {
                        channel: channel,
                        number_of_channels: input[2] & 0x0F,
                    }),
                    127 => MidiMessage::PolyModeOn(PolyModeOn { channel: channel }),
                    _ => MidiMessage::ControlChange(ControlChange {
                        channel: channel,
                        control_number: control_number,
                        control_value: input[2] & 0x7F,
                    }),
                }
            }
            0xc => MidiMessage::ProgramChange(ProgramChange {
                channel: channel,
                program_number: input[1] & 0x7F,
            }),
            0xd => MidiMessage::ChannelPressure(ChannelPressure {
                channel: channel,
                pressure: input[1] & 0x7F,
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
                0x8 | 0x9 | 0xa | 0xb | 0xc | 0xd | 0xe => MidiParseStatus::Complete(EventPacket {
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
    fn parse_returns_all_sound_off_channel_mode_message() {
        let buf: [u8; 4] = [0x2b, 0xb1, 120, 0x00];
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

        let all_sound_off = match midi_message {
            MidiMessage::AllSoundOff(all_sound_off) => all_sound_off,
            _ => panic!("wrong variant"),
        };

        assert_eq!(1, all_sound_off.channel());
    }

    #[test]
    fn parse_returns_reset_all_controllers_channel_mode_message() {
        let buf: [u8; 4] = [0x2b, 0xb1, 121, 0x00];
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

        let reset_all_controllers = match midi_message {
            MidiMessage::ResetAllControllers(reset_all_controllers) => reset_all_controllers,
            _ => panic!("wrong variant"),
        };

        assert_eq!(1, reset_all_controllers.channel());
    }

    #[test]
    fn parse_returns_local_control_channel_mode_message() {
        let buf: [u8; 4] = [0x2b, 0xb1, 122, 0x7F];
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

        let local_control = match midi_message {
            MidiMessage::LocalControl(local_control) => local_control,
            _ => panic!("wrong variant"),
        };

        assert_eq!(1, local_control.channel());
        assert_eq!(true, local_control.on());
    }

    #[test]
    fn parse_returns_all_notes_off_channel_mode_message() {
        let buf: [u8; 4] = [0x2b, 0xb1, 123, 0x00];
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

        let all_notes_off = match midi_message {
            MidiMessage::AllNotesOff(all_notes_off) => all_notes_off,
            _ => panic!("wrong variant"),
        };

        assert_eq!(1, all_notes_off.channel());
    }

    #[test]
    fn parse_returns_omni_mode_off_channel_mode_message() {
        let buf: [u8; 4] = [0x2b, 0xb1, 124, 0x00];
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

        let omni_mode_off = match midi_message {
            MidiMessage::OmniModeOff(omni_mode_off) => omni_mode_off,
            _ => panic!("wrong variant"),
        };

        assert_eq!(1, omni_mode_off.channel());
    }

    #[test]
    fn parse_returns_omni_mode_on_channel_mode_message() {
        let buf: [u8; 4] = [0x2b, 0xb1, 125, 0x00];
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

        let omni_mode_on = match midi_message {
            MidiMessage::OmniModeOn(omni_mode_on) => omni_mode_on,
            _ => panic!("wrong variant"),
        };

        assert_eq!(1, omni_mode_on.channel());
    }

    #[test]
    fn parse_returns_mono_mode_on_channel_mode_message() {
        let buf: [u8; 4] = [0x2b, 0xb1, 126, 0x08];
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

        let mono_mode_on = match midi_message {
            MidiMessage::MonoModeOn(mono_mode_on) => mono_mode_on,
            _ => panic!("wrong variant"),
        };

        assert_eq!(1, mono_mode_on.channel());
        assert_eq!(8, mono_mode_on.number_of_channels());
    }

    #[test]
    fn parse_returns_poly_mode_on_channel_mode_message() {
        let buf: [u8; 4] = [0x2b, 0xb1, 127, 0x00];
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

        let poly_mode_on = match midi_message {
            MidiMessage::PolyModeOn(poly_mode_on) => poly_mode_on,
            _ => panic!("wrong variant"),
        };

        assert_eq!(1, poly_mode_on.channel());
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
    fn parse_returns_channel_pressure_message() {
        let buf: [u8; 4] = [0x3d, 0xd2, 0x20, 0x0];
        let mut usb_midi_parser = UsbMidiParser::new();
        let midi_message = usb_midi_parser.parse(&buf);

        let midi_message = match midi_message {
            (MidiParseStatus::Complete(packet), n) => {
                assert_eq!(3, packet.cable_number());
                assert_eq!(4, n);
                packet.midi_message()
            }
            _ => panic!("wrong variant"),
        };

        let channel_pressure = match midi_message {
            MidiMessage::ChannelPressure(channel_pressure) => channel_pressure,
            _ => panic!("wrong variant"),
        };

        assert_eq!(2, channel_pressure.channel());
        assert_eq!(0x20, channel_pressure.pressure());
    }

    #[test]
    fn parse_returns_polyphonic_key_pressure_message() {
        let buf: [u8; 4] = [0x2a, 0xa5, 0x12, 0x13];
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

        let poly_key_press = match midi_message {
            MidiMessage::PolyphonicKeyPressure(poly_key_press) => poly_key_press,
            _ => panic!("wrong variant"),
        };

        assert_eq!(5, poly_key_press.channel());
        assert_eq!(0x12, poly_key_press.note_number());
        assert_eq!(0x13, poly_key_press.pressure());
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
            0x24, 0xf0, 0x7e, 0x1, 0x24, 0x2, 0x3, 0x4, 0x25, 0xf7, 0x00, 0x00
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
            0x24, 0xf0, 0x00, 0x10, 0x44, 0xf0, 0x00, 0x12, 0x27, 0x11, 0x3, 0xf7, 0x47, 0x34, 0x5,
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
            0x24, 0xf0, 0x00, 0x10, 0x27, 0x11, 0x3, 0xf7, 0x24, 0xf0, 0x7e, 0x42, 0x25, 0xf7, 0x0,
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
