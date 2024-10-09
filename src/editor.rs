use std::io::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
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

    fn process_keypress(&mut self) -> Result<bool> {
        match (self.reader.read_key())? {  // gets the keypress from the reader struct and matches it 
            KeyEvent {
                code: KeyCode::Char('Q'),
                modifiers: event::KeyModifiers::SHIFT,
                ..                                  
            } => return Ok(false), // when the keycode comes here and it's 'SHIFT + Q', the terminal breaks.
            KeyEvent {
                code: direction @ (
                      KeyCode::Up 
                    | KeyCode::Down 
                    | KeyCode::Left 
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End
                ),
                modifiers: KeyModifiers::NONE,
                ..
            } => self.output.move_cursor(direction),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
                ..
            } => (0..self.output.get_win_size_1()).for_each(|_| {
                self.output.move_cursor(if matches!(val, KeyCode::PageUp) {
                    KeyCode::Up
                } else {
                    KeyCode::Down
                });
            }),
            _ => {}
        }
       Ok(true)
    }

    pub fn run(&mut self) -> Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }

}

