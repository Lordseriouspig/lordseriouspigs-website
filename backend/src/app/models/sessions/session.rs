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
use crate::app::models::client_event::ClientInput;
use crate::app::models::state::App;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

pub struct Session {
    pub id: Uuid,
    pub app: App,
    pub output_tx: broadcast::Sender<Vec<u8>>,

    pub input_tx: mpsc::UnboundedSender<ClientInput>,
    pub input_rx: mpsc::UnboundedReceiver<ClientInput>,
}

pub struct SessionHandle {
    pub id: Uuid,
    pub input_tx: mpsc::UnboundedSender<ClientInput>,
    pub output_tx: broadcast::Sender<Vec<u8>>,
}

impl Session {
    pub fn new(id: Uuid) -> (Self, SessionHandle) {
        let (output_tx, _) = broadcast::channel(100);
        let (input_tx, input_rx) = mpsc::unbounded_channel();

        let session = Session {
            id,
            app: App::default(),
            input_tx: input_tx.clone(),
            input_rx,
            output_tx: output_tx.clone(),
        };

        let handle = SessionHandle {
            id,
            input_tx,
            output_tx,
        };

        (session, handle)
    }
}
