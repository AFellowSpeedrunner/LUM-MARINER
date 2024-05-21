use core::fmt::Write;

// VGA text mode buffer address.
static mut VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

// Define VGA display dimensions.
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// Define a simple spinlock mutex for synchronization.
static mut VGA_MUTEX: SpinlockMutex<()> = SpinlockMutex::new(());

// Define a simple spinlock mutex.
pub struct SpinlockMutex<T> {
    data: T,
    locked: bool,
}

impl<T> SpinlockMutex<T> {
    pub const fn new(data: T) -> Self {
        SpinlockMutex { data, locked: false }
    }

    pub fn lock(&mut self) -> SpinlockMutexGuard<T> {
        while self.locked {}
        self.locked = true;
        SpinlockMutexGuard { inner: self }
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }
}

pub struct SpinlockMutexGuard<'a, T> {
    inner: &'a mut SpinlockMutex<T>,
}

impl<'a, T> Drop for SpinlockMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.inner.unlock();
    }
}

// Function to log a string to the VGA buffer.
pub fn log_string(s: &str) {
    // Lock the mutex to ensure exclusive access to the VGA buffer.
    let _lock = unsafe { VGA_MUTEX.lock() };

    // Initialize variables for tracking cursor position.
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    // Write each byte of the string to the VGA buffer.
    for byte in s.bytes() {
        match byte {
            // Handle newline character.
            b'\n' => {
                // Move to the beginning of the next line.
                cursor_x = 0;
                cursor_y += 1;

                // Check if cursor reached the end of the screen.
                if cursor_y >= VGA_HEIGHT {
                    // Scroll the screen up by one line.
                    scroll_up();
                    cursor_y -= 1;
                }
            }
            // Handle other characters.
            _ => {
                // Calculate the position in the VGA buffer.
                let position = cursor_y * VGA_WIDTH + cursor_x;

                // Write the character to the VGA buffer.
                unsafe {
                    *VGA_BUFFER.offset(position as isize) = (0x0f00 | byte as u16) as u16;
                }

                // Move cursor to the next position.
                cursor_x += 1;

                // Check if cursor reaches the end of the line.
                if cursor_x >= VGA_WIDTH {
                    cursor_x = 0;
                    cursor_y += 1;

                    // Check if cursor reached the end of the screen.
                    if cursor_y >= VGA_HEIGHT {
                        // Scroll the screen up by one line.
                        scroll_up();
                        cursor_y -= 1;
                    }
                }
            }
        }
    }
}

// Function to scroll the screen up by one line.
fn scroll_up() {
    // Lock the mutex to ensure exclusive access to the VGA buffer.
    let _lock = unsafe { VGA_MUTEX.lock() };

    // Copy each line of the screen buffer to the line above it.
    for y in 1..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            unsafe {
                let src_offset = y * VGA_WIDTH + x;
                let dest_offset = (y - 1) * VGA_WIDTH + x;
                *VGA_BUFFER.offset(dest_offset as isize) = *VGA_BUFFER.offset(src_offset as isize);
            }
        }
    }

    // Clear the last line of the screen buffer.
    for x in 0..VGA_WIDTH {
        unsafe {
            let offset = (VGA_HEIGHT - 1) * VGA_WIDTH + x;
            *VGA_BUFFER.offset(offset as isize) = (0x0f00 | b' ' as u16);
        }
    }
}

// Function to log formatted string to the VGA buffer.
pub fn log_formatted(fmt: core::fmt::Arguments) {
    // Lock the mutex to ensure exclusive access to the VGA buffer.
    let _lock = unsafe { VGA_MUTEX.lock() };

    // Write the formatted string to the VGA buffer using the Writer.
    let mut writer = Writer {
        buffer: unsafe { VGA_BUFFER },
        column_position: 0,
        row_position: 0,
    };
    writer.write_fmt(fmt).unwrap();

    // After printing the message, move the cursor to the next line.
    let mut cursor_x = writer.column_position;
    let mut cursor_y = writer.row_position;

    // Move to the next line.
    cursor_x = 0;
    cursor_y += 1;

    // Check if cursor reached the end of the screen.
    if cursor_y >= VGA_HEIGHT {
        // Scroll the screen up by one line.
        scroll_up();
    }
}

// Function to clear the VGA buffer (clear the screen).
pub fn clear_screen() {
    // Lock the mutex to ensure exclusive access to the VGA buffer.
    let _lock = unsafe { VGA_MUTEX.lock() };

    // Iterate over each character position in the VGA buffer and clear it.
    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            let position = y * VGA_WIDTH + x;
            unsafe {
                // Set each character to a space with default attribute.
                *VGA_BUFFER.offset(position as isize) = (0x0f00 | b' ' as u16) as u16;
            }
        }
    }
}

// Writer struct for logging formatted strings.
struct Writer {
    buffer: *mut u16,
    column_position: usize,
    row_position: usize,
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Write each byte of the string to the VGA buffer.
        for byte in s.bytes() {
            match byte {
                // Handle newline character.
                b'\n' => {
                    // Move to the beginning of the next line.
                    self.column_position = 0;
                    self.row_position += 1;
                }
                // Handle other characters.
                byte => {
                    // Check if the current character exceeds the screen width.
                    if self.column_position >= VGA_WIDTH {
                        // Move to the next line.
                        self.column_position = 0;
                        self.row_position += 1;
                    }
                    // Check if the current position exceeds the screen height.
                    if self.row_position >= VGA_HEIGHT {
                        // Scroll the screen up by one line.
                        scroll_up();
                        self.row_position -= 1;
                    }

                    // Calculate the position in the VGA buffer.
                    let position = self.row_position * VGA_WIDTH + self.column_position;

                    // Write the character to the VGA buffer.
                    unsafe {
                        *self.buffer.offset(position as isize) = (0x0f00 | byte as u16) as u16;
                    }

                    // Move cursor to the next position.
                    self.column_position += 1;

                    // If the character is a space, check if the next word fits on the current line.
                    if byte == b' ' {
                        let next_word_start = s[self.column_position..].find(|c: char| !c.is_whitespace());
                        if let Some(next_word_start) = next_word_start {
                            // Calculate the length of the next word.
                            let next_word_length = s[self.column_position + next_word_start..]
                                .find(|c: char| c.is_whitespace())
                                .unwrap_or(s.len() - self.column_position);

                            // Check if the next word fits on the current line.
                            let word_fits = self.column_position + next_word_length < VGA_WIDTH;

                            // Check if the next character is a punctuation mark.
                            let next_char_is_punctuation = s.as_bytes().get(self.column_position + next_word_start + next_word_length) == Some(&b'.');

                            // If the word doesn't fit and the next character is a punctuation mark, move to the next line.
                            if !word_fits && next_char_is_punctuation {
                                // Move to the next line.
                                self.column_position = 0;
                                self.row_position += 1;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
