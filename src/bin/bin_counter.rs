#![no_main]
#![no_std]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut p1 = pins.d2.into_output();
    let mut p2 = pins.d3.into_output();
    let mut p3 = pins.d4.into_output();
    let mut number = 0;
    p1.set_low();
    p2.set_low();
    p3.set_low();
    loop {
        if number & 0b1 != 0 {
            p1.set_high();
        } else {
            p1.set_low();
        }
        if number & 0b10 != 0 {
            p2.set_high();
        } else {
            p2.set_low();
        }
        if number & 0b100 != 0 {
            p3.set_high();
        } else {
            p3.set_low();
        }
        arduino_hal::delay_ms(500);
        number += 1;
        if number >= 8 {
            number = 0;
        }
    }
}
