/*!
 * Sweep a standard SG90 compatible servo from its left limit all the way to its right limit.
 *
 * Because avr-hal does not have a dedicated servo driver library yet, we do this manually using
 * timer TC1.  The servo should be connected to D9 (AND D9 ONLY!  THIS DOES NOT WORK ON OTHER PINS
 * AS IT IS).
 *
 * As the limits are not precisely defined, we undershoot the datasheets 1ms left limit and
 * overshoot the 2ms right limit by a bit - you can figure out where exactly the limits are for
 * your model by experimentation.
 *
 * Connections
 * -----------
 *  - `D9`: Servo's PWM signal
 */
#![no_std]
#![no_main]

use panic_halt as _;
use yt_tutorial_projects::servo::IntoServo;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let tc1 = dp.TC1;

    let mut servo_p9 = pins.d9.into_output().into_servo(&tc1);
    let mut servo_p10 = pins.d10.into_output().into_servo(&tc1);

    loop {
        for i in 0..180 {
            servo_p9.set_angle(i, &tc1);
            servo_p10.set_angle(180 - i, &tc1);
            arduino_hal::delay_ms(20);
        }
    }
}

#[allow(dead_code)]
fn to_bin_rep(byte: u8) -> [char; 8] {
    let mut res = ['0'; 8];
    for i in 0..8 {
        if byte & (1 << i) != 0 {
            res[i] = '1'
        }
    }
    res
}
