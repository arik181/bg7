pub use std::{error::Error, io};
pub use std::io::Stdout;
pub use std::result;
pub use tui::{
    backend::{Backend, CrosstermBackend},
    //widgets::{Block,Borders},
    //style::{Color, Style},
    //Frame, 
    Terminal,
};
pub use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


pub fn setup_term() -> Result<(), io::Error> 
{
    let mut stdout = io::stdout();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(())
}

pub fn cleanup_term(mut t: Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> 
{
    let mut stdout = io::stdout();

    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    t.show_cursor()?;

    Ok(())
}
