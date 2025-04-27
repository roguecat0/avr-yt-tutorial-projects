#![no_main]
#![no_std]

use arduino_hal::{prelude::_unwrap_infallible_UnwrapInfallible, simple_pwm::*};
use panic_halt as _;
const RANGE: usize = 255;
const ARR_SIZE: usize = RANGE * 2;
const DELAY: u32 = 50;
const TOTAL_LIGHT: u32 = 255;
const RED_RICO: u32 = 9;
const GREEN_RICO: u32 = 19;

#[inline]
fn brightness_plot(val: u32) -> u8 {
    let val = val % ARR_SIZE as u32;
    if val <= TOTAL_LIGHT as u32 {
        val as u8
    } else {
        (2 * TOTAL_LIGHT - val) as u8
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut red_led = pins.d11.into_output().into_pwm(&timer2);
    let mut green_led = pins.d10.into_output().into_pwm(&timer1);
    let mut blue_led = pins.d9.into_output().into_pwm(&timer1);
    red_led.enable();
    blue_led.enable();
    green_led.enable();
    let mut i = 0;
    // todo: better use YIQ color space
    loop {
        let red = brightness_plot(i * RED_RICO);
        let blue =
            (((255 - red) as f32 / 255 as f32) * brightness_plot(i * GREEN_RICO) as f32) as u8;
        let green = (TOTAL_LIGHT - red as u32 - blue as u32) as u8;
        red_led.set_duty(red);
        blue_led.set_duty(blue);
        green_led.set_duty(green);
        arduino_hal::delay_ms(DELAY);
        i += 1;
    }
}
