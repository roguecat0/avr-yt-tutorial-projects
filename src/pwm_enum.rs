use arduino_hal::hal::port::*;
use arduino_hal::port::{Pin, mode::PwmOutput};
use arduino_hal::simple_pwm::*;

pub enum PwmUno {
    D11(Pin<PwmOutput<Timer2Pwm>, PB3>),
    D10(Pin<PwmOutput<Timer1Pwm>, PB2>),
    D9(Pin<PwmOutput<Timer1Pwm>, PB1>),
    D6(Pin<PwmOutput<Timer0Pwm>, PD6>),
    D5(Pin<PwmOutput<Timer0Pwm>, PD5>),
    D4(Pin<PwmOutput<Timer2Pwm>, PD3>),
}
impl PwmUno {
    pub fn set_duty(&mut self, duty: u8) {
        match self {
            PwmUno::D11(pwm) => pwm.set_duty(duty),
            PwmUno::D10(pwm) => pwm.set_duty(duty),
            PwmUno::D9(pwm) => pwm.set_duty(duty),
            PwmUno::D6(pwm) => pwm.set_duty(duty),
            PwmUno::D5(pwm) => pwm.set_duty(duty),
            PwmUno::D4(pwm) => pwm.set_duty(duty),
        }
    }
    pub fn enable(&mut self) {
        match self {
            PwmUno::D11(pwm) => pwm.enable(),
            PwmUno::D10(pwm) => pwm.enable(),
            PwmUno::D9(pwm) => pwm.enable(),
            PwmUno::D6(pwm) => pwm.enable(),
            PwmUno::D5(pwm) => pwm.enable(),
            PwmUno::D4(pwm) => pwm.enable(),
        }
    }
}
