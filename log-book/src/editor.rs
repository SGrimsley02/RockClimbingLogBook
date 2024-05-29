use core::cmp::min;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use terminal::{Terminal, Size, Position};
use std::{env, io::Error};
mod view;
use view::View;


/*
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
 */
#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().skip(1).collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(event)?;
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

    #[allow(clippy::needless_pass_by_value)] //Size is not an issue so passing by value is fine
    fn evaluate_event(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::Key(KeyEvent {
            code, 
            modifiers, 
            kind: KeyEventKind::Press, //Windows compatibility
            ..
        }) => match (code, modifiers) {
                //Quit
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                },
                //Move caret
                (
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home,
                _,
                ) => {
                    self.move_point(code)?;},
                //Otherwise do nothing
                _ => {},
            },
            Event::Resize(width_u16, height_u16 ) => {
                let width = width_u16 as usize;
                let height = height_u16 as usize;
                self.view.resize(Size{width, height});
            },
            _ => {},
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret(Position{col: self.location.x, row: self.location.y})?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    

}