use std::io::{Result, stdout};
use crossterm::{execute, terminal::{ClearType, Clear}};

pub struct Output;

impl Output {
    pub fn new() -> Self {
        Self
    }

    fn clear_screen() -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn refresh_screen(&self) -> Result<()> {
        Self::clear_screen()
    }
}