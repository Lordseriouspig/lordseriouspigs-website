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
use tokio::sync::broadcast;
use std::net::SocketAddr;

pub async fn start_server(tx: broadcast::Sender<Vec<u8>>) {
    let socket = Router::new().route(
        "/ws", 
        get(move |ws: WebSocketUpgrade| {
            let tx = tx.clone();
            async move { ws. on_upgrade(move |socket| handle_socket(socket, tx.subscribe())) }
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
) {
    while let Ok(bytes) = rx.recv().await {
        eprintln!("Received bytes: {:?}", bytes);
        if socket.send(Message::Binary(bytes.into())).await.is_err() {
            break;
        }
    }
}