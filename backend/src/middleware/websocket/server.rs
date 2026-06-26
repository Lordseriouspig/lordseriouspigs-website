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

use crate::app::models::sessions::shared_sessions::SharedSessions;
use axum::{
    Router,
    extract::{
        Json, Path, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Serialize;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Serialize)]
struct CreateSessionResponse {
    session_id: String,
}

pub async fn start_server(sessions: SharedSessions) {
    let app = Router::new()
        .route("/api/session", post(create_session))
        .route("/ws/{id}", get(ws_handler))
        .with_state(sessions);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    println!(
        "Server listening on port {}!",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn create_session(State(sessions): State<SharedSessions>) -> Json<CreateSessionResponse> {
    let mut sessions = sessions.write().await;
    let id = sessions.create_session();
    Json(CreateSessionResponse {
        session_id: id.to_string(),
    })
}

async fn ws_handler(
    Path(id): Path<String>,
    State(sessions): State<SharedSessions>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST).into_response(),
    };
    let sessions = sessions.read().await;
    let session = match sessions.get_session(&id) {
        Some(session) => session,
        None => return (StatusCode::NOT_FOUND).into_response(),
    };
    let rx = session.terminal_tx.subscribe();

    ws.on_upgrade(move |socket| handle_socket(socket, rx))
        .into_response()
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<Vec<u8>>) {
    loop {
        match rx.recv().await {
            Ok(bytes) => {
                if socket.send(Message::Binary(bytes.into())).await.is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}
