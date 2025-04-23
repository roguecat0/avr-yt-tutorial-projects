#![no_main]
#![no_std]

use arduino_hal::Adc;
use arduino_hal::prelude::*;
use core::cmp::max;
use panic_halt as _;

macro_rules! flip_lights {
    ($p1:expr, $($px:expr),*) => {{
        $p1.set_high();
        $(
            $px.set_low();
        )*
        }};
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = Adc::new(dp.ADC, Default::default());
    let pa4 = pins.a4.into_analog_input(&mut adc);
    let mut greenl = pins.d4.into_output_high();
    let mut yellowl = pins.d3.into_output_high();
    let mut redl = pins.d2.into_output_high();
    let mut max_voltage = 0;
    loop {
        let read_val = pa4.analog_read(&mut adc);
        max_voltage = max(max_voltage, read_val);

        let percent = (read_val as f32 / max_voltage as f32) * 100.0;
        // ufmt::uwriteln!(
        //     serial,
        //     "max = {}, read = {}, percent = {}",
        //     max_voltage,
        //     read_val,
        //     percent as usize
        // )
        // .unwrap_infallible();
        if percent <= 33_f32 {
            // ufmt::uwriteln!(serial, "green");
            flip_lights!(greenl, yellowl, redl);
            // greenl.set_high();
            // yellowl.set_low();
            // redl.set_low();
        } else if percent <= 66_f32 {
            // ufmt::uwriteln!(serial, "red");
            flip_lights!(yellowl, greenl, redl);
            // greenl.set_low();
            // yellowl.set_high();
            // redl.set_low();
        } else {
            // ufmt::uwriteln!(serial, "red");
            flip_lights!(redl, yellowl, greenl);
            // greenl.set_low();
            // yellowl.set_low();
            // redl.set_high();
        }

        arduino_hal::delay_ms(500);
    }
}
