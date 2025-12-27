use anyhow::Result;

use tui::{log::init_log, run};

fn main() -> Result<()> {
    init_log()?;
    rust_i18n::set_locale("zh-cn");
    run()
}
