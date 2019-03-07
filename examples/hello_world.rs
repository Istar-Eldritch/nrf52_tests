#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;

extern crate embedded_hal_spy;
extern crate nrf52832_hal;
extern crate cortex_m_semihosting as sh;
extern crate panic_halt;

use rt::entry;
use sh::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("Hello World");
    loop {}
}
