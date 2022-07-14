use std::io;
use std::thread;
use std::time::Duration;
use std::error::Error;
use tui::{
    backend::{CrosstermBackend},
    Terminal,
};

pub mod app;
pub mod term;

use app::App;
use term::setup_term;
use term::cleanup_term;

fn main() -> Result<(),Box<dyn Error>> {

    setup_term()?; 

    let mut t = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();

    App::run(&mut t)?;

    cleanup_term(t)?;

    println!("We done!");
    Ok(())
}
