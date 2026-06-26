/*
 * Copyright (C) 2026 Lordseriouspig
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod app;
pub mod middleware;
pub mod tui;

use color_eyre::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::app::models::sessions::manager::SessionManager;
use crate::middleware::websocket::server::start_server;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let sessions = Arc::new(RwLock::new(SessionManager::new()));

    start_server(sessions).await;

    Ok(())
}
