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
use dotenvy::from_filename;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::EnvFilter;

use crate::app::models::sessions::manager::SessionManager;
use crate::middleware::websocket::server::start_server;

#[tokio::main]
async fn main() -> Result<()> {
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    match environment.as_str() {
        "production" => {
            from_filename(".env.production").expect(".env.production file not found");
        }
        _ => {
            from_filename(".env.development").expect(".env.development file not found");
        }
    }

    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!(environment, "Starting!");

    let sessions = Arc::new(RwLock::new(SessionManager::new()));

    start_server(sessions).await?;

    Ok(())
}
