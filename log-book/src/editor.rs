use core::cmp::min;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use terminal::{Terminal, Size, Position};
use std::{env, io::Error, panic::{set_hook, take_hook}};
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

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(e) => {
                    #[cfg(debug_assertions)]
                    panic!("Could not read event: {e:?}");
                }
            }
        }
        Ok(())
    }

    fn move_point(&mut self, key: KeyCode) {
        let Location{mut x, mut y} = self.location;
        let Size{width, height} = Terminal::size().unwrap_or_default();
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(y.saturating_add(1), height.saturating_sub(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(x.saturating_add(1), width.saturating_sub(1)),
            KeyCode::Home => x = 0,
            KeyCode::End => x = width.saturating_sub(1),
            _ => (),
        }
        self.location = Location{x, y};
        
    }

    #[allow(clippy::needless_pass_by_value)] //Size is not an issue so passing by value is fine
    fn evaluate_event(&mut self, event: Event) {
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
                }
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
                    self.move_point(code);
                }
                //Otherwise do nothing
                _ => {}
            },
            Event::Resize(width_u16, height_u16 ) => {
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                self.view.resize(Size{width, height});
            },
            _ => {},
        }
        
    }

    fn refresh_screen(&mut self) {
        //Using let _ because we don't care if there's any errors here
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret(Position{row: self.location.y, col: self.location.x});
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye!\r\n");
        }
    }
}