use super::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
 
pub struct View;

impl View {
    pub fn render() -> Result<(), Error> {
        let Size {height, ..} = Terminal::size()?;
        Terminal::clear_line()?;
        Terminal::print("Hello, world!")?;
        for row in 1..height {
            Terminal::clear_line()?;

            if row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_line()?;
            }
            if row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} Editor -- Version {VERSION}"); //Switch to NAME, VERSION at end
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        let padding = (width.saturating_sub(len))/2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_empty_line() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}