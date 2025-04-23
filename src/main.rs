#![no_main]
#![no_std]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    loop {}
}
