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
use crate::app::models::sessions::session::{Session, SessionHandle};
use crate::app::models::sessions::shared_sessions::SharedSessions;
use std::collections::HashMap;
use uuid::Uuid;

pub struct SessionManager {
    sessions: HashMap<Uuid, SessionHandle>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, shared: SharedSessions) -> Uuid {
        let id = Uuid::new_v4();
        let (session, handle) = Session::new(id);

        self.sessions.insert(id, handle);

        tokio::spawn(async move {
            session.run().await;
            println!("Destroying session {}", id);

            let mut sessions = shared.write().await;
            sessions.destroy_session(&id);
        });

        id
    }

    pub fn get_session(&self, id: &Uuid) -> Option<&SessionHandle> {
        self.sessions.get(id)
    }

    pub fn destroy_session(&mut self, id: &Uuid) {
        self.sessions.remove(id);
    }
}
