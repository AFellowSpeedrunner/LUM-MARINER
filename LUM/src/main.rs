#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod print;
mod wait;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Extract the location if available
    if let Some(location) = info.location() {
        println!("{}", format_args!(
            "PANIC: at file '{}' line {}\n",
            location.file(),
            location.line()
        ));
    } else {
        println!("{}", format_args!("PANIC: Location unknown.\n"));
    }

    // Extract the payload if available
    if let Some(payload) = info.payload().downcast_ref::<&str>() {
        println!("{}", format_args!("Message: {}\n", payload));
    } else {
        println!("{}", format_args!("Message: Well, something happened.\n"));
    }

    loop {}
}

// Entry point of the program.
#[no_mangle]
pub extern "C" fn _start() -> () {
    let hello = "Hello, LUM/MARINER!\n";
    let oh = "Oh and...\n";
    let newline = "We now have a better print system in LUM!\n";
    let thanksandsorry = "Credits to Phillip Oppermann for the print code, sorry I stole it from blog_os... nothing was working and I was desperate...\n";

    // Log messages to the screen
    println!("{}", format_args!("{}\n{}\n{}\n{}", hello, oh, newline, thanksandsorry));

    // Wait for 4 seconds
    wait::wait(4);

    // Trigger a panic to demonstrate panic handling
    panic!("");
}
