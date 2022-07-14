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

    let t = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();
    App::new();

    App::run();

    thread::sleep(Duration::new(5,0));

    cleanup_term(t)?;

    println!("We done!");
    Ok(())
}
