pub mod tui;
pub mod app;
pub mod middleware;

use color_eyre::Result;

use crate::app::models::state::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}
