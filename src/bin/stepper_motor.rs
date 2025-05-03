#![no_main]
#![no_std]

use arduino_hal::{Delay, prelude::_unwrap_infallible_UnwrapInfallible};
use panic_halt as _;
use uln2003::{StepperMotor, ULN2003};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let delay = Delay::new();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let p8 = pins.d8.into_output();
    let p10 = pins.d10.into_output();
    let p9 = pins.d9.into_output();
    let p11 = pins.d11.into_output();
    let mut driver = ULN2003::new(p8, p10, p9, p11, Some(delay));
    if let Err(_) = driver.step_for(4096, 4) {
        ufmt::uwriteln!(serial, "step error").unwrap_infallible();
    } else {
        ufmt::uwriteln!(serial, "stepped").unwrap_infallible();
    }
    loop {}
}
