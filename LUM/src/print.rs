use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    cursor_position: (usize, usize),
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            cursor_position: (BUFFER_HEIGHT - 1, 0),
        }
    }

    pub fn update_cursor(&self) {
        let position = (self.cursor_position.0 * BUFFER_WIDTH + self.cursor_position.1) as u16;
        unsafe {
            x86_64::instructions::port::Port::new(0x3D4).write(0x0F as u16);
            x86_64::instructions::port::Port::new(0x3D5).write(position as u8);
            x86_64::instructions::port::Port::new(0x3D4).write(0x0E as u16);
            x86_64::instructions::port::Port::new(0x3D5).write((position >> 8) as u8);
        }
    }
    
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\x08' => self.handle_backspace(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
                self.cursor_position = (row, self.column_position);
                self.update_cursor();
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        // Only process if we are not at the start of the line
        if self.column_position > 0 {
            // Move the cursor one position back
            self.column_position -= 1;
            let row = BUFFER_HEIGHT - 1;
            let col = self.column_position;

            // Clear the character at the current position
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            });
        } else if self.cursor_position.0 > 0 {
            // If we are at the start of the line and not at the top row, move to the previous line
            self.cursor_position.0 -= 1;
            self.column_position = BUFFER_WIDTH - 1;

            let row = self.cursor_position.0;
            let col = self.column_position;

            // Clear the character at the new position
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            });
        }
        
        // Update the cursor position
        self.update_cursor();
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            if byte.is_ascii() {
                self.write_byte(byte);
            } else {
                self.write_byte(0xfe);
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        self.cursor_position = (BUFFER_HEIGHT - 1, 0);
        self.update_cursor();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap_or_else(|e| {
            println!("Error writing to VGA buffer: {:?}", e);
        });
    });
}
