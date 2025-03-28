use axum::{
    Router,
    extract::{WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
    routing::any,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", any(ws_handler));

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3000))
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    sender
        .send(axum::extract::ws::Message::Text("Hi".into()))
        .await
        .unwrap();
}
