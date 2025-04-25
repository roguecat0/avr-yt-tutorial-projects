#![no_main]
#![no_std]

use panic_halt as _;
const TICK: u32 = 50;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let passive_note = 5000;
    let active_note = 1000;
    let mut passive_buzz = pins.d3.into_output();
    let mut active_buzz = pins.d2.into_output();
    let mut passive_counter = 0;
    let mut active_counter = 0;
    let passive_max = passive_note / TICK;
    let active_max = active_note / TICK;
    loop {
        if passive_counter >= passive_max {
            passive_buzz.toggle();
            passive_counter = 0;
        }
        if active_counter >= active_max {
            active_buzz.toggle();
            active_counter = 0;
        }
        passive_counter += 1;
        active_counter += 1;
        arduino_hal::delay_us(TICK);
    }
}
