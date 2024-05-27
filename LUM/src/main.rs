#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod print;
mod wait;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print::clear_screen();

    // Extract the location if available
    if let Some(location) = info.location() {
        print::log_formatted(format_args!(
            "PANIC: at file '{}' line {}\n",
            location.file(),
            location.line()
        ));
    } else {
        print::log_formatted(format_args!("PANIC: Location unknown.\n"));
    }

    // Extract the payload if available
    if let Some(payload) = info.payload().downcast_ref::<&str>() {
        print::log_formatted(format_args!("Message: {}\n", payload));
    } else {
        print::log_formatted(format_args!("Message: Well, something happened.\n"));
    }

    loop {}
}

// Entry point of the program.
#[no_mangle]
pub extern "C" fn _start() -> () {
    let hello = "Hello, LUM/MARINER!\n";
    let oh = "Oh and...\n";
    let newline = "We now have newline support in LUM!";

    // Log messages to the screen
    print::log_formatted(format_args!("{}\n{}\n{}", hello, oh, newline));

    // Wait for 4 seconds
    wait::wait(4);

    // Trigger a panic to demonstrate panic handling
    panic!("");
}
