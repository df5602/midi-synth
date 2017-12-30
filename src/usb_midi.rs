use std::fmt;

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
pub enum MidiMessage {
    NoteOn(NoteOn),
    NoteOff(NoteOff),
    PitchBend(PitchBend),
    ControlChange(ControlChange),
    ProgramChange(ProgramChange),
}

impl fmt::Display for MidiMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MidiMessage::NoteOn(ref note_on) => write!(f, "{}", note_on),
            MidiMessage::NoteOff(ref note_off) => write!(f, "{}", note_off),
            MidiMessage::PitchBend(ref pitch_bend) => write!(f, "{}", pitch_bend),
            MidiMessage::ControlChange(ref control_change) => write!(f, "{}", control_change),
            MidiMessage::ProgramChange(ref program_change) => write!(f, "{}", program_change),
        }
    }
}

impl MidiMessage {
    pub fn from_bytes(input: &[u8]) -> Self {
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
}

pub fn from_bytes(input: &[u8]) -> (MidiParseStatus, usize) {
    if input.len() < 4 {
        return (MidiParseStatus::Incomplete, 0);
    }

    let cable_number = (input[0] & 0xF0) >> 4;
    let code_index_number = input[0] & 0x0F;

    let result = match code_index_number {
        0x8 | 0x9 | 0xb | 0xc | 0xe => MidiParseStatus::Complete(EventPacket {
            cable_number: cable_number,
            midi_message: MidiMessage::from_bytes(&input[1..]),
        }),
        _ => return (MidiParseStatus::Unknown, 1),
    };

    (result, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_returns_incomplete_when_given_less_than_4_bytes() {
        let buf: [u8; 2] = [0; 2];
        assert_eq!((MidiParseStatus::Incomplete, 0), from_bytes(&buf));
    }

    #[test]
    fn from_bytes_returns_unknown_if_bytes_unknown_midi_message() {
        let buf: [u8; 4] = [0; 4];
        assert_eq!((MidiParseStatus::Unknown, 1), from_bytes(&buf));
    }

    #[test]
    fn from_bytes_returns_note_on_message() {
        let buf: [u8; 4] = [0x29, 0x94, 0x60, 0x65];
        let midi_message = from_bytes(&buf);

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
    fn from_bytes_returns_note_off_message() {
        let buf: [u8; 4] = [0x28, 0x84, 0x60, 0x50];
        let midi_message = from_bytes(&buf);

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
    fn from_bytes_returns_note_off_message_when_note_on_with_velocity_0() {
        let buf: [u8; 4] = [0x29, 0x94, 0x60, 0x00];
        let midi_message = from_bytes(&buf);

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
    fn from_bytes_returns_pitch_bend_message() {
        let buf: [u8; 4] = [0x2e, 0xe5, 0x50, 0x40];
        let midi_message = from_bytes(&buf);

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
    fn from_bytes_returns_control_change_message() {
        let buf: [u8; 4] = [0x2b, 0xb3, 0x01, 0x50];
        let midi_message = from_bytes(&buf);

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
    fn from_bytes_returns_program_change_message() {
        let buf: [u8; 4] = [0x6c, 0xc5, 0x4, 0x0];
        let midi_message = from_bytes(&buf);

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
}
