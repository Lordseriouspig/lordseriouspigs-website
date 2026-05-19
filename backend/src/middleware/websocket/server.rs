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

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tokio::sync::oneshot;

pub async fn start_server(tx: broadcast::Sender<Vec<u8>>, ready_tx: oneshot::Sender<()>) {
    let ready_tx = Arc::new(Mutex::new(Some(ready_tx)));
    let socket = Router::new().route(
        "/ws", 
        get(move |ws: WebSocketUpgrade| {
            let tx = tx.clone();
            let ready_tx = ready_tx.clone();
            async move {
                ws.on_upgrade(move |socket| handle_socket(socket, tx.subscribe(), ready_tx))
            }
        }),
    );

    eprintln!("WebSocket server running");
    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, socket).await.unwrap();
}

async fn handle_socket(
    mut socket: WebSocket,
    mut rx: broadcast::Receiver<Vec<u8>>,
    ready_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
) {
    let mut browser_ready = false;

    loop {
        tokio::select! {
            message = socket.recv() => {
                match message {
                    Some(Ok(Message::Text(text))) if text == "browser-ready" && !browser_ready => {
                        browser_ready = true;
                        if let Some(ready_tx) = ready_tx.lock().unwrap().take() {
                            let _ = ready_tx.send(());
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
            bytes = rx.recv() => {
                match bytes {
                    Ok(bytes) => {
                        eprintln!("Received bytes: {:?}", bytes);
                        if socket.send(Message::Binary(bytes.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        eprintln!("Client lagged, dropped {} messages", n);
                        // skip and continue; don't tear down the websocket for slow clients
                        continue;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        eprintln!("Broadcast channel closed");
                        break;
                    }
                }
            }
        }
    }
}