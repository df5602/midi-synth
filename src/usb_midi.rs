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
    pub fn create(channel: u8, note_number: u8, key_velocity: u8) -> MidiMessage {
        MidiMessage::NoteOn(NoteOn {
            channel: channel & 0x0F,
            note_number: note_number & 0x7F,
            key_velocity: key_velocity & 0x7F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note_number(&self) -> u8 {
        self.note_number
    }

    pub fn key_velocity(&self) -> u8 {
        self.key_velocity
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0x90 | (self.channel & 0x0F);
        buf[1] = self.note_number & 0x7F;
        buf[2] = self.key_velocity & 0x7F;
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
    pub fn create(channel: u8, note_number: u8, off_velocity: u8) -> MidiMessage {
        MidiMessage::NoteOff(NoteOff {
            channel: channel & 0x0F,
            note_number: note_number & 0x7F,
            off_velocity: off_velocity & 0x7F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note_number(&self) -> u8 {
        self.note_number
    }

    pub fn off_velocity(&self) -> u8 {
        self.off_velocity
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0x80 | (self.channel & 0x0F);
        buf[1] = self.note_number & 0x7F;
        buf[2] = self.off_velocity & 0x7F;
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
    pub fn create(channel: u8, pitch_bend_change: u16) -> MidiMessage {
        MidiMessage::PitchBend(PitchBend {
            channel: channel & 0x0F,
            pitch_bend_change: pitch_bend_change & 0x3FFF,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn pitch_bend_change(&self) -> u16 {
        self.pitch_bend_change
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xE0 | (self.channel & 0x0F);
        buf[1] = (self.pitch_bend_change & 0x7F) as u8;
        buf[2] = ((self.pitch_bend_change >> 7) & 0x7F) as u8;
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
    pub fn create(channel: u8, control_number: u8, control_value: u8) -> MidiMessage {
        MidiMessage::ControlChange(ControlChange {
            channel: channel & 0x0F,
            control_number: control_number & 0x7F,
            control_value: control_value & 0x7F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn control_number(&self) -> u8 {
        self.control_number
    }

    pub fn control_value(&self) -> u8 {
        self.control_value
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = self.control_number & 0x7F;
        buf[2] = self.control_value & 0x7F;
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
    pub fn create(channel: u8) -> MidiMessage {
        MidiMessage::AllSoundOff(AllSoundOff {
            channel: channel & 0x0F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = 120;
        buf[2] = 0;
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
    pub fn create(channel: u8) -> MidiMessage {
        MidiMessage::ResetAllControllers(ResetAllControllers {
            channel: channel & 0x0F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = 121;
        buf[2] = 0;
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
    pub fn create(channel: u8, on: bool) -> MidiMessage {
        MidiMessage::LocalControl(LocalControl {
            channel: channel & 0x0F,
            on: on,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn on(&self) -> bool {
        self.on
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = 122;
        buf[2] = if self.on { 127 } else { 0 };
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
    pub fn create(channel: u8) -> MidiMessage {
        MidiMessage::AllNotesOff(AllNotesOff {
            channel: channel & 0x0F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = 123;
        buf[2] = 0;
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
    pub fn create(channel: u8) -> MidiMessage {
        MidiMessage::OmniModeOff(OmniModeOff {
            channel: channel & 0x0F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = 124;
        buf[2] = 0;
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
    pub fn create(channel: u8) -> MidiMessage {
        MidiMessage::OmniModeOn(OmniModeOn {
            channel: channel & 0x0F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = 125;
        buf[2] = 0;
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
    pub fn create(channel: u8, number_of_channels: u8) -> MidiMessage {
        MidiMessage::MonoModeOn(MonoModeOn {
            channel: channel & 0x0F,
            number_of_channels: number_of_channels & 0x0F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn number_of_channels(&self) -> u8 {
        self.number_of_channels
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = 126;
        buf[2] = self.number_of_channels & 0x7F;
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
    pub fn create(channel: u8) -> MidiMessage {
        MidiMessage::PolyModeOn(PolyModeOn {
            channel: channel & 0x0F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xB0 | (self.channel & 0x0F);
        buf[1] = 127;
        buf[2] = 0;
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
    pub fn create(channel: u8, program_number: u8) -> MidiMessage {
        MidiMessage::ProgramChange(ProgramChange {
            channel: channel & 0x0F,
            program_number: program_number & 0x7F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn program_number(&self) -> u8 {
        self.program_number
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xC0 | (self.channel & 0x0F);
        buf[1] = self.program_number & 0x7F;
        buf[2] = 0;
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
    pub fn create(channel: u8, pressure: u8) -> MidiMessage {
        MidiMessage::ChannelPressure(ChannelPressure {
            channel: channel & 0x0F,
            pressure: pressure & 0x7F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn pressure(&self) -> u8 {
        self.pressure
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xD0 | (self.channel & 0x0F);
        buf[1] = self.pressure & 0x7F;
        buf[2] = 0;
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
    pub fn create(channel: u8, note_number: u8, pressure: u8) -> MidiMessage {
        MidiMessage::PolyphonicKeyPressure(PolyphonicKeyPressure {
            channel: channel & 0x0F,
            note_number: note_number & 0x7F,
            pressure: pressure & 0x7F,
        })
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note_number(&self) -> u8 {
        self.note_number
    }

    pub fn pressure(&self) -> u8 {
        self.pressure
    }

    fn as_bytes(&self, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        buf[0] = 0xA0 | (self.channel & 0x0F);
        buf[1] = self.note_number & 0x7F;
        buf[2] = self.pressure & 0x7F;
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
    pub fn create(id: SystemExlusiveId, payload: Vec<u8>) -> MidiMessage {
        MidiMessage::SystemExclusive(SystemExclusive {
            id: id,
            payload: payload,
        })
    }

    pub fn id(&self) -> SystemExlusiveId {
        self.id.clone()
    }

    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }

    fn length(&self) -> usize {
        let id_length = match self.id {
            SystemExlusiveId::OneByte(_) => 1,
            SystemExlusiveId::TwoByte(_, _) => 3,
        };
        id_length + self.payload.len() + 2 // SOX + ID + Payload + EOX
    }

    fn serialize(self) -> SysExSerializer {
        SysExSerializer {
            iteration: 0,
            payload_byte: 0,
            state: SysExSerializerState::StartOfExclusive,
            sysex: self,
        }
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

enum SysExSerializerState {
    StartOfExclusive,
    Id,
    Payload,
    EndOfExclusive,
    Finished,
}

struct SysExSerializer {
    iteration: usize,
    payload_byte: usize,
    state: SysExSerializerState,
    sysex: SystemExclusive,
}

impl SysExSerializer {
    fn remaining_length(&self) -> usize {
        self.sysex.length() - self.iteration
    }
}

impl Iterator for SysExSerializer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.state {
            SysExSerializerState::StartOfExclusive => {
                self.state = SysExSerializerState::Id;
                0xF0
            }
            SysExSerializerState::Id => match (&self.sysex.id, self.iteration) {
                (&SystemExlusiveId::OneByte(a), _) => {
                    self.state = SysExSerializerState::Payload;
                    a
                }
                (&SystemExlusiveId::TwoByte(_, _), 1) => 0,
                (&SystemExlusiveId::TwoByte(a, _), 2) => a,
                (&SystemExlusiveId::TwoByte(_, b), _) => {
                    self.state = SysExSerializerState::Payload;
                    b
                }
            },
            SysExSerializerState::Payload => {
                if self.payload_byte == self.sysex.payload.len() - 1 {
                    self.state = SysExSerializerState::EndOfExclusive;
                }
                let i = self.payload_byte;
                self.payload_byte += 1;
                self.sysex.payload[i]
            }
            SysExSerializerState::EndOfExclusive => {
                self.state = SysExSerializerState::Finished;
                0xF7
            }
            SysExSerializerState::Finished => return None,
        };
        self.iteration += 1;
        Some(next)
    }
}

pub struct Serializer {
    midi_message: MidiMessage,
    sysex: Option<SysExSerializer>,
    cable_number: u8,
    iteration: usize,
    finished: bool,
    bytes: [u8; 4],
}

impl Iterator for Serializer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match (&mut self.midi_message, self.iteration) {
            (&mut MidiMessage::NoteOn(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0x9 & 0x0F)
            }
            (&mut MidiMessage::NoteOff(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0x8 & 0x0F)
            }
            (&mut MidiMessage::PitchBend(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xE & 0x0F)
            }
            (&mut MidiMessage::ControlChange(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::AllSoundOff(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::ResetAllControllers(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::LocalControl(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::AllNotesOff(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::OmniModeOff(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::OmniModeOn(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::MonoModeOn(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::PolyModeOn(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xB & 0x0F)
            }
            (&mut MidiMessage::ProgramChange(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xC & 0x0F)
            }
            (&mut MidiMessage::ChannelPressure(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xD & 0x0F)
            }
            (&mut MidiMessage::PolyphonicKeyPressure(ref inner), 0) => {
                inner.as_bytes(&mut self.bytes[1..]);
                self.cable_number << 4 | (0xA & 0x0F)
            }
            (&mut MidiMessage::SystemExclusive(ref mut inner), n) => {
                if self.sysex.is_none() {
                    let sysex = mem::replace(
                        &mut *inner,
                        SystemExclusive {
                            id: SystemExlusiveId::OneByte(1),
                            payload: vec![],
                        },
                    );
                    self.sysex = Some(sysex.serialize());
                }

                match self.sysex {
                    Some(ref mut sysex) => {
                        if n % 4 == 0 {
                            let cin = match sysex.remaining_length() {
                                0 => {
                                    self.finished = true;
                                    return None;
                                }
                                1 => 0x5,
                                2 => 0x6,
                                3 => 0x7,
                                _ => 0x4,
                            };
                            self.cable_number << 4 | (cin & 0x0F)
                        } else {
                            match sysex.next() {
                                Some(byte) => byte,
                                None => {
                                    self.finished = true;
                                    0
                                }
                            }
                        }
                    }
                    None => unreachable!(),
                }
            }
            (_, n) if n < 4 => self.bytes[n],
            (_, _) => return None,
        };
        self.iteration += 1;
        Some(next)
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
    SystemExclusive(SystemExclusive),
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
            MidiMessage::SystemExclusive(ref inner) => write!(f, "{}", inner),
        }
    }
}

impl MidiMessage {
    pub fn serialize(self) -> Serializer {
        self.serialize_on_cable(0)
    }

    pub fn serialize_on_cable(self, cable_number: u8) -> Serializer {
        Serializer {
            midi_message: self,
            sysex: None,
            cable_number: cable_number,
            iteration: 0,
            finished: false,
            bytes: [0; 4],
        }
    }

    fn from_bytes(input: &[u8]) -> Self {
        assert!(input.len() >= 3);

        let message_type = (input[0] & 0xF0) >> 4;
        let channel = input[0];

        match message_type {
            0x8 => NoteOff::create(channel, input[1], input[2]),
            0x9 => {
                let key_velocity = input[2];
                if key_velocity == 0 {
                    NoteOff::create(channel, input[1], 64)
                } else {
                    NoteOn::create(channel, input[1], key_velocity)
                }
            }
            0xa => PolyphonicKeyPressure::create(channel, input[1], input[2]),
            0xb => {
                let control_number = input[1] & 0x7F;
                match control_number {
                    120 => AllSoundOff::create(channel),
                    121 => ResetAllControllers::create(channel),
                    122 => LocalControl::create(channel, (input[2] & 0x7F) >= 64),
                    123 => AllNotesOff::create(channel),
                    124 => OmniModeOff::create(channel),
                    125 => OmniModeOn::create(channel),
                    126 => MonoModeOn::create(channel, input[2]),
                    127 => PolyModeOn::create(channel),
                    _ => ControlChange::create(channel, control_number, input[2]),
                }
            }
            0xc => ProgramChange::create(channel, input[1]),
            0xd => ChannelPressure::create(channel, input[1]),
            0xe => PitchBend::create(
                channel,
                u16::from(input[2] & 0x7F) << 7 | u16::from(input[1] & 0x7F),
            ),
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

    #[allow(dead_code)]
    pub fn serialize(self) -> Serializer {
        self.midi_message.serialize_on_cable(self.cable_number)
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
                    midi_message: SystemExclusive::create(id, payload),
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

        let midi_message = NoteOn::create(
            note_on.channel(),
            note_on.note_number(),
            note_on.key_velocity(),
        );

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = NoteOff::create(
            note_off.channel(),
            note_off.note_number(),
            note_off.off_velocity(),
        );

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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
        let buf: [u8; 4] = [0x2e, 0xe5, 0x51, 0x41];
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
        assert_eq!((0x41 << 7) | 0x51, pitch_bend.pitch_bend_change());

        let midi_message = PitchBend::create(pitch_bend.channel(), pitch_bend.pitch_bend_change());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = ControlChange::create(
            control_change.channel(),
            control_change.control_number(),
            control_change.control_value(),
        );

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = AllSoundOff::create(all_sound_off.channel());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = ResetAllControllers::create(reset_all_controllers.channel());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = LocalControl::create(local_control.channel(), local_control.on());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = AllNotesOff::create(all_notes_off.channel());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = OmniModeOff::create(omni_mode_off.channel());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = OmniModeOn::create(omni_mode_on.channel());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message =
            MonoModeOn::create(mono_mode_on.channel(), mono_mode_on.number_of_channels());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = PolyModeOn::create(poly_mode_on.channel());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message =
            ProgramChange::create(program_change.channel(), program_change.program_number());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(6).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message =
            ChannelPressure::create(channel_pressure.channel(), channel_pressure.pressure());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(3).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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

        let midi_message = PolyphonicKeyPressure::create(
            poly_key_press.channel(),
            poly_key_press.note_number(),
            poly_key_press.pressure(),
        );

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 4);
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());

        let midi_message =
            SystemExclusive::create(system_exclusive.id(), system_exclusive.payload().clone());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, buf.len());
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1, 0x2];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());

        let midi_message =
            SystemExclusive::create(system_exclusive.id(), system_exclusive.payload().clone());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, buf.len());
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1, 0x2, 0x3];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());

        let midi_message =
            SystemExclusive::create(system_exclusive.id(), system_exclusive.payload().clone());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, buf.len());
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());

        let midi_message =
            SystemExclusive::create(system_exclusive.id(), system_exclusive.payload().clone());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, buf.len());
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x3];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::TwoByte(0x10, 0x11), system_exclusive.id());

        let midi_message =
            SystemExclusive::create(system_exclusive.id(), system_exclusive.payload().clone());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, buf.len());
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x3];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::TwoByte(0x10, 0x11), system_exclusive.id());

        let midi_message =
            SystemExclusive::create(system_exclusive.id(), system_exclusive.payload().clone());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf.into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 8);

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
            MidiMessage::SystemExclusive(system_exclusive) => system_exclusive,
            _ => panic!("wrong variant"),
        };

        let expected: Vec<u8> = vec![0x42];
        assert_eq!(expected, *system_exclusive.payload());
        assert_eq!(SystemExlusiveId::OneByte(0x7e), system_exclusive.id());

        let midi_message =
            SystemExclusive::create(system_exclusive.id(), system_exclusive.payload().clone());

        let mut i = 0;
        for (a, &b) in midi_message.serialize_on_cable(2).zip(buf[8..].into_iter()) {
            assert_eq!(a, b);
            i += 1;
        }
        assert_eq!(i, 8);
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
