#![no_main]
#![no_std]

use arduino_hal::simple_pwm::*;
use core::f32::consts;
use micromath::F32Ext;
use panic_halt as _;
use yt_tutorial_projects::PwmUno;

const PERIOD: u32 = 1000;
const DELAY: u32 = 50;
const INTERVAL: u32 = PERIOD / DELAY;
const INTERVAL_TAU: f32 = 1.0 / (INTERVAL as f32) * consts::TAU;
#[inline]
pub fn to_light_val(f: f32) -> u8 {
    // only +0.5 to make the leds dim for longer
    (((f + 0.5) * 127.0) as u8).saturating_sub(1)
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let l1 = pins.d11.into_output().into_pwm(&mut timer2);
    let l2 = pins.d10.into_output().into_pwm(&mut timer1);
    let l3 = pins.d9.into_output().into_pwm(&mut timer1);
    let l4 = pins.d6.into_output().into_pwm(&mut timer0);
    let l5 = pins.d5.into_output().into_pwm(&mut timer0);
    let l6 = pins.d3.into_output().into_pwm(&mut timer2);

    let mut pwn_pin_vals = [
        (PwmUno::D11(l1), 0.0),
        (PwmUno::D10(l2), consts::FRAC_PI_8),
        (PwmUno::D9(l3), consts::FRAC_PI_8 * 2.0),
        (PwmUno::D6(l4), consts::FRAC_PI_8 * 3.0),
        (PwmUno::D5(l5), consts::FRAC_PI_8 * 4.0),
        (PwmUno::D4(l6), consts::FRAC_PI_8 * 5.0),
    ];
    for (pwm, _) in pwn_pin_vals.iter_mut() {
        pwm.enable();
    }
    loop {
        for (pwm, val) in pwn_pin_vals.iter_mut() {
            *val += INTERVAL_TAU;
            let sinm: f32 = val.sin();
            let led_light = to_light_val(sinm);
            pwm.set_duty(led_light);
        }
        arduino_hal::delay_ms(DELAY);
    }
}
