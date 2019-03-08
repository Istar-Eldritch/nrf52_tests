#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;

extern crate embedded_hal_spy;
extern crate nrf52832_hal as hal;
extern crate cortex_m_semihosting as sh;
extern crate panic_halt;
extern crate nrf52_tests;

use rt::entry;
use cortex_m::peripheral::syst;

#[entry]
fn main() -> ! {
    let mut board = nrf52_tests::Board::take().unwrap();
    board.leds.led_1.enable();


    let mut systick = board.SYST;

    systick.set_clock_source(syst::SystClkSource::Core);

    // 32Hz 
    systick.set_reload(32_000_000);

    systick.enable_counter();

    let mut led_is_on = false;
    loop {
        while !systick.has_wrapped() {}
        if led_is_on {
            board.leds.led_1.disable();
        } else {
            board.leds.led_1.enable();
        }
        systick.clear_current();

        led_is_on = !led_is_on;
    }
}
