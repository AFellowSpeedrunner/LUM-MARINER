use crate::{print, println};
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use alloc::string::String;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

/// Called by the keyboard interrupt handler
///
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        // Ensure the queue is initialized once
        initialize_scancode_queue();
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );
    let mut buffer = String::new(); // Buffer to store the current line

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\n' {
                            println!("{}", buffer); // Print the buffer when Enter is pressed
                            buffer.clear(); // Clear the buffer for the next line
                        } else if character == '\x08' { // Backspace character
                            if !buffer.is_empty() {
                                buffer.pop();
                                print!("\x08 \x08"); // Handle backspace correctly
                            }
                        } else {
                            buffer.push(character);
                            print!("{}", character);
                        }
                    }
                    DecodedKey::RawKey(key) => {
                        // Handle raw key if needed
                    }
                }
            }
        }
    }
}


/// Initialize the SCANCODE_QUEUE if it hasn't been initialized yet
pub fn initialize_scancode_queue() {
    if SCANCODE_QUEUE.try_get().is_err() {
        if let Err(_) = SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100)) {
            println!("ERROR: Failed to initialize scancode queue");
        } else {
            println!("Scancode queue initialized");
        }
    }
}
