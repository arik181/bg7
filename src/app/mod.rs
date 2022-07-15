pub use std::{error::Error, io};
pub use std::io::Stdout;
pub use std::result;
use crossterm::{
    event::{self, Event, KeyCode},
};
use tui::{
    backend::{CrosstermBackend},
    layout::{Rect},
    style::{Color, Style},
    widgets::{Block,Borders,BorderType},
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

        let block = Block::default()
            .style(Style::default().bg(Color::Rgb(0,0,0)).fg(Color::Rgb(0,255,0)))
            .title("bg7")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let inner = Block::default()
            .style(Style::default().bg(Color::Rgb(0,0,0)).fg(Color::Rgb(0,255,0)))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let innersize = 
            Rect { 
                x : (size.width / 2) - 20,
                y : (size.height / 2) - 4,
                width : 40,
                height : 10,
            };

        let text = Block::default().title("Hello.");
        let textsize = 
            Rect {
                x : (innersize.width / 2) - 3 + innersize.x,
                y : (innersize.height / 2) - 1 + innersize.y,
                width : 6 as u16,
                height : 1 as u16,
            };
            

        f.render_widget(block, size);
        f.render_widget(inner, innersize);
        f.render_widget(text, textsize);
    }
}
