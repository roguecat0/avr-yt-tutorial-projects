#![no_std]
#![no_main]

use ag_lcd::{Blink, Display, Error as LcdErr, LcdDisplay, Lines, Mode, Scroll};
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let rs = pins.d12.into_output().downgrade();
    let en = pins.d11.into_output().downgrade(); // Can also use d11
    let d4 = pins.d5.into_output().downgrade();
    let d5 = pins.d4.into_output().downgrade();
    let d6 = pins.d3.into_output().downgrade();
    let d7 = pins.d2.into_output().downgrade();

    // Setting up LCD
    let delay = arduino_hal::Delay::new();
    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new(rs, en, delay)
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .with_lines(Lines::TwoLines)
        .with_blink(Blink::On)
        .build();

    ufmt::uwriteln!(serial, "mode: {}", lcd.mode() as u8).unwrap_infallible();
    // print_lcd_error(lcd.error(), |e| {
    //     ufmt::uwriteln!(serial, "{}", e).unwrap_infallible()
    // });
    // lcd.set_character(7, [7; 8]);
    // arduino_hal::delay_ms(200);
    // lcd.set_blink(Blink::On);
    ufmt::uwrite!(lcd, "hello person").unwrap_infallible();
    loop {}
}
fn print_lcd_error(e: LcdErr, mut f: impl FnMut(&str) -> ()) {
    match e {
        LcdErr::None => f("no error"),
        LcdErr::NoPinRS => f("error: NoPinRS"),
        LcdErr::NoPinEN => f("error: NoPinEN"),
        LcdErr::NoPinRW => f("error: NoPinRW"),
        LcdErr::InvalidMode => f("error: InvalidMode"),
        LcdErr::InvalidCode => f("error: InvalidCode"),
        LcdErr::NoPinD7 => f("error: NoPinD7"),
        LcdErr::NoPinD6 => f("error: NoPinD6"),
        LcdErr::NoPinD5 => f("error: NoPinD5"),
        LcdErr::NoPinD4 => f("error: NoPinD4"),
        _ => f("some error"),
    }
}
