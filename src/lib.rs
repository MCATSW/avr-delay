#![no_std]
#![feature(asm_experimental_arch)]

use core::arch::asm;

/// Makes a delay lasting the specified duration (in microseconds).
pub fn delay_us(duration_us: u16) {
    unsafe {
        asm!(
            "1: sbiw {counter}, 1",
            "brne 1b",
            counter = inout(reg_iw) duration_us => _,
        );
    }
}

/// Makes a delay lasting the specified duration (in miliseconds).
pub fn delay_ms(duration_ms: u16) {
    for _ in 0..duration_ms {
        delay_us(1000);
    }
}

