use std::io::stdout;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use tui::{log::init_log, run};

fn main() -> Result<()> {
    init_log()?;
    rust_i18n::set_locale("zh-cn");
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;
    result
}
