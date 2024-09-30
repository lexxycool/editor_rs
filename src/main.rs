use std::io::Result;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use editor::Editor;

mod reader;
mod editor;
mod output;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() -> Result<()> {

    let _clean_up = CleanUp;
    enable_raw_mode()?;
    
    let editor = Editor::new(); 
    while editor.run()? {
       
    }
    
    Ok(())
    
}
