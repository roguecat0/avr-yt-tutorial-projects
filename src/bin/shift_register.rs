#![no_main]
#![no_std]

use arduino_hal::{
    hal::port::Dynamic,
    port::{Pin, mode::Output},
    prelude::_unwrap_infallible_UnwrapInfallible,
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut latch_pin = pins.d2.into_output().downgrade();
    let mut clock_pin = pins.d3.into_output().downgrade();
    let mut input_pin = pins.d4.into_output().downgrade();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    loop {
        let mut text: [u8; 3] = [0x30; 3];
        let mut i = 0;
        loop {
            let byte = serial.read_byte();
            if byte == 10 {
                break;
            }
            if i < 3 {
                text[2] = text[1];
                text[1] = text[0];
                text[0] = byte;
            }
            i += 1;
        }
        ufmt::uwriteln!(serial, "text: [{}, {}, {}]", text[2], text[1], text[0])
            .unwrap_infallible();
        match parse_number(&text) {
            Err(e) => {
                ufmt::uwriteln!(serial, "{}", e).unwrap_infallible();
            }
            Ok(n) => {
                ufmt::uwriteln!(serial, "number: {}", n).unwrap_infallible();
                pulse(&mut latch_pin);
                shift_out(&mut clock_pin, &mut input_pin, (n & 255) as u8);
                pulse(&mut latch_pin);
            }
        }
    }
}
type DynPin = Pin<Output, Dynamic>;
fn shift_out(clock_pin: &mut DynPin, input_pin: &mut DynPin, byte: u8) {
    for i in (0..8).rev() {
        let is_high = (byte >> i) & 1 == 1;
        if is_high {
            input_pin.set_high();
        } else {
            input_pin.set_low();
        }
        pulse(clock_pin);
    }
}
fn parse_number(arr: &[u8; 3]) -> Result<u32, &'static str> {
    let mut number = 0;
    for (i, &c) in arr.iter().enumerate() {
        let d: char = c.into();
        let d = d.to_digit(10).ok_or("not a valid digit")?;
        number += d * 10_u32.pow(i as u32);
    }
    Ok(number)
}
#[inline]
fn pulse(dyn_pin: &mut DynPin) {
    dyn_pin.set_high();
    dyn_pin.set_low();
}
