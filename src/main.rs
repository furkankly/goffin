use game_of_life::terminal;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = terminal::setup_terminal()?;
    terminal::run(&mut terminal)?;
    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}
