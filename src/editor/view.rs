use super::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {
    pub fn render() -> Result<(), Error> {
        let Size { height,.. } = Terminal::size()?;
        Terminal::clear_line()?;
        Terminal::print("Hellow, World!\r\n")?;
        for current_row in 1..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::print_pkg_name()?;
            } else if current_row == (height/3).saturating_add(1) {
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
        let Size { width,.. } = Terminal::size()?;
        
        let name_len = NAME.len();
        let padding = (width.saturating_sub(name_len)/2).saturating_sub(1);
        let spaces = " ".repeat(padding);
        let name_text = format!("~{spaces}{NAME}");
        Terminal::print(&name_text)?;

        Ok(())
    }

    fn print_pkg_version() -> Result<(), Error> {
        let Size { width,.. } = Terminal::size()?;
        
        let vrsn_text = format!("Version - {VERSION}");
        let padding = (width.saturating_sub(vrsn_text.len())/2).saturating_sub(1);
        if padding < 1 {
            let lil_padding = width.saturating_sub(VERSION.len())/2;
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
}
