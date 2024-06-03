use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear,ClearType, size};
use std::io::stdout;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error>{
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::draw_rows()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen() -> Result<(), std::io::Error>{
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        if let Ok(term_size) = size() {
                for i in 1..term_size.1 {
                    execute!(stdout, MoveTo(0,i)).unwrap();
                    execute!(stdout, Print("~")).unwrap();
                }
            }
        execute!(stdout, MoveTo(0,0))
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
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
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Danke.\r\n");
        }
        Ok(())
    }
}
