/*!
 * Demonstration of writing to and reading from the eeprom.
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

enum Action {
    Erase,
    Write,
    Nil,
}
const ACTION: Action = Action::Nil;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut ep = arduino_hal::Eeprom::new(dp.EEPROM);
    let ep_capacity = ep.capacity();
    ufmt::uwriteln!(&mut serial, "eeprom capacity is:{}\r", ep_capacity).unwrap_infallible();

    // KNOWN ISSUE: Avoid to read entire eeprom capacity at once
    // See: https://github.com/Rahix/avr-hal/issues/410
    let mut data = [0_u8; 10];

    let _start_address: u16 = 0;
    if let Action::Write = ACTION {
        if let Err(_) = ep.write(0, &[128, 64, 32, 16, 8, 2, 1, 0]) {
            ufmt::uwriteln!(&mut serial, "erase eeprom fail:\r").unwrap_infallible();
            loop {}
        }
    }

    if ep.read(0, &mut data).is_err() {
        ufmt::uwriteln!(&mut serial, "read eeprom fail:\r").unwrap_infallible();
        loop {}
    }

    ufmt::uwriteln!(&mut serial, "Got:\r").unwrap_infallible();
    for i in data {
        ufmt::uwriteln!(&mut serial, "{}", i).unwrap_infallible();
    }

    if let Action::Erase = ACTION {
        if let Err(_) = ep.erase(0, arduino_hal::Eeprom::CAPACITY) {
            ufmt::uwriteln!(&mut serial, "erase eeprom fail:\r").unwrap_infallible();
            loop {}
        }
    }

    loop {}
}
