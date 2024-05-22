use core::cmp::min;
use crossterm::event::{read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use terminal::{Terminal, Size, Position};
use std::io::Error;
mod document;
use document::Document;


const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    document: Document::open(),
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

    fn move_point(&mut self, key: KeyCode) -> Result<(), Error> {
        let Location{mut x, mut y} = self.location;
        let Size{width, height} = Terminal::size()?;
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(y + 1, height),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(x + 1, width),
            KeyCode::Home => x = 0,
            KeyCode::End => x = width.saturating_sub(1),
            _ => (),
        }
        self.location = Location{x, y};
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, 
            modifiers, 
            kind: KeyEventKind::Press, //Windows compatibility
            ..
        }) = event {
            match code {
                //Quit
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                //Move caret
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;},
                //Otherwise do nothing
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r");
        } else {
            Self::draw_rows()?;
            Terminal::move_caret(Position{col: self.location.x, row: self.location.y})?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for terminal_row in 0..height {
            Terminal::clear_line()?;
            if let Some(row) = self.document.row(terminal_row as usize) {
                Self::draw_row(row)?;
            } else if terminal_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            
            if row < height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
            if row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{} editor, v{}", NAME, VERSION);
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        let padding = (width.saturating_sub(len))/2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_row(row: &Row) -> Result<(), Error> {
        let start = self.location.x;
        let end = self.location.x + Terminal::size()?.width;
        let row = row.render(start, end);
        Terminal::print(&row)?;
        Ok(())
    }
    

}