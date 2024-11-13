#![no_std]
#![feature(asm_experimental_arch)]

mod fcpu_16mhz;

#[cfg(feature = "fcpu-16mhz")]
pub use fcpu_16mhz::*;


/// Makes a delay lasting the specified duration (in miliseconds).
pub fn delay_ms(duration_ms: u16) {
    for _ in 0..duration_ms {
        delay_us(1000);
    }
}
