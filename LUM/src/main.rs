#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod print;
mod wait;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print::clear_screen();
    let crash = "PANIC: Well, something happened.";
    print::log_formatted(format_args!("{}\n", crash));
    loop {}
}

// Entry point of the program.
#[no_mangle]
pub extern "C" fn _start() -> () {
    let hello = "Hello, LUM/MARINER!\n";
    let oh = "Oh and...\n";
    let newline = "We now have newline support in LUM!";
    print::log_formatted(format_args!("{}\n{}\n{}", hello, oh, newline));

    wait::wait(4);

    // If we have nothing else to do, jump to the panic handler.
    panic!();

}
