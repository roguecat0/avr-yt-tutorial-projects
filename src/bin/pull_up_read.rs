#![no_main]
#![no_std]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let p2 = pins.d2.into_pull_up_input();
    loop {
        let val = p2.is_high();
        ufmt::uwriteln!(serial, "val: {}", val).unwrap();
        arduino_hal::delay_ms(50);
    }
}
