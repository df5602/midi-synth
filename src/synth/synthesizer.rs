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

const NUMBER_OF_NOTES: usize = 32;

#[derive(Copy, Clone)]
struct Note {
    note: f32,
}

struct NoteSelector {
    notes: [Note; NUMBER_OF_NOTES],
    current_note: f32,
    number_of_notes: usize,
}

/// Selects the lowest of the currently playing notes (low note priority).
///
/// Notes are stored in a binary heap for performance reasons, i.e. the lowest
/// note is at the root (index 1).
impl NoteSelector {
    fn new() -> Self {
        Self {
            notes: [Note {
                note: f32::INFINITY,
            }; NUMBER_OF_NOTES],
            current_note: f32::INFINITY,
            number_of_notes: 0,
        }
    }

    /// Inserts new note, returns the lowest note.
    fn turn_on_note(&mut self, note: f32) -> f32 {
        if self.number_of_notes + 1 < NUMBER_OF_NOTES {
            self.number_of_notes += 1;
        }

        // Insert new note at next free slot (or reuse last slot if no free slots)
        self.notes[self.number_of_notes].note = note;

        let mut pos = self.number_of_notes;
        let mut parent = pos / 2;

        // Min Heap: Swap elements until parent node is smaller or equal all its child nodes
        while parent > 0 {
            if self.notes[parent].note > self.notes[pos].note {
                self.notes.swap(parent, pos);
            } else {
                break;
            }

            pos = parent;
            parent = pos / 2;
        }

        // Lowest note is always stored at index 1
        self.current_note = self.notes[1].note;
        self.current_note
    }

    /// Removes note from list of playing notes, returns lowest note.
    fn turn_off_note(&mut self, note: f32) -> f32 {
        // Perform linear search for note to turn off
        let mut pos = 0;
        for i in 1..self.number_of_notes + 1 {
            if (note - self.notes[i].note).abs() < 1e-6 {
                pos = i;
                break;
            }
        }

        // If found...
        if pos > 0 {
            // .. swap note to remove with note at last position
            self.notes[pos].note = self.notes[self.number_of_notes].note;
            self.notes[self.number_of_notes].note = f32::INFINITY;
            self.number_of_notes -= 1;

            let mut left_child = 2 * pos;
            let mut right_child = left_child + 1;

            // Min heap: Swap parent node with smaller of its children, until parent node is smaller
            // or equal than both its child nodes
            loop {
                if right_child >= NUMBER_OF_NOTES {
                    break;
                }

                if self.notes[pos].note <= self.notes[left_child].note
                    && self.notes[pos].note <= self.notes[right_child].note
                {
                    break;
                }

                if self.notes[left_child].note <= self.notes[right_child].note {
                    self.notes.swap(pos, left_child);
                    pos = left_child;
                } else {
                    self.notes.swap(pos, right_child);
                    pos = right_child;
                }

                left_child = 2 * pos;
                right_child = left_child + 1;
            }
        }

        // Lowest note is always stored at index 1 (expect if no note is playing), in that
        // case return the currently playing note
        self.current_note = if self.notes[1].note < f32::INFINITY {
            self.notes[1].note
        } else {
            self.current_note
        };
        self.current_note
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

#[cfg(all(feature = "benchmarks", test))]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn few_notes_1(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        note_selector.turn_on_note(2.0);
        note_selector.turn_on_note(1.6);
        note_selector.turn_on_note(1.2);

        b.iter(|| {
            note_selector.turn_on_note(0.8);

            note_selector.turn_off_note(0.8);
        })
    }

    #[bench]
    fn few_notes_2(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        note_selector.turn_on_note(2.0);
        note_selector.turn_on_note(1.6);
        note_selector.turn_on_note(1.2);

        b.iter(|| {
            note_selector.turn_on_note(3.0);

            note_selector.turn_off_note(3.0);
        })
    }

    #[bench]
    fn filled_one_quarter_1(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..NUMBER_OF_NOTES / 4 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(0.2);

            note_selector.turn_off_note(0.2);
        })
    }

    #[bench]
    fn filled_one_quarter_2(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..NUMBER_OF_NOTES / 4 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(40.0);

            note_selector.turn_off_note(40.0);
        })
    }

    #[bench]
    fn filled_half_1(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..NUMBER_OF_NOTES / 2 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(0.2);

            note_selector.turn_off_note(0.2);
        })
    }

    #[bench]
    fn filled_half_2(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..NUMBER_OF_NOTES / 2 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(40.0);

            note_selector.turn_off_note(40.0);
        })
    }

    #[bench]
    fn filled_half_3(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..NUMBER_OF_NOTES / 2 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(4.1);

            note_selector.turn_off_note(4.1);
        })
    }

    #[bench]
    fn filled_three_quarters_1(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..3 * NUMBER_OF_NOTES / 4 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(0.2);

            note_selector.turn_off_note(0.2);
        })
    }

    #[bench]
    fn filled_three_quarters_2(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..3 * NUMBER_OF_NOTES / 4 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(40.0);

            note_selector.turn_off_note(40.0);
        })
    }

    #[bench]
    fn filled_three_quarters_3(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..3 * NUMBER_OF_NOTES / 4 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(6.2);

            note_selector.turn_off_note(6.2);
        })
    }

    #[bench]
    fn many_notes_1(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..NUMBER_OF_NOTES - 1 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(0.2);

            note_selector.turn_off_note(0.2);
        })
    }

    #[bench]
    fn many_notes_2(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..NUMBER_OF_NOTES - 1 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(40.0);

            note_selector.turn_off_note(40.0);
        })
    }

    #[bench]
    fn many_notes_3(b: &mut Bencher) {
        let mut note_selector = NoteSelector::new();

        for i in 0..NUMBER_OF_NOTES - 1 {
            note_selector.turn_on_note(((i + 1) as f32) * 0.5);
        }

        b.iter(|| {
            note_selector.turn_on_note(8.2);

            note_selector.turn_off_note(8.2);
        })
    }
}
