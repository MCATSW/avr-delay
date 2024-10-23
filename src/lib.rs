#![no_std]
#![feature(asm_experimental_arch)]

use core::arch::asm;

/// Makes a delay lasting the specified duration (in microseconds).
pub fn delay_us(duration_us: u16) {
    /*
     * the cycle formula:
     * cycles = (delay_us / tick_time_us) / ticks_per_loop
     * cycles = (delay_us / cpu_frequency^(-1)) / ticks_per_loop
     * cycles = (delay_us / 16^(-1)) / 4
     * cycles = (delay_us * 16) / 4
     * cycles = delay_us * (16 / 4)
     *
     * convertor = 16 / 4
     * cycles = delay_us * convertor
     */
    const CONVERTOR: f32 = 16.0 / 4.0;
    // calculates the cycle count, adding a 0.5
    // to guarantee ceiling on integer cast
    let cycles: u16 = (duration_us as f32 * CONVERTOR + 0.5) as u16;
    unsafe {
        asm!(
            // 2 TICKS TOTAL
            "1: sbiw {counter}, 1", // 2 TICKS
            "brne 1b", // 1/2 TICKS
            counter = inout(reg_iw) cycles => _,
        );
    }
}

/// Makes a delay lasting the specified duration (in miliseconds).
pub fn delay_ms(duration_ms: u16) {
    for _ in 0..duration_ms {
        delay_us(1000);
    }
}

