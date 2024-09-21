#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use LUM::println;
use bootloader::{entry_point, BootInfo};
use LUM::task::{executor::Executor, keyboard, Task};
use LUM::task::keyboard::initialize_scancode_queue;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Directly use the message, assuming it’s a string slice
    let message = if let Some(msg) = info.payload().downcast_ref::<&str>() {
        *msg
    } else {
        "Unknown panic message."
    };

    // Print the panic message
    println!("PANIC: {}\n", message);

    // Halt the CPU in an infinite loop after a panic
    LUM::hlt_loop();
}

entry_point!(lum_main);

fn lum_main(boot_info: &'static BootInfo) -> ! {
    use LUM::allocator;
    use LUM::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    let ahoy = "Ahoy, LUM/MARINER!\n";
    let oh = "Oh and...\n";
    let shell = "We now have a non-func- I mean somewhat functional shell in LUM... sorry me...\n";
    let thanksandsorry = "Credits to Phillip Oppermann for literally the entire current base of this kernel, sorry I stole it from blog_os... I guess I'm too lazy... sorry once again...\n";

    // Log messages to the screen
    println!("{}", format_args!("{}\n{}\n{}\n{}", ahoy, oh, shell, thanksandsorry));

    // Figured it out.
    LUM::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Ensure scancode queue is initialized before starting tasks
    initialize_scancode_queue();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.spawn(Task::new(load_shell()));
    executor.run();

    // If executor.run() returns, panic because there are no tasks left to execute.
    // panic!("No other tasks, bailing out for safety...");

    // Commented because it's useless currently and won't even work due to latest changes.
}

async fn load_shell() {
    LUM::ulsh::shell::ulsh_main().await;
}
