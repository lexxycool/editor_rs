use std::io::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use crate::{reader::Reader, output::Output};

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    fn process_keypress(&self) -> Result<bool> {
        match (self.reader.read_key())? {  // gets the keypress from the reader struct and matches it 
            KeyEvent {
                code: KeyCode::Char('Q'),
                modifiers: event::KeyModifiers::SHIFT,
                ..
                                   
            } => return Ok(false), // when the keycode comes here and it's 'SHIFT + Q', the terminal breaks.
            _ => {}
        }
       Ok(true)
    }

    pub fn run(&self) -> Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }

}

