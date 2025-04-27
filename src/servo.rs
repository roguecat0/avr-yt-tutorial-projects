use arduino_hal::hal::{pac::TC1, port::*};
use arduino_hal::port::{Pin, mode::Output};

pub enum Servo {
    D10(Pin<Output, PB2>),
    D9(Pin<Output, PB1>),
}

impl Servo {
    /// angle from 0 to 180
    pub fn set_angle(&mut self, angle: u8, tc1: &TC1) {
        let duty = (100 + (660 - 100) * (angle as u32) / 180) as u16;
        match self {
            Self::D9(_) => tc1.ocr1a.write(|w| w.bits(duty)),
            Self::D10(_) => tc1.ocr1b.write(|w| w.bits(duty)),
        }
    }
    pub fn free(self, tc1: &TC1) -> Self {
        match self {
            Self::D9(_) => tc1.tccr1a.write(|w| w.com1a().disconnected()),
            Self::D10(_) => tc1.tccr1a.write(|w| w.com1b().disconnected()),
        };
        self
    }
}

pub trait IntoServo {
    fn into_servo(self, tc1: &TC1) -> Servo;
}
impl IntoServo for Pin<Output, PB1> {
    fn into_servo(self, tc1: &TC1) -> Servo {
        tc1.icr1.write(|w| w.bits(4999));
        let mut register = tc1.tccr1a.read().bits();
        register |= 0b10000010;
        tc1.tccr1a.write(|w| unsafe { w.bits(register) });
        tc1.tccr1b
            .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
        Servo::D9(self)
    }
}

impl IntoServo for Pin<Output, PB2> {
    fn into_servo(self, tc1: &TC1) -> Servo {
        tc1.icr1.write(|w| w.bits(4999));
        let mut register = tc1.tccr1a.read().bits();
        register |= 0b00100010;
        tc1.tccr1a.write(|w| unsafe { w.bits(register) });
        tc1.tccr1b
            .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
        Servo::D10(self)
    }
}
