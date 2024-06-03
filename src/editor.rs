mod terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use terminal::{Terminal, Size, Position};
use std::io::Error;

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
        let program_name = "Hecto";
        let version_info = "Version 0.1";
        let Size { height, width } = Terminal::size()?;
        Terminal::move_cursor_to(Position { x: (width/2 - (program_name.len() as u16)/2) , y: height/3 })?;
        Terminal::print("Hecto")?;
        Terminal::move_cursor_to(Position { x: (width/2 - (version_info.len() as u16)/2) , y: height/3 + 1 })?;
        Terminal::print("Version 0.1")?;
        Terminal::execute()?;
        Ok(())
    }
}
