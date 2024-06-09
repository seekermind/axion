mod buffer;
mod line;
mod location;

use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};
use buffer::Buffer;
use location::Location;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}
impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
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
        let top = self.scroll_offset.y;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));
            } else if current_row == height / 3 && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::pkg_name(width));
            } else if current_row == (height / 3).saturating_add(1) && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::pkg_version(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Quit => {}
        }
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
    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }
    pub fn resize(&mut self, to_size: Size) {
        self.size = to_size;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }
    fn move_text_location(&mut self, direction: &Direction) {
        let Location {mut x, mut y} = self.location;
        let Size { height, width } = self.size;

        match direction {
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::Left => x = x.saturating_sub(1),
            Direction::Right => x = x.saturating_add(1),
            Direction::PageUp => y = 0,
            Direction::PageDown => y = height.saturating_sub(1),
            Direction::Home => x = 0,
            Direction::End => x = width.saturating_sub(1),
        }
        self.location = Location {x, y};
        self.scroll_location_into_view();
    }
    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { height, width } = self.size;
        let mut offset_changed = false;
        
        //scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        //scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }

        self.needs_redraw = offset_changed;
    }
    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_line(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line.");
    }
}
