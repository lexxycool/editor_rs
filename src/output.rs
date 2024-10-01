use std::io::{stdout, Result, Write};
use crossterm::{cursor, execute, terminal::{self, Clear, ClearType}};

pub struct Output {
    win_size: (usize, usize),
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
        .map(|(x, y)| (x as usize, y as usize))
        .unwrap();
        Self {win_size}
    }

    pub fn clear_screen() -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0,0))
    }

    pub fn refresh_screen(&self) -> Result<()> {
        Self::clear_screen()?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn draw_rows(&self) {
        let screen_rows = self.win_size.1;
        for i in 0..screen_rows {
            print!("~");
            if i < screen_rows - 1 {
                println!("\r")
            }
            stdout().flush();
        }
    }

}