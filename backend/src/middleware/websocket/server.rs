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

use crate::app::models::client_event::{ClientInput, ClientKey, WireInput};
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
use tokio::sync::mpsc::UnboundedSender;
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
    let shared = sessions.clone();
    let mut manager = sessions.write().await;
    let id = manager.create_session(shared);
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
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    let sessions = sessions.read().await;
    let session = match sessions.get_session(&id) {
        Some(session) => session,
        None => return StatusCode::NOT_FOUND.into_response(),
    };
    let rx = session.output_tx.subscribe();
    let tx = session.input_tx.clone();

    ws.on_upgrade(move |socket| handle_socket(socket, rx, tx))
        .into_response()
}

async fn handle_socket(
    mut socket: WebSocket,
    mut rx: broadcast::Receiver<Vec<u8>>,
    tx: UnboundedSender<ClientInput>,
) {
    loop {
        tokio::select! {
            // tx to client
            Ok(bytes) = rx.recv() => {
                if socket.send(Message::Binary(bytes.into())).await.is_err() {
                    break;
                }
            }

            // rx from client
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        let text = text.to_string();
                        if let Some(input) = parse_input(text) {
                            let _ = tx.send(input);
                        }
                    }
                    Some(Ok(Message::Binary(_))) => {}
                    Some(Err(_)) | None => break,
                _ => {}
                }
            }
        }
    }
    tx.send(ClientInput::Disconnect).unwrap();
}

fn parse_input(text: String) -> Option<ClientInput> {
    let msg: WireInput = serde_json::from_str(&text).ok()?;

    match msg {
        WireInput::Key { key } => {
            let ck = match key.as_str() {
                "Enter" => ClientKey::Enter,
                "Escape" => ClientKey::Escape,
                "ArrowLeft" => ClientKey::ArrowLeft,
                "ArrowRight" => ClientKey::ArrowRight,
                "Tab" => ClientKey::Tab,
                c if c.len() == 1 => ClientKey::Char(c.chars().next().unwrap()),
                _ => return None,
            };

            Some(ClientInput::Key(ck))
        }

        WireInput::Resize { cols, rows } => Some(ClientInput::Resize { cols, rows }),
    }
}
