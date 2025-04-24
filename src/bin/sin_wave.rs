#![no_main]
#![no_std]

use arduino_hal::hal::port::*;
use arduino_hal::port::{Pin, mode::PwmOutput};
use arduino_hal::{prelude::*, simple_pwm::*};
use core::f32::consts;

use libm::sinf;
use panic_halt as _;

const PERIOD: u32 = 800;
const DELAY: u32 = 50;
const INTERVAL: u32 = PERIOD / DELAY;
const INTERVAL_TAU: f32 = 1.0 / (INTERVAL as f32) * consts::TAU;
#[inline]
pub fn to_light_val(f: f32) -> u8 {
    (((f + 0.7) * 127.0) as u8) //.saturating_sub(1)
}
enum PwmEnum {
    Pwm1(Pin<PwmOutput<Timer2Pwm>, PB3>),
    Pwm2(Pin<PwmOutput<Timer1Pwm>, PB2>),
    Pwm3(Pin<PwmOutput<Timer1Pwm>, PB1>),
    Pwm4(Pin<PwmOutput<Timer0Pwm>, PD6>),
    Pwm5(Pin<PwmOutput<Timer0Pwm>, PD5>),
    Pwm6(Pin<PwmOutput<Timer2Pwm>, PD3>),
}
impl PwmEnum {
    pub fn set_duty(&mut self, duty: u8) {
        match self {
            PwmEnum::Pwm1(pwm) => pwm.set_duty(duty),
            PwmEnum::Pwm2(pwm) => pwm.set_duty(duty),
            PwmEnum::Pwm3(pwm) => pwm.set_duty(duty),
            PwmEnum::Pwm4(pwm) => pwm.set_duty(duty),
            PwmEnum::Pwm5(pwm) => pwm.set_duty(duty),
            PwmEnum::Pwm6(pwm) => pwm.set_duty(duty),
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let mut l1 = pins.d11.into_output_high().into_pwm(&mut timer2);
    let mut l2 = pins.d10.into_output_high().into_pwm(&mut timer1);
    let mut l3 = pins.d9.into_output_high().into_pwm(&mut timer1);
    let mut l4 = pins.d6.into_output_high().into_pwm(&mut timer0);
    let mut l5 = pins.d5.into_output_high().into_pwm(&mut timer0);
    let mut l6 = pins.d3.into_output_high().into_pwm(&mut timer2);
    l1.enable();
    l2.enable();
    l3.enable();
    l4.enable();
    l5.enable();
    l6.enable();
    let mut values = [
        0.0,
        consts::FRAC_PI_8,
        consts::FRAC_PI_8 * 2.0,
        consts::FRAC_PI_8 * 3.0,
        consts::FRAC_PI_8 * 4.0,
        consts::FRAC_PI_8 * 5.0,
    ];
    let mut pwm_pins = [
        PwmEnum::Pwm1(l1),
        PwmEnum::Pwm2(l2),
        PwmEnum::Pwm3(l3),
        PwmEnum::Pwm4(l4),
        PwmEnum::Pwm5(l5),
        PwmEnum::Pwm6(l6),
    ];
    loop {
        for i in 0..values.len() {
            *values.get_mut(i).unwrap() += INTERVAL_TAU;
            let led_light = to_light_val(sinf(values[i]));
            pwm_pins.get_mut(i).unwrap().set_duty(led_light);
        }
        arduino_hal::delay_ms(DELAY);
    }
}
