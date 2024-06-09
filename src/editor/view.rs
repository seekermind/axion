mod buffer;
use buffer::Buffer;

use super::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}
impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };

                Self::render_line(current_row, truncated_line);
            } else if current_row == height / 3 && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::pkg_name(width) );
            } else if current_row == (height / 3).saturating_add(1) && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::pkg_version(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }
    fn pkg_name(width: usize) -> String {
        let name_len = NAME.len();
        if width < name_len {
            return "~".to_string();
        }
        let padding = (width.saturating_sub(name_len) / 2).saturating_sub(1);
        let spaces = " ".repeat(padding);
        let name_text = format!("~{spaces}{NAME}");
        name_text
    }

    fn pkg_version(width: usize) -> String {
        let vrsn_text = format!("Version - {VERSION}");
        let padding = (width.saturating_sub(vrsn_text.len()) / 2).saturating_sub(1);
        if padding == 0 {
            let lil_padding = (width.saturating_sub(VERSION.len()) / 2).saturating_sub(1);
            let spaces = " ".repeat(lil_padding);
            let mut vrsn_text_spaces = format!("~{spaces}{VERSION}");
            if width < vrsn_text_spaces.len() {
                vrsn_text_spaces = "~".to_string();
            }
            vrsn_text_spaces
        } else {
            let spaces = " ".repeat(padding);
            let vrsn_text_spaces = format!("~{spaces}{vrsn_text}");
            vrsn_text_spaces
        }
    }
    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
    pub fn resize(&mut self, to_size: Size) {
        self.size = to_size;
        self.needs_redraw = true;
    }
    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_line(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line."); 
    }
}
