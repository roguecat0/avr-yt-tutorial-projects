#![no_main]
#![no_std]

use MorseLength::{Long, Short};
use panic_halt as _;

struct MorseCodes {
    a: [MorseLength; 2],
    b: [MorseLength; 4],
    c: [MorseLength; 4],
    d: [MorseLength; 3],
    e: [MorseLength; 1],
    f: [MorseLength; 4],
    g: [MorseLength; 3],
    h: [MorseLength; 4],
    i: [MorseLength; 2],
    j: [MorseLength; 4],
    k: [MorseLength; 3],
    l: [MorseLength; 4],
    m: [MorseLength; 2],
    n: [MorseLength; 2],
    o: [MorseLength; 3],
    p: [MorseLength; 4],
    q: [MorseLength; 4],
    r: [MorseLength; 3],
    s: [MorseLength; 3],
    t: [MorseLength; 1],
    u: [MorseLength; 3],
    v: [MorseLength; 4],
    w: [MorseLength; 3],
    x: [MorseLength; 4],
    y: [MorseLength; 4],
    z: [MorseLength; 4],
}
static CODES: MorseCodes = MorseCodes {
    a: [Short, Long],
    b: [Long, Short, Short, Short],
    c: [Long, Short, Long, Short],
    d: [Long, Short, Short],
    e: [Short],
    f: [Short, Short, Long, Short],
    g: [Long, Long, Short],
    h: [Short, Short, Short, Short],
    i: [Short, Short],
    j: [Short, Long, Long, Long],
    k: [Long, Short, Long],
    l: [Short, Long, Short, Short],
    m: [Long, Long],
    n: [Long, Short],
    o: [Long, Long, Long],
    p: [Short, Long, Long, Short],
    q: [Long, Long, Short, Long],
    r: [Short, Long, Short],
    s: [Short, Short, Short],
    t: [Long],
    u: [Short, Short, Long],
    v: [Short, Short, Short, Long],
    w: [Short, Long, Long],
    x: [Long, Short, Short, Long],
    y: [Long, Short, Long, Long],
    z: [Long, Long, Short, Short],
};
fn into_morse_codes(c: char) -> &'static [MorseLength] {
    match c {
        'a' => &CODES.a,
        'b' => &CODES.b,
        'c' => &CODES.c,
        'd' => &CODES.d,
        'e' => &CODES.e,
        'f' => &CODES.f,
        'g' => &CODES.g,
        'h' => &CODES.h,
        'i' => &CODES.i,
        'j' => &CODES.j,
        'k' => &CODES.k,
        'l' => &CODES.l,
        'm' => &CODES.m,
        'n' => &CODES.n,
        'o' => &CODES.o,
        'p' => &CODES.p,
        'q' => &CODES.q,
        'r' => &CODES.r,
        's' => &CODES.s,
        't' => &CODES.t,
        'u' => &CODES.u,
        'v' => &CODES.v,
        'w' => &CODES.w,
        'x' => &CODES.x,
        'y' => &CODES.y,
        'z' => &CODES.z,
        _ => panic!("Character '{}' is not a valid Morse letter", c),
    }
}
const DELAY: u32 = 100;
const DOT: u32 = DELAY;
const DASH: u32 = DELAY * 3;
const INTRA_CHAR: u32 = DELAY;
const INTER_CHAR: u32 = DELAY * 3;
const INTER_WORD: u32 = DELAY * 7;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let text = "sos";
    let mut buzz_pin = pins.d2.into_output();
    loop {
        for beeps in text.chars().map(into_morse_codes) {
            for beep in beeps {
                buzz_pin.set_high();
                match beep {
                    Long => arduino_hal::delay_ms(DASH),
                    Short => arduino_hal::delay_ms(DOT),
                }
                buzz_pin.set_low();
                arduino_hal::delay_ms(INTRA_CHAR);
            }
            arduino_hal::delay_ms(INTER_CHAR);
        }
        arduino_hal::delay_ms(INTER_WORD);
    }
}

enum MorseLength {
    Long,
    Short,
}
