use std::f32;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

use synth::dispatcher::SynthControl;
use synth::mixer::Mixer;
use synth::oscillator::Oscillator;
use synth::sample_stream::SampleStream;

pub struct Synthesizer {
    osc1: Rc<Oscillator>,
    mixer: Mixer,
    note_selector: NoteSelector,
    ctrl_in: Receiver<SynthControl>,
}

impl Synthesizer {
    pub fn new(ctrl_in: Receiver<SynthControl>) -> Self {
        let osc1 = Rc::new(Oscillator::new(1.0, 0.0));
        Self {
            osc1: Rc::clone(&osc1),
            mixer: Mixer::new(osc1),
            note_selector: NoteSelector::new(),
            ctrl_in,
        }
    }

    fn turn_on_note(&mut self, note: f32) {
        self.osc1.set_note(self.note_selector.turn_on_note(note));
    }

    fn turn_off_note(&mut self, note: f32) {
        self.osc1.set_note(self.note_selector.turn_off_note(note));
    }

    pub fn next_sample(&mut self) -> f32 {
        if let Ok(f) = self.ctrl_in.try_recv() {
            match f {
                SynthControl::MasterTune(frequency) => self.osc1.set_master_tune(frequency),
                SynthControl::Oscillator1Range(range) => self.osc1.set_range(range),
                SynthControl::Oscillator1Enable(enabled) => self.mixer.set_enabled(enabled),
                SynthControl::Oscillator1Volume(volume) => self.mixer.set_volume(volume),
                SynthControl::NoteOn(note) => self.turn_on_note(note),
                SynthControl::NoteOff(note) => self.turn_off_note(note),
            }
        }

        self.mixer.next_sample()
    }
}

#[derive(Copy, Clone)]
struct Note {
    on: bool,
    note: f32,
}

struct NoteSelector {
    notes: [Note; 32],
    current_note: f32,
}

impl NoteSelector {
    fn new() -> Self {
        Self {
            notes: [Note {
                on: false,
                note: 0.0,
            }; 32],
            current_note: f32::INFINITY,
        }
    }

    fn turn_on_note(&mut self, note: f32) -> f32 {
        for i in 0..self.notes.len() {
            if !self.notes[i].on {
                self.notes[i].on = true;
                self.notes[i].note = note;
                break;
            }
        }

        self.play_lowest_note()
    }

    fn turn_off_note(&mut self, note: f32) -> f32 {
        for i in 0..self.notes.len() {
            if self.notes[i].on && ((note - self.notes[i].note).abs() < 1e-6) {
                self.notes[i].on = false;
                self.notes[i].note = 0.0;
                break;
            }
        }

        self.play_lowest_note()
    }

    fn play_lowest_note(&mut self) -> f32 {
        let current = self.current_note;
        let mut lowest = f32::INFINITY;

        for i in 0..self.notes.len() {
            if self.notes[i].on {
                lowest = if self.notes[i].note < lowest {
                    self.notes[i].note
                } else {
                    lowest
                };
            }
        }

        self.current_note = if lowest < f32::INFINITY {
            lowest
        } else {
            current
        };
        self.current_note
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_float_eq {
        ($left:expr, $right:expr, $eps:expr) => {{
            let left = $left;
            let right = $right;
            assert!(
                (left - right).abs() < $eps,
                "Expected: {}, got: {}",
                left,
                right
            );
        }};
    }

    #[test]
    fn new_note_is_higher() {
        let mut note_selector = NoteSelector::new();

        note_selector.turn_on_note(1.2);
        let current_note = note_selector.turn_on_note(1.5);
        assert_float_eq!(1.2, current_note, 1e-6);
    }

    #[test]
    fn new_note_is_lower() {
        let mut note_selector = NoteSelector::new();

        note_selector.turn_on_note(1.2);
        let current_note = note_selector.turn_on_note(0.8);
        assert_float_eq!(0.8, current_note, 1e-6);
    }

    #[test]
    fn release_lower_note() {
        let mut note_selector = NoteSelector::new();

        note_selector.turn_on_note(0.8);
        let mut current_note = note_selector.turn_on_note(1.2);
        assert_float_eq!(0.8, current_note, 1e-6);

        current_note = note_selector.turn_off_note(0.8);
        assert_float_eq!(1.2, current_note, 1e-6);

        current_note = note_selector.turn_off_note(1.2);
        assert_float_eq!(1.2, current_note, 1e-6);
    }

    #[test]
    fn release_higher_note() {
        let mut note_selector = NoteSelector::new();

        note_selector.turn_on_note(0.8);
        let mut current_note = note_selector.turn_on_note(1.2);
        assert_float_eq!(0.8, current_note, 1e-6);

        current_note = note_selector.turn_off_note(1.2);
        assert_float_eq!(0.8, current_note, 1e-6);

        current_note = note_selector.turn_off_note(0.8);
        assert_float_eq!(0.8, current_note, 1e-6);
    }

    #[test]
    fn release_middle_note() {
        let mut note_selector = NoteSelector::new();

        note_selector.turn_on_note(0.8);
        note_selector.turn_on_note(1.2);
        let mut current_note = note_selector.turn_on_note(1.5);
        assert_float_eq!(0.8, current_note, 1e-6);

        current_note = note_selector.turn_off_note(1.2);
        assert_float_eq!(0.8, current_note, 1e-6);

        current_note = note_selector.turn_off_note(0.8);
        assert_float_eq!(1.5, current_note, 1e-6);
    }
}
