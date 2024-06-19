use alloc::string::String;
use crate::println;
use crate::task::keyboard::ScancodeStream;
use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub async fn ulsh_main() {
    let mut keyboard = Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore);

    println!("Welcome to ULSH! (Unix-Like SHell)\n\n");
    loop {
        println!("ULSH > ");
        let input = read_line(&mut keyboard).await;

        match input.trim() {
            "exit" => {
                println!("Exiting shell...\n");
                return;
            }
            "stars" => {
                println!("LUM/MARINER, as of the 19th of June 2024 22:53 BST, has 9 stars. Thanks to everyone who has starred the repository!\n")
            }
            "ahoy" => {
                println!("Ahoy, LUM/MARINER!\n");
            }
            "f" => {
                println!("F\n\n");
                panic!("F");
            }
            "F" => {
                println!("F\n\n");
                panic!("F");
            }
            "panic" => {
                panic!();
            }
            "help" => {
                println!("Available commands:\nahoy - Ahoy, captain!\nexit - Exit the shell\nstars - See how many stars the repo has since the latest ULSH update\npanic - Make the kernel panic\n");
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
    let mut buffer = String::new(); // Buffer to store the current line

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\n' {
                            println!("{}", character); // Print the newline character
                            break;
                        } else if character == '\x08' { // Backspace character
                            if !input.is_empty() {
                                input.pop();
                                buffer.pop();
                            }
                        } else {
                            input.push(character);
                            buffer.push(character);
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
