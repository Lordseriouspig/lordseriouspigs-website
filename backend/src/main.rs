// Copyright (C) 2026 Lordseriouspig
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod tui;
pub mod app;
pub mod middleware;

use color_eyre::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::{broadcast, oneshot};

use crate::app::models::state::App;
use crate::middleware::websocket::{writer::WsWriter, server::start_server};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // Create websocket channel (PLACEHOLDER)
    let (tx, _) = broadcast::channel::<Vec<u8>>(100);
    let (ready_tx, ready_rx) = oneshot::channel::<()>();

    tokio::spawn(start_server(tx.clone(), ready_tx));

    // Wait until the first websocket client connects before opening the TUI.
    let _ = ready_rx.await;

    // Wire up my writer
    let writer = WsWriter { tx: tx.clone(), buffer: Vec::new() };

    // custom backend
    let backend = CrosstermBackend::new(writer);

    // fire up the terminal
    let terminal = Terminal::new(backend)?;
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}
