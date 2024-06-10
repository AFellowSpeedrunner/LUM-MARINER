use alloc::string::String;
use crate::println;
use crate::task::keyboard::ScancodeStream;
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub async fn ulsh_main() {
    let mut keyboard = Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore);

    println!("Welcome to ULSH! (Unix-Like SHell)");
    loop {
        println!("ULSH > ");
        let input = read_line(&mut keyboard).await;

        match input.trim() {
            "exit" => {
                println!("Exiting shell...\n");
                break;
            }
            "hello" => {
                println!("Hello, world!\n");
            }
            "help" => {
                println!("Available commands:\nhello - Say hello\nexit - Exit the shell\n");
            }
            _ => {
                println!("Unknown command: {}\n", input.trim());
            }
        }
    }
}

async fn read_line(keyboard: &mut Keyboard<layouts::Us104Key, ScancodeSet1>) -> String {
    let mut input = String::new();
    let mut scancodes = ScancodeStream::new();

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\n' {
                            break;
                        } else {
                            input.push(character);
                            println!("{}", character);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    input
}
