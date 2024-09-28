// #![warn(clippy::all, clippy::pedantic)]
// mod editor;

// use editor::Editor;
// use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode, KeyEvent};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() {
   
    // let editor = Editor::default();
    // editor.run();
    let _clean_up = CleanUp;
    enable_raw_mode().expect("Could not turn on raw mode");
    
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => break,
                _ => {
                    //todo
                }
            }
            println!("{:?}\r", event);
        };
    }

    // let mut buf = [0; 1];
    // while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
    //     let character = buf[0] as char;
    //     if character.is_control() {
    //         println!("{}\r", character as u8);
    //     } else {
    //         println!("{}\r", character);
    //     }
    // }
    
    
}
