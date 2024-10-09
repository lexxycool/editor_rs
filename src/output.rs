use std::io::{stdout, Result, Write};
use crossterm::{cursor, event::KeyCode, execute, queue, terminal::{self, Clear, ClearType}};

use crate::editor_contents::EditorContents;
use crate::cursor_controller::CursorController;

pub struct Output {
    win_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
        .map(|(x, y)| (x as usize, y as usize))
        .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),   
        }
    }

    pub fn get_win_size_1(&self) -> usize {
        self.win_size.1
    }

    pub fn clear_screen() -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0,0))
    }

    pub fn refresh_screen(&mut self) -> Result<()> {
        queue!(
            self.editor_contents, 
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        
        let cursor_x = self.cursor_controller.get_cursor_x();
        let cursor_y = self.cursor_controller.get_cursor_y();

        queue!(
            self.editor_contents, 
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        for i in 0..screen_rows {
            if i == screen_rows / 3 {
                let mut welcome = format!("Obinna Editor --- Version");
                if welcome.len() > screen_columns {
                    welcome.truncate(screen_columns);
                }
                let mut padding = (screen_columns - welcome.len()) / 2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome);
            } else {
                self.editor_contents.push('~');
            }
           
            queue!(
                self.editor_contents,
                Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }

}