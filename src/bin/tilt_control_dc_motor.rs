#![no_main]
#![no_std]

use arduino_hal::{
    port::{Pin, mode::PwmOutput},
    simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps, Timer2Pwm},
};
use panic_halt as _;

#[derive(Clone, Copy)]
enum DCStates {
    Clockwise,
    CounterClockwise,
    Off,
}
impl DCStates {
    fn reverse(self) -> Self {
        match self {
            Self::Clockwise => Self::CounterClockwise,
            Self::CounterClockwise => Self::Clockwise,
            state => state,
        }
    }
    fn to_pinstate(&self) -> (bool, bool) {
        match self {
            Self::Clockwise => (true, false),
            Self::CounterClockwise => (false, true),
            Self::Off => (false, false),
        }
    }
}
fn boosted_set_duty<PIN: PwmPinOps<TC>, TC>(pin: &mut Pin<PwmOutput<TC>, PIN>, duty: u8) {
    pin.set_duty(255);
    arduino_hal::delay_ms(200);
    pin.set_duty(duty);
}
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let tilt_pin = pins.d6.into_pull_up_input();
    let mut in_pins = (
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
    );

    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut pwm_11 = pins.d11.into_output().into_pwm(&timer2);
    pwm_11.enable();
    const DUTY: u8 = 150;
    boosted_set_duty(&mut pwm_11, DUTY);
    let mut prev_state = tilt_pin.is_high();
    in_pins.0.set_high();
    in_pins.1.set_low();
    loop {
        if tilt_pin.is_high() != prev_state {
            prev_state = tilt_pin.is_high();
            in_pins.0.toggle();
            in_pins.1.toggle();
            boosted_set_duty(&mut pwm_11, DUTY);
        }
        arduino_hal::delay_ms(500);
    }
}
