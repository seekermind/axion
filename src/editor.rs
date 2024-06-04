mod terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::{cmp::min, io::Error};
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
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
            self.evaluate_event(&event)?;
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press, // needed for Windows
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_location(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }
    fn move_location(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            KeyCode::End => x = width.saturating_sub(1),
            KeyCode::Home => x = 0,
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Danke.\r\n")?;
        } else {
            Self::draw_tildes()?;
            Self::print_welcome()?;
            Terminal::move_cursor_to(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    fn draw_tildes() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn print_welcome() -> Result<(), Error> {
        let version_text = format!("Version - {VERSION}");

        #[allow(clippy::cast_possible_truncation)]
        let name_len = NAME.len();
        #[allow(clippy::cast_possible_truncation)]
        let version_len = version_text.len();

        let Size { height, width } = Terminal::size()?;
        Terminal::move_cursor_to(Position {
            #[allow(clippy::arithmetic_side_effects)]
            x: (width / 2) - (name_len / 2),
            y: height / 3,
        })?;
        Terminal::print(NAME)?;
        Terminal::move_cursor_to(Position {
            #[allow(clippy::arithmetic_side_effects)]
            x: (width / 2) - (version_len / 2),
            #[allow(clippy::arithmetic_side_effects)]
            y: (height / 3) + 1,
        })?;
        Terminal::print(version_text)?;
        Ok(())
    }
}
