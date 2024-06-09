mod terminal;
mod view;
use core::{cmp::min, panic};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::{Position, Size, Terminal};
use view::View;

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }
    // needless_pass_by_value: Event is not huge, so there is not a
    // performance overhead in passing by value, and pattern matching in this
    // function would be needlessly complicated if we pass by reference here.
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::End
                    | KeyCode::Home
                    | KeyCode::PageUp
                    | KeyCode::PageDown,
                    _,
                ) => {
                    self.move_location(code);
                }
                _ => (),
            },
            Event::Resize(width_u16, height_u16) => {
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;

                self.view.resize(Size { height, width });
            }
            _ => (),
        }
    }
    fn move_location(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            KeyCode::End => x = width.saturating_sub(1),
            KeyCode::Home => x = 0,
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height.saturating_sub(1),
            _ => (),
        }
        self.location = Location { x, y };
    }
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor_to(Position {
            x: self.location.x,
            y: self.location.y,
        });
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Danke\r\n");
        }
    }
}
