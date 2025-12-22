use std::io::stdout;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use tui::run;
fn main() -> Result<()> {
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;
    result
}
