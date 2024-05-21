use core::arch::asm;

// Function to wait for a specified time in seconds.
pub fn wait(seconds: u64) {
    let cycles = seconds * 10000000;

    // Loop and waste CPU cycles for the specified duration.
    for _ in 0..cycles {
        // This loop just consumes CPU cycles and doesn't perform any meaningful work.
        // Depending on your kernel's architecture and configuration,
        // you may need to add some actual delay logic here.
        unsafe {
            // This is a simple "no operation" instruction that consumes one CPU cycle.
            asm!("nop");
        }
    }
}
