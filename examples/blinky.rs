#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate nrf52832_hal as hal;
extern crate panic_halt;
extern crate nrf52_tests;
extern crate nb;

use rt::entry;
use hal::prelude::*;
use nb::block;

#[entry]
fn main() -> ! {
    let mut board = nrf52_tests::Board::take().unwrap();

    let mut timer = board.TIMER0.constrain();

    let mut led_is_on = false;
    loop {
        if led_is_on {
            board.leds.led_1.disable();
        } else {
            board.leds.led_1.enable();
        }
        timer.start(1_000_000_u32);
        block!(timer.wait());
        led_is_on = !led_is_on;
    }
}
