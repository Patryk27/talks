#![no_std]
#![no_main]
#![recursion_limit = "512"]

#[macro_use]
mod track;

mod piezo;
mod tracks;

use self::{piezo::*, track::*};
use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut piezo = Piezo::new(dp.TC2, pins.d11.into_output());
    let mut led = pins.d13.into_output();
    let pc0 = pins.a0.into_opendrain();
    let pc1 = pins.a1.into_opendrain();
    let pc2 = pins.a2.into_opendrain();
    let pc3 = pins.a3.into_opendrain();

    if pc0.is_high() {
        tracks::never_gonna_give_you_up::TRACK.play(&mut piezo);
    } else if pc1.is_high() {
        tracks::the_model::TRACK.play(&mut piezo);
    } else if pc2.is_high() {
        tracks::mial_byc_slub::TRACK.play(&mut piezo);
    } else if pc3.is_high() {
        tracks::sandstorm::TRACK.play(&mut piezo);
    }

    loop {
        led.toggle();
        delay_ms(250);
    }
}
