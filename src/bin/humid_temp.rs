#![no_main]
#![no_std]

use arduino_hal::{delay_ms, prelude::_unwrap_infallible_UnwrapInfallible};
use dht11::{Dht11, Error as DHError};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut delay = arduino_hal::Delay::new();
    let pin = pins.d3.into_opendrain();
    let mut ht11 = Dht11::new(pin);
    loop {
        match ht11.perform_measurement(&mut delay) {
            Ok(meas) => ufmt::uwriteln!(
                serial,
                "Ok, tmp: {}, hum: {}",
                meas.temperature,
                meas.humidity
            )
            .unwrap_infallible(),
            Err(e) => match e {
                DHError::Timeout => ufmt::uwriteln!(serial, "Error: timeout").unwrap_infallible(),
                DHError::CrcMismatch => {
                    ufmt::uwriteln!(serial, "Error: CrcMissmatch").unwrap_infallible()
                }
                _ => ufmt::uwriteln!(serial, "GPIO Error").unwrap_infallible(),
            },
        }
        delay_ms(100);
    }
}
