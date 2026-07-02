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
use crate::app::api_poller::spawn_poller;
use crate::app::models::api::api_state::{ApiState, SharedApiState};
use crate::app::models::api::stats::StatsConf;
use crate::app::models::client_event::ClientInput;
use crate::app::models::state::App;
use std::env;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast, mpsc};
use tracing::Instrument;
use uuid::Uuid;

pub struct Session {
    pub id: Uuid,
    pub app: App,
    pub output_tx: broadcast::Sender<Vec<u8>>,

    pub input_tx: mpsc::UnboundedSender<ClientInput>,
    pub input_rx: mpsc::UnboundedReceiver<ClientInput>,

    pub api_state: SharedApiState,
}

pub struct SessionHandle {
    pub id: Uuid,
    pub input_tx: mpsc::UnboundedSender<ClientInput>,
    pub output_tx: broadcast::Sender<Vec<u8>>,
}

impl Session {
    pub fn new(id: Uuid) -> (Self, SessionHandle) {
        let (output_tx, _) = broadcast::channel(200);
        let (input_tx, input_rx) = mpsc::unbounded_channel();
        let api_state: SharedApiState = Arc::new(RwLock::new(ApiState::default()));
        let api_state_poller = Arc::clone(&api_state);
        let api_client = reqwest::Client::new();

        let stats_cfg = StatsConf {
            base_url: env::var("HACKATIME_URL")
                .unwrap_or_else(|_| "https://hackatime.hackclub.com".to_string()),

            username: env::var("HACKATIME_USERNAME").ok(),

            api_key: env::var("HACKATIME_STATS_API_KEY").ok(),
        };

        tokio::spawn(
            async move {
                spawn_poller(api_state_poller, api_client, stats_cfg).await;
            }
            .instrument(tracing::info_span!("API Poller", id = %id)),
        );

        let app = App {
            api_state: api_state.clone(),
            ..App::default()
        };

        let session = Session {
            id,
            app,
            input_tx: input_tx.clone(),
            input_rx,
            output_tx: output_tx.clone(),
            api_state,
        };

        let handle = SessionHandle {
            id,
            input_tx,
            output_tx,
        };

        (session, handle)
    }
}
