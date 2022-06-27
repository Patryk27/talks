use arduino_hal::{
    delay_ms,
    hal::port::PB3,
    pac::{tc2::tccr2b, TC2},
    port::{mode::Output, Pin},
};

pub struct Piezo {
    tc2: TC2,
    _pin: Pin<Output, PB3>,
}

impl Piezo {
    pub fn new(tc2: TC2, pin: Pin<Output, PB3>) -> Self {
        Self { tc2, _pin: pin }
    }

    /// Plays note of given frequency for `duration` milliseconds.
    ///
    /// # Implementation
    ///
    /// Playing the note requires generating a certain wave - in our case, we're
    /// generating a square wave using AVR's Timer2's PWM facility.
    ///
    /// So what happens here is that we:
    ///
    /// - take the frequency (e.g. 567 Hz),
    ///
    /// - count how many iterations the AVR timer has to tick to count _up to_
    ///   that frequency (e.g. 16 MHz / 567 Hz / 256¹ ~= 110),
    ///
    /// - setup the timer for that value and tell it to "count down to zero and
    ///   after you do, please toggle this pin and start over",
    ///
    /// - wait for `duration` milliseconds (while the timer toggles the pin
    ///   up/down/up/down in the background automatically),
    ///
    /// - shut the timer down.
    ///
    /// Overall, that's a relatively popular method of generating square waves
    /// on AVRs, so feel free to Google for `avr generate sound` to get a better
    /// explanation.
    ///
    /// ¹ that's the timer's prescaler - in our case, to get better sound, it's
    ///   not hard-coded to 256 but rather computed automatically
    pub fn play(&mut self, frequency: u16, duration: u16) {
        let (prescaler, ticks) = if let Some(v) = Self::analyze(frequency) {
            v
        } else {
            // Error: Note has too low / too high frequency
            return;
        };

        self.tc2.tccr2a.write(|w| {
            let w = w.wgm2().pwm_phase();
            let w = w.com2a().match_toggle();
            w
        });

        self.tc2.tccr2b.write(|w| {
            let w = prescaler.apply(w.cs2());
            let w = w.wgm22().set_bit();
            w
        });

        self.tc2.ocr2a.write(|w| unsafe { w.bits(ticks.0) });
        self.tc2.tcnt2.write(|w| unsafe { w.bits(0) });

        delay_ms(duration);

        self.tc2.tccr2a.modify(|_, w| w.com2a().disconnected());
    }

    /// Analyzes given frequency and returns timer settings should be used to
    /// play that note in the most, uhm, aesthetic way.
    ///
    /// This function returns timer prescaler (e.g. 256) and number of ticks for
    /// which the piezo's pin has to be up/down to play the note.
    fn analyze(frequency: u16) -> Option<(TimerPrescaler, TimerCounter)> {
        for prescaler in TimerPrescaler::all() {
            if let Some(ticks) = prescaler.ticks_needed_to_generate_frequency(frequency) {
                return Some((prescaler, ticks));
            }
        }

        None
    }
}

#[derive(Clone, Copy)]
enum TimerPrescaler {
    Prescale1,
    Prescale8,
    Prescale32,
    Prescale64,
    Prescale128,
    Prescale256,
    Prescale1024,
}

impl TimerPrescaler {
    fn all() -> impl Iterator<Item = Self> {
        [
            Self::Prescale1,
            Self::Prescale8,
            Self::Prescale32,
            Self::Prescale64,
            Self::Prescale128,
            Self::Prescale256,
            Self::Prescale1024,
        ]
        .into_iter()
    }

    fn prescale(self) -> u32 {
        match self {
            Self::Prescale1 => 1,
            Self::Prescale8 => 8,
            Self::Prescale32 => 32,
            Self::Prescale64 => 64,
            Self::Prescale128 => 128,
            Self::Prescale256 => 256,
            Self::Prescale1024 => 1024,
        }
    }

    fn ticks_needed_to_generate_frequency(self, frequency: u16) -> Option<TimerCounter> {
        const AVR_CLOCK: u32 = 16_000_000;

        /// Number of times the AVR's timer has to fire for the speaker to
        /// receive a single square wave.
        ///
        /// This is equal to 2, because we're using `.match_toggle()` - meaning
        /// that with each timer's firing, the speaker's pin gets _toggled_, and
        /// so it takes two timer's firings for the pin to go low->high->low.
        const TIMER_FIRES_NEEDED_TO_GENERATE_SINGLE_WAVE: u32 = 2;

        /// Because we're using _phase correct PWM mode_, we have to account for
        /// the fact that timer counts not only from 0 to OCR2A, but also back
        /// (from OCR2A to 0) - which means that to generate a certain square
        /// wave, we have to use half as many ticks as in the _fast PWM mode_.
        const PHASE_CORRECT_PWM_FACTOR: u32 = 2;

        let ticks = AVR_CLOCK
            / self.prescale()
            / TIMER_FIRES_NEEDED_TO_GENERATE_SINGLE_WAVE
            / PHASE_CORRECT_PWM_FACTOR
            / (frequency as u32);

        ticks.try_into().ok().map(TimerCounter)
    }

    fn apply(self, cs: tccr2b::CS2_W<'_>) -> &mut tccr2b::W {
        match self {
            Self::Prescale1 => cs.direct(),
            Self::Prescale8 => cs.prescale_8(),
            Self::Prescale32 => cs.prescale_32(),
            Self::Prescale64 => cs.prescale_64(),
            Self::Prescale128 => cs.prescale_128(),
            Self::Prescale256 => cs.prescale_256(),
            Self::Prescale1024 => cs.prescale_1024(),
        }
    }
}

struct TimerCounter(u8);
