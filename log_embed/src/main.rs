#![no_std]
#![no_main]

#[macro_use]
mod fmt;
use cortex_m::delay::Delay;
use cortex_m_rt::entry;

use cortex_m::Peripherals as CorePeripherals;

#[cfg(feature = "defmt")]
use defmt_rtt as _;

use cortex_m as _; //otherwise won see the vector table and shit

#[derive(defmt::Format)]
struct MyType(u32);

use heapless::Vec;
use nrf52833_hal::{
    clocks::{self},
    Clocks,
};
use nrf52833_pac::Peripherals;
use panic_halt as _;

#[entry]
fn main() -> ! {
    fmt::init();

    let p = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let clock = Clocks::new(p.CLOCK);
    let _ = clock.enable_ext_hfosc();

    let mut systick = Delay::new(cp.SYST, clocks::HFCLK_FREQ);
    info!("Starting {}", MyType(122));
    let mut v: Vec<u8, 16> = Vec::new();
    v.push(12).ok();
    v.push(12).ok();
    v.push(12).ok();
    v.push(12).ok();
    v.push(12).ok();
    info!("vec {}", v);

    trace!("Print bool: {}", false);
    trace!("Print u64 {}", 42u64);
    trace!("Print f {}", 42.0f32);
    let mut x: usize = 1;
    debug!("Print 2: {}", x);
    x += 1;
    debug!("Print 3: {}", x);
    x += 1;
    debug!("Print 4: {}", x);
    x += 1;
    debug!("Print 5: {}", x);
    x += 1;
    debug!("Print 6: {}", x);
    x += 1;
    debug!("Print 7: {}", x);
    x += 1;
    debug!("Print 8: {}", x);
    x += 1;
    debug!("Print 9: {}", x);
    x += 1;
    debug!("Print 10: {}", x);
    x += 1;
    debug!("Print 11: {}", x);
    x += 1;
    debug!("Print 12: {}", x);
    x += 1;
    debug!("Print 13: {}", x);
    x += 1;
    debug!("Print 14: {}", x);
    x += 1;
    debug!("Print 15: {}", x);
    x += 1;
    debug!("Print 16: {}", x);
    x += 1;
    debug!("Print 17: {}", x);
    x += 1;
    debug!("Print 18: {}", x);
    x += 1;
    debug!("Print 19: {}", x);
    x += 1;
    debug!("Print 20: {}", x);
    x += 1;
    debug!("Print 21: {}", x);
    x += 1;
    debug!("Print 22: {}", x);
    x += 1;
    debug!("Print 23: {}", x);
    x += 1;
    debug!("Print 24: {}", x);
    x += 1;
    debug!("Print 25: {}", x);
    x += 1;
    debug!("Print 26: {}", x);
    x += 1;
    debug!("Print 27: {}", x);
    x += 1;
    debug!("Print 28: {}", x);
    x += 1;
    debug!("Print 29: {}", x);
    x += 1;
    debug!("Print 30: {}", x);
    x += 1;
    debug!("Print 31: {}", x);
    x += 1;
    debug!("Print 32: {}", x);
    x += 1;
    debug!("Print 33: {}", x);
    x += 1;
    debug!("Print 34: {}", x);
    x += 1;
    debug!("Print 35: {}", x);
    x += 1;
    debug!("Print 36: {}", x);
    x += 1;
    debug!("Print 37: {}", x);
    x += 1;
    debug!("Print 38: {}", x);
    x += 1;
    debug!("Print 39: {}", x);
    x += 1;
    debug!("Print 40: {}", x);
    x += 1;
    debug!("Print 41: {}", x);
    x += 1;
    debug!("Print 42: {}", x);
    x += 1;
    debug!("Print 43: {}", x);
    x += 1;
    debug!("Print 44: {}", x);
    x += 1;
    debug!("Print 45: {}", x);
    x += 1;
    debug!("Print 46: {}", x);
    x += 1;
    debug!("Print 47: {}", x);
    x += 1;
    debug!("Print 48: {}", x);
    x += 1;
    debug!("Print 49: {}", x);
    x += 1;
    debug!("Print 50: {}", x);
    x += 1;
    debug!("Print 51: {}", x);
    x += 1;
    debug!("Print 52: {}", x);
    x += 1;
    debug!("Print 53: {}", x);
    x += 1;
    debug!("Print 54: {}", x);
    x += 1;
    debug!("Print 55: {}", x);
    x += 1;
    debug!("Print 56: {}", x);
    x += 1;
    debug!("Print 57: {}", x);
    x += 1;
    debug!("Print 58: {}", x);
    x += 1;
    debug!("Print 59: {}", x);
    x += 1;
    debug!("Print 60: {}", x);
    x += 1;
    debug!("Print 61: {}", x);
    x += 1;
    debug!("Print 62: {}", x);
    x += 1;
    debug!("Print 63: {}", x);
    x += 1;
    debug!("Print 64: {}", x);
    x += 1;
    debug!("Print 65: {}", x);
    x += 1;
    debug!("Print 66: {}", x);
    x += 1;
    debug!("Print 67: {}", x);
    x += 1;
    debug!("Print 68: {}", x);
    x += 1;
    debug!("Print 69: {}", x);
    x += 1;
    debug!("Print 70: {}", x);
    x += 1;
    debug!("Print 71: {}", x);
    x += 1;
    debug!("Print 72: {}", x);
    x += 1;
    debug!("Print 73: {}", x);
    x += 1;
    debug!("Print 74: {}", x);
    x += 1;
    debug!("Print 75: {}", x);
    x += 1;
    debug!("Print 76: {}", x);
    x += 1;
    debug!("Print 77: {}", x);
    x += 1;
    debug!("Print 78: {}", x);
    x += 1;
    debug!("Print 79: {}", x);
    x += 1;
    debug!("Print 80: {}", x);
    x += 1;
    debug!("Print 81: {}", x);
    x += 1;
    debug!("Print 82: {}", x);
    x += 1;
    debug!("Print 83: {}", x);
    x += 1;
    debug!("Print 84: {}", x);
    x += 1;
    debug!("Print 85: {}", x);
    x += 1;
    debug!("Print 86: {}", x);
    x += 1;
    debug!("Print 87: {}", x);
    x += 1;
    debug!("Print 88: {}", x);
    x += 1;
    debug!("Print 89: {}", x);
    x += 1;
    debug!("Print 90: {}", x);
    x += 1;
    debug!("Print 91: {}", x);
    x += 1;
    debug!("Print 92: {}", x);
    loop {
        systick.delay_ms(1000);
        debug!("hello");
    }
}
