
pub struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
}

impl CursorController {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn get_cursor_x(&self) -> usize {
        self.cursor_x
    }

    pub fn get_cursor_y(&self) -> usize {
        self.cursor_y
    }

    pub fn move_cursor(&mut self, direction: char) {
        match direction {
            'w' => self.cursor_y -= 1,
            
            'a' => self.cursor_x -= 1,

            's' => self.cursor_y += 1,

            'd' => self.cursor_x += 1,

            _ => unimplemented!(),
        }
    }

}

