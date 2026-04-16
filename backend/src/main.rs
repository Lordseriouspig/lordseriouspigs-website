pub mod tui;
pub mod app;
pub mod middleware;

use color_eyre::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::broadcast;

use crate::app::models::state::App;
use crate::middleware::websocket::{writer::WsWriter, server::start_server};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // Create websocket channel (PLACEHOLDER)
    let (tx, _) = broadcast::channel::<Vec<u8>>(100);

    // Wire up my writer
    let writer = WsWriter { tx: tx.clone() };

    // custom backend
    let backend = CrosstermBackend::new(writer);
    
    // fire up the terminal
    let terminal = Terminal::new(backend)?;
    tokio::spawn(start_server(tx));
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}
