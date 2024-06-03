mod terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use terminal::{Terminal, Size, Position};
use std::{io::Error, u16};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event 
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Danke.\r\n")?;
        } else {
            Self::draw_tildes()?;
            Self::print_welcome()?;
            Terminal::move_cursor_to(Position {x:0, y:0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    fn draw_tildes() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn print_welcome() -> Result<(), Error> {
        let version_text = format!("Version - {VERSION}");
        let name_len = NAME.len() as u16;
        let version_len = version_text.len() as u16;
        let Size { height, width } = Terminal::size()?;
        Terminal::move_cursor_to(Position { x: (width/2 - name_len/2) , y: height/3 })?;
        Terminal::print(NAME)?;
        Terminal::move_cursor_to(Position { x: (width/2 - (version_len)/2) , y: height/3 + 1 })?;
        Terminal::print(version_text)?;
        Ok(())
    }
}
