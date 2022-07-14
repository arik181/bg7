pub use std::{error::Error, io};
pub use std::io::Stdout;
pub use std::result;
use crossterm::{
    event::{self, Event, KeyCode},
};
use tui::{
    backend::{CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block,Borders},
    Frame, 
    Terminal,
};

//pub mod secrets;
//pub mod accounts;
//pub mod bills;
//pub mod budgets;
//pub mod goals;

pub struct App {
}

impl App {
    pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
        loop {
            terminal.draw(|f| App::ui(f))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    //KeyCode::Right => app.next(),
                    //KeyCode::Left => app.previous(),
                    _ => {}
                }
            }
        }
    }

    fn ui(f: &mut Frame<CrosstermBackend<Stdout>>) {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
        f.render_widget(block, size);
    }
}
