#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

/**
 * Infrared example using a Irdroino shield.
 * https://github.com/irdroid/irdroino
 *
 * Pins and functions for the shield
 *
 *  d7:    Yellow led
 *  d6:    Blue led
 *  d5:    Rx Button
 *  d4:    Tx Button
 *  d3:    Infrared tx
 *  d2:    Infrared rx
 *
 */
use core::cell::Cell;

use arduino_hal::{
    hal::port::PD2,
    pac::tc0::tccr0b::CS0_A,
    port::{
        Pin,
        mode::{Floating, Input},
    },
    prelude::*,
};
use avr_device::interrupt::Mutex;
use infrared::{
    protocol::{nec::NecCommand, *},
    receiver::PeriodicPoll,
};
use panic_halt as _;

type IrPin = Pin<Input<Floating>, PD2>;
type IrProto = Nec;
type IrCmd = NecCommand;

static mut RECEIVER: Option<PeriodicPoll<IrProto, IrPin>> = None;
static CMD: Mutex<Cell<Option<IrCmd>>> = Mutex::new(Cell::new(None));

const POLL_FREQ: u32 = 20_000;

#[arduino_hal::entry]
#[allow(static_mut_refs)]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // TOP = CPU_FREQ / TARGET_FREG / PRESCALER - 1
    // 16_000_000 / 20_000 / 8 - 1 = 99
    timer_start(dp.TC0, CS0_A::PRESCALE_8, 99);

    let ir = PeriodicPoll::with_pin(POLL_FREQ, pins.d2);

    unsafe {
        RECEIVER.replace(ir);
    }

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    ufmt::uwriteln!(&mut serial, "Hello from Arduino with Irdroino!\r").unwrap_infallible();

    loop {
        if let Some(cmd) = avr_device::interrupt::free(|cs| CMD.borrow(cs).take()) {
            ufmt::uwriteln!(
                &mut serial,
                "Cmd: Address: {}, Command: {}, repeat: {}\r",
                cmd.addr,
                cmd.cmd,
                cmd.repeat
            )
            .unwrap_infallible();
        }

        arduino_hal::delay_ms(100);
    }
}

fn timer_start(tc0: arduino_hal::pac::TC0, prescaler: CS0_A, top: u8) {
    // Configure the timer for the above interval (in CTC mode)
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(top));
    tc0.tccr0b.write(|w| w.cs0().variant(prescaler));

    // Enable interrupt
    tc0.timsk0.write(|w| w.ocie0a().set_bit());
}

#[avr_device::interrupt(atmega328p)]
#[allow(static_mut_refs)]
fn TIMER0_COMPA() {
    let recv = unsafe { RECEIVER.as_mut().unwrap() };

    if let Ok(Some(cmd)) = recv.poll() {
        // Command received

        avr_device::interrupt::free(|cs| {
            let cell = CMD.borrow(cs);
            cell.set(Some(cmd));
        });
    }
}
