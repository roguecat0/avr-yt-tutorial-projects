#![no_main]
#![no_std]

use arduino_hal::port::PinOps;
use avr_hal_generic::port::Pin;
use avr_hal_generic::port::mode::Output;
// use embedded_hal::digital::OutputPin;

use panic_halt as _;

#[inline]
fn show_conditional<P: PinOps>(pin: &mut Pin<Output, P>, condition: bool) {
    if condition {
        let _ = pin.set_high();
    } else {
        let _ = pin.set_low();
    }
}

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
        show_conditional(&mut p1, number & 0b1 != 0);
        show_conditional(&mut p2, number & 0b10 != 0);
        show_conditional(&mut p3, number & 0b100 != 0);
        arduino_hal::delay_ms(500);
        number += 1;
        if number >= 8 {
            number = 0;
        }
    }
}
