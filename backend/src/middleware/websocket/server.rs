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