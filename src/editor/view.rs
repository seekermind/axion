mod buffer;
use buffer::Buffer;

use super::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            Self::render_welcome_msg()
        } else {
            self.render_buffer()
        }
    }
    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            } else {
                Terminal::print("~\r\n")?;
            }
        }
        Ok(())
    }
    pub fn render_welcome_msg() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            if current_row == height / 3 {
                Self::print_pkg_name()?;
            } else if current_row == (height / 3).saturating_add(1) {
                Self::print_pkg_version()?;
            } else {
                Terminal::print("~")?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn print_pkg_name() -> Result<(), Error> {
        let Size { width, .. } = Terminal::size()?;

        let name_len = NAME.len();
        let padding = (width.saturating_sub(name_len) / 2).saturating_sub(1);
        let spaces = " ".repeat(padding);
        let name_text = format!("~{spaces}{NAME}");
        Terminal::print(&name_text)?;

        Ok(())
    }

    fn print_pkg_version() -> Result<(), Error> {
        let Size { width, .. } = Terminal::size()?;

        let vrsn_text = format!("Version - {VERSION}");
        let padding = (width.saturating_sub(vrsn_text.len()) / 2).saturating_sub(1);
        if padding < 1 {
            let lil_padding = width.saturating_sub(VERSION.len()) / 2;
            let spaces = " ".repeat(lil_padding);
            let vrsn_text_spaces = format!("{spaces}{VERSION}");
            Terminal::print(&vrsn_text_spaces)?;
        } else {
            let spaces = " ".repeat(padding);
            let vrsn_text_spaces = format!("~{spaces}{vrsn_text}");
            Terminal::print(&vrsn_text_spaces)?;
        }

        Ok(())
    }
    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
        }
    }
}
