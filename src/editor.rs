use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char};

pub struct Editor {


}

impl Editor {

    pub fn default() -> Self {
        Editor {

        }
    }
    
    pub fn run(&self) {
        enable_raw_mode();
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye. \r\n");
        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");
                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;  
        Ok(())  
    }

}

