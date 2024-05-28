#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use LUM::println;
use LUM::task::{executor::Executor, keyboard, Task};
use bootloader::{entry_point, BootInfo};

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

    LUM::hlt_loop();
}

entry_point!(lum_main);

fn lum_main(boot_info: &'static BootInfo) -> ! {
    use LUM::allocator;
    use LUM::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    let hello = "Hello, LUM/MARINER!\n";
    let oh = "Oh and...\n";
    let newline = "We now have a better print system in LUM!\n";
    let thanksandsorry = "Credits to Phillip Oppermann for literally the entire current base of this kernel, sorry I stole it from blog_os... I guess I'm too lazy... sorry once again...\n";

    // Log messages to the screen
    println!("{}", format_args!("{}\n{}\n{}\n{}", hello, oh, newline, thanksandsorry));

    // Prints dots, unneeded right now.
    // LUM::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Keyboard press print code
    // let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(keyboard::print_keypresses()));
    // executor.run();

    // Wait for 4 seconds
    wait::wait(4);

    // Trigger a panic to demonstrate panic handling
    panic!("");
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
