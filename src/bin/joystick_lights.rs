#![no_main]
#![no_std]

use arduino_hal::{
    prelude::*,
    simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer2Pwm},
};
use panic_halt as _;
use yt_tutorial_projects::pwm_enum::{IntoPwmUno, PwmUno};

const RANGE: u32 = 650;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let tc0_pwm = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let tc2_pwm = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut p11 = pins.d11.into_output().into_pwm(&tc2_pwm).into_pwm_uno();
    let mut p6 = pins.d6.into_output().into_pwm(&tc0_pwm).into_pwm_uno();
    let mut p5 = pins.d5.into_output().into_pwm(&tc0_pwm).into_pwm_uno();
    let mut p3 = pins.d3.into_output().into_pwm(&tc2_pwm).into_pwm_uno();
    p11.set_duty(0);
    p6.set_duty(1);
    p5.set_duty(100);
    p3.set_duty(250);
    // visually pleasing coordinates
    let mut pwm_coordinates: [(PwmUno, (u16, u16)); 4] = [
        (p11, (0, 0)),
        (p6, (1023, 0)),
        (p5, (0, 1023)),
        (p3, (1023, 1023)),
    ];

    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    loop {
        let read_x = a0.analog_read(&mut adc);
        let read_y = a1.analog_read(&mut adc);
        for (pwm, (x, y)) in pwm_coordinates.iter_mut() {
            let x = *x;
            let y = *y;
            let diff_x = if read_x > x { read_x - x } else { x - read_x } as u32;
            let diff_y = if read_y > y { read_y - y } else { y - read_y } as u32;
            if diff_x >= RANGE || diff_y >= RANGE {
                pwm.set_duty(1);
                continue;
            }
            let distance = (diff_x.pow(2) + diff_y.pow(2)).isqrt();
            if distance > RANGE {
                pwm.set_duty(1);
            } else {
                let duty = core::cmp::min(255 - (255 * distance / RANGE), 255) as u8;
                ufmt::uwriteln!(
                    serial,
                    "xval: {}, yval: {}, distance: {}, duty: {}",
                    read_x,
                    read_y,
                    distance,
                    duty
                )
                .unwrap_infallible();
                pwm.set_duty(duty);
            }
        }
        arduino_hal::delay_ms(100);
    }
}
