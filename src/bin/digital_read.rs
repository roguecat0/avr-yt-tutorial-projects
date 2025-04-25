#![no_main]
#![no_std]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let dread_pin = pins.a0.into_floating_input();
    loop {
        ufmt::uwriteln!(serial, "dread_pin high: {}", dread_pin.is_high()).unwrap_infallible();
        arduino_hal::delay_ms(50);
    }
}
