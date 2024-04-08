use std::io::{self, stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    /// # Errors
    ///
    /// * `std::io::Error` - If the char cannot be read 
    pub fn new() -> Result<Self, io::Error> {
        let size = termion::terminal_size()?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }
    
    /// # Errors
    ///
    /// * `std::io::Error` - If the char cannot be read 
    pub fn flush() -> Result<(), io::Error> {
        stdout().flush()
    }

    /// # Errors
    ///
    /// * `std::io::Error` - If the char cannot be read 
    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
    
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
    
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
    
    pub fn size(&self) -> &Size {
        &self.size
    }
}
