use std::{io::Error, result::Result, time::Duration};
use crossterm::event::{self, Event, KeyEvent};

pub struct Reader;

impl Reader {
    pub fn read_key(&self) -> Result<KeyEvent, Error> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    } 
}