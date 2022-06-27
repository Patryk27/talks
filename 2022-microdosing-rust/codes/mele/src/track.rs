use crate::Piezo;
use arduino_hal::delay_ms;
use avr_progmem::wrapper::ProgMem;

pub struct Track<const LEN: usize> {
    pub opcodes: &'static ProgMem<[Opcode; LEN]>,
}

impl<const LEN: usize> Track<LEN> {
    pub fn play(&self, piezo: &mut Piezo) {
        for i in 0..LEN {
            self.opcodes.load_at(i).run(piezo);
        }
    }
}

#[derive(Clone, Copy)]
pub enum Opcode {
    Note {
        frequency: u16,
        duration: u16,
    },
    NoteWithCooldown {
        frequency: u16,
        duration: u16,
        cooldown: u16,
    },
    Pause {
        duration: u16,
    },
}

impl Opcode {
    fn run(self, piezo: &mut Piezo) {
        match self {
            Opcode::Note {
                frequency,
                duration,
            } => {
                piezo.play(frequency, duration);
            }

            Opcode::NoteWithCooldown {
                frequency,
                duration,
                cooldown,
            } => {
                piezo.play(frequency, duration - cooldown);
                delay_ms(cooldown);
            }

            Opcode::Pause { duration } => {
                delay_ms(duration);
            }
        }
    }
}

/// Converts BPM into milliseconds per quarter note.
pub const fn bpm_to_quarter_ms(bpm: u16) -> u16 {
    60 * 1000 / bpm
}

macro_rules! track {
    ([ $( $opcode_name:tt $opcode_args:tt ),* $(,)? ]) => {
        pub static TRACK: Track<{ track!(@count_opcodes $( $opcode_name )*) }> = Track {
            opcodes: &TRACK_OPCODES,
        };

        // We've gotta use progmem, since all the tracks don't fit in AVR's RAM
        avr_progmem::progmem! {
            pub static progmem<const LEN: usize> TRACK_OPCODES: [Opcode; LEN] = [
                $(
                    track!(@opcode $opcode_name $opcode_args),
                )+
            ];
        }
    };

    (@count_opcodes $opcode:tt) => {
        1
    };

    (@count_opcodes $opcode:tt $( $another_opcode:tt )+) => {
        1 + track!(@count_opcodes $( $another_opcode )+)
    };

    // Plays `$note` for `$duration` ms
    (@opcode note($note:tt, $duration:expr)) => {
        Opcode::Note {
            frequency: track!(@note_frequency $note),
            duration: $duration,
        }
    };

    // Plays `$note` for `$duration-$cooldown` ms and pauses for `$cooldown` ms
    (@opcode note($note:tt, $duration:expr, $cooldown:expr)) => {
        Opcode::NoteWithCooldown {
            frequency: track!(@note_frequency $note),
            duration: $duration,
            cooldown: $cooldown,
        }
    };

    (@opcode pause($duration:expr)) => {
        Opcode::Pause {
            duration: $duration,
        }
    };

    // Frequencies taken from:
    // https://pages.mtu.edu/~suits/notefreqs.html
    //
    // Note that ideally we'd generate them through some `const fn`, but my
    // attempts at that have gracefully failed :-P

    (@note_frequency "E2") => { 82 };
    (@note_frequency "F2") => { 87 };
    (@note_frequency "F#2") => { 92 };
    (@note_frequency "G2") => { 98 };
    (@note_frequency "G#2") => { 103 };
    (@note_frequency "A2") => { 110 };
    (@note_frequency "A#2") => { 116 };
    (@note_frequency "B2") => { 123 };
    (@note_frequency "C3") => { 130 };
    (@note_frequency "C#3") => { 138 };
    (@note_frequency "D3") => { 146 };
    (@note_frequency "D#3") => { 155 };
    (@note_frequency "E3") => { 164 };
    (@note_frequency "F3") => { 174 };
    (@note_frequency "F#3") => { 185 };
    (@note_frequency "G3") => { 196 };
    (@note_frequency "G#3") => { 207 };
    (@note_frequency "A3") => { 220 };
    (@note_frequency "A#3") => { 233 };
    (@note_frequency "B3") => { 246 };
    (@note_frequency "C4") => { 261 };
    (@note_frequency "C#4") => { 277 };
    (@note_frequency "D4") => { 293 };
    (@note_frequency "D#4") => { 311 };
    (@note_frequency "E4") => { 329 };
    (@note_frequency "F4") => { 349 };
    (@note_frequency "F#4") => { 369 };
    (@note_frequency "G4") => { 392 };
    (@note_frequency "G#4") => { 415 };
    (@note_frequency "A4") => { 440 };
    (@note_frequency "A#4") => { 466 };
    (@note_frequency "B4") => { 493 };
    (@note_frequency "C5") => { 523 };
    (@note_frequency "C#5") => { 554 };
    (@note_frequency "D5") => { 587 };
    (@note_frequency "D#5") => { 622 };
    (@note_frequency "E5") => { 659 };
    (@note_frequency "F5") => { 698 };
    (@note_frequency "F#5") => { 739 };
    (@note_frequency "G5") => { 783 };
    (@note_frequency "G#5") => { 830 };
    (@note_frequency "A5") => { 880 };
    (@note_frequency "A#5") => { 932 };
    (@note_frequency "B5") => { 987 };
    (@note_frequency "C6") => { 1046 };
    (@note_frequency "C#6") => { 1108 };
    (@note_frequency "D6") => { 1174 };
    (@note_frequency "D#6") => { 1244 };
    (@note_frequency "E6") => { 1318 };
    (@note_frequency "F6") => { 1396 };
    (@note_frequency "F#6") => { 1479 };
    (@note_frequency "G6") => { 1567 };
    (@note_frequency "G#6") => { 1661 };
    (@note_frequency "A6") => { 1760 };
    (@note_frequency "A#6") => { 1864 };
    (@note_frequency "B6") => { 1975 };
    (@note_frequency "C7") => { 2093 };
}
