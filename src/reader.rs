use std::{io::Error, result::Result, time::Duration};
use crossterm::event::{self, Event, KeyEvent};

pub struct Reader;

impl Reader {
    pub fn read_key(&self) -> Result<KeyEvent, Error> {
        loop {
            if (event::poll(Duration::from_millis(500)))? { // returns OK(false) and loop continues every 500ms until there's a key press
                if let Event::Key(event) = (event::read())? { // OK(true) and it comes here when there's key press 
                    return Ok(event); // returns OK(with the event params)
                }
            }
        }
    } 
}