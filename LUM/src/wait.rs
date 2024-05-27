use core::arch::asm;

fn read_tsc() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high,
        );
    }
    ((high as u64) << 32) | (low as u64)
}

// Function to estimate CPU frequency in Hz
fn estimate_cpu_frequency() -> u64 {
    let _calibration_time_seconds = 1u64; // 1 second for calibration

    let start = read_tsc();
    let end = start + 100_000_000; // Roughly 100ms worth of TSC cycles

    while read_tsc() < end {
        unsafe {
            asm!("nop");
        }
    }

    let elapsed_tsc = read_tsc() - start;
    let cpu_frequency_hz = elapsed_tsc * 10; // Since we measured over 100ms, multiply by 10 to get Hz

    cpu_frequency_hz
}

// Function to wait for a specified time in seconds.
pub fn wait(seconds: u64) {
    let cpu_frequency_hz = estimate_cpu_frequency();
    let start = read_tsc();
    let wait_cycles = seconds * cpu_frequency_hz;

    while read_tsc() - start < wait_cycles {
        unsafe {
            asm!("nop");
        }
    }
}
