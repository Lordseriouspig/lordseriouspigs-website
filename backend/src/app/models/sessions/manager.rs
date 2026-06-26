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
use crate::app::models::sessions::session::Session;
use crate::middleware::websocket::writer::WsWriter;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::collections::HashMap;
use tokio::task;
use uuid::Uuid;

pub struct SessionManager {
    sessions: HashMap<Uuid, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        let session = Session::new(id);

        let tx = session.terminal_tx.clone();
        let app = session.app.clone();

        task::spawn(async move {
            let writer = WsWriter {
                tx,
                buffer: Vec::new(),
            };

            let backend = CrosstermBackend::new(writer);
            let terminal = Terminal::new(backend).unwrap();

            let _ = app.run(terminal);
        });

        self.sessions.insert(id, session);

        id
    }

    pub fn get_session(&self, id: &Uuid) -> Option<&Session> {
        self.sessions.get(id)
    }
}
