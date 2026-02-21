pub mod tui;

use color_eyre::Result;

use crate::tui::main::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}
