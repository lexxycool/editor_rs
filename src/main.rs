use std::io::Result;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use editor::Editor;
use output::Output;

mod reader;
mod editor;
mod output;
mod editor_contents;
mod cursor_controller;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not disable raw mode");
        Output::clear_screen().expect("Error");
    }
}
 
fn main() -> Result<()> {

    let _clean_up = CleanUp;
    enable_raw_mode()?;
    
    let mut editor = Editor::new(); 
    while editor.run()? {
       
    }  
    Ok(())
    
}
