#![no_main]
#![no_std]

use arduino_hal::prelude::*;
use panic_halt as _;

const VT: u32 = 5000;
const R1: u32 = 5000;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let a0 = pins.a0.into_analog_input(&mut adc);
    loop {
        let value = a0.analog_read(&mut adc);
        let v1 = value as u32 * VT / 1024;
        let rr = R1 * VT / v1 - R1;
        ufmt::uwriteln!(serial, "val: {}, mV: {}, Rr: {}       \r", value, v1, rr)
            .unwrap_infallible();
        arduino_hal::delay_ms(100);
    }
}
