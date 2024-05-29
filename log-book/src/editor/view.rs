use super::terminal::{Size, Position, Terminal};
use std::io::Error;
mod buffer;
use buffer::Buffer;

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
    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.needs_redraw = true;
    }

    pub fn render_line(row: usize, line: &str) -> Result<(), Error> {
        Terminal::move_caret(Position{row, col: 0})?;
        Terminal::clear_line()?;
        Terminal::print(line)?;
        Ok(())
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} Editor -- Version {VERSION}"); //Switch to NAME, VERSION at end
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        let padding = (width.saturating_sub(len).saturating_sub(1))/2;
        
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);

        full_message
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        let Size {height, width} = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }
        let vert_center = height / 3;
        
        for cur_row in 0..height {
            if let Some(line) = self.buffer.lines.get(cur_row) {
                let truncated_line = if line.len() > width {
                    &line[..width]
                } else {
                    line
                };
                Self::render_line(cur_row, truncated_line)?;
            } else if cur_row == vert_center && self.buffer.is_empty(){
                Self::render_line(cur_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(cur_row, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}