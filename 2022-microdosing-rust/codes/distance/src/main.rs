#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut trig = pins.d9.into_output();
    let echo = pins.d8.into_floating_input();

    loop {
        trig.set_high();
        arduino_hal::delay_us(15);
        trig.set_low();

        while echo.is_low() {
            //
        }

        let mut us = 0;

        while echo.is_high() {
            us += 1;
            arduino_hal::delay_us(1);
        }

        let dist = us / 58;

        ufmt::uwriteln!(&mut serial, "dist = {} cm\r", dist).void_unwrap();
        arduino_hal::delay_ms(100);
    }
}
