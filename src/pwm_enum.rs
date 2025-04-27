use arduino_hal::hal::port::*;
use arduino_hal::port::{Pin, mode::PwmOutput};
use arduino_hal::simple_pwm::*;

macro_rules! set_duty {
    ($pwm:expr, $duty:expr) => {{
        if $duty == 0 {
            $pwm.disable();
        } else {
            $pwm.enable();
        }
        $pwm.set_duty($duty)
    }};
}
macro_rules! into_pwm_uno {
    [$enum:tt, $timer:ty, $port:ty] => {
        impl IntoPwmUno for Pin<PwmOutput<$timer>, $port> {
            fn into_pwm_uno(self) -> PwmUno {
                PwmUno::$enum(self)
            }
        }
    };
}
into_pwm_uno!(D11, Timer2Pwm, PB3);
into_pwm_uno!(D10, Timer1Pwm, PB2);
into_pwm_uno!(D9, Timer1Pwm, PB1);
into_pwm_uno!(D6, Timer0Pwm, PD6);
into_pwm_uno!(D5, Timer0Pwm, PD5);
into_pwm_uno!(D4, Timer2Pwm, PD3);
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
            PwmUno::D11(pwm) => set_duty!(pwm, duty),
            PwmUno::D10(pwm) => set_duty!(pwm, duty),
            PwmUno::D9(pwm) => set_duty!(pwm, duty),
            PwmUno::D6(pwm) => set_duty!(pwm, duty),
            PwmUno::D5(pwm) => set_duty!(pwm, duty),
            PwmUno::D4(pwm) => set_duty!(pwm, duty),
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
pub trait IntoPwmUno {
    fn into_pwm_uno(self) -> PwmUno;
}
