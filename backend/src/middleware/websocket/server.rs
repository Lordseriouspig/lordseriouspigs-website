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
use color_eyre::Result;
use serde::Serialize;
use tokio::sync::broadcast;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{Instrument, info_span};
use uuid::Uuid;

#[derive(Serialize)]
struct CreateSessionResponse {
    session_id: String,
}

pub async fn start_server(sessions: SharedSessions) -> Result<()> {
    let app = Router::new()
        .route("/api/session", post(create_session))
        .route("/ws/{id}", get(ws_handler))
        .with_state(sessions);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await?;
    tracing::info!(address = %listener.local_addr()?, "Server listening!!");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn create_session(State(sessions): State<SharedSessions>) -> Json<CreateSessionResponse> {
    tracing::debug!("POST /api/session");
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
    tracing::debug!(id = ?id, "GET /ws/{id}");
    let sessions = sessions.read().await;
    let session = match sessions.get_session(&id) {
        Some(session) => session,
        None => return StatusCode::NOT_FOUND.into_response(),
    };
    let rx = session.output_tx.subscribe();
    let tx = session.input_tx.clone();

    ws.on_upgrade(move |socket| {
        handle_socket(socket, rx, tx).instrument(info_span!("socket", session_id = %id))
    })
    .into_response()
}

async fn handle_socket(
    mut socket: WebSocket,
    mut rx: broadcast::Receiver<Vec<u8>>,
    tx: UnboundedSender<ClientInput>,
) {
    tracing::debug!("ws upgrade");
    loop {
        tokio::select! {
            // tx to client
            result = rx.recv() => {
                match result {
                    Ok(bytes) => {
                        tracing::trace!(msg = ?bytes, "tx to client");
                        if let Err(e) = socket.send(Message::Binary(bytes.into())).await {
                            tracing::warn!(error = %e, "Error sending client message");
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!(messages = n, "Broadcast receiver lagged");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        tracing::error!("Broadcast channel closed");
                        break;
                    }
                }
            }

            // rx from client
            msg = socket.recv() => {
                tracing::trace!(msg = ?msg, "rx from client");
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        let text = text.to_string();
                        match parse_input(&text) {
                            Some(input) => {
                                if let Err(err) = tx.send(input) {
                                    tracing::warn!(%err, "Error forwarding client input");
                                    break;
                                };
                            }

                            None => {
                                tracing::warn!(message = %text, "Invalid client message");
                            }
                        }
                    }
                    Some(Ok(Message::Binary(_))) => {
                        tracing::debug!("Ignoring binary message");
                    }
                    Some(Ok(Message::Close(_))) => {
                        tracing::info!("Client requested close");
                        break;
                    }
                    Some(Err(err)) => {
                        tracing::warn!(%err, "Websocket error");
                        break;
                    }
                    None => {
                        tracing::info!("Websocket stream ended");
                        break;
                    }
                    _ => {
                        tracing::debug!("Ignoring unknown websocket message");
                    }
                }
            }
        }
    }
    tracing::debug!("ws disconnect");
    if let Err(err) = tx.send(ClientInput::Disconnect) {
        tracing::warn!(%err, "Error sending disconnect message");
    };
}

fn parse_input(text: &String) -> Option<ClientInput> {
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
        WireInput::Ready => Some(ClientInput::Ready),
    }
}
