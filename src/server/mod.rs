use self::{packet::Packet, state::AppState};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};

mod packet;
mod state;

pub use packet::Reply; // is this poor design?

pub fn app() -> Router {
    Router::new().route("/live", get(handler).with_state(AppState::new()))
}

async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| callback(socket, state))
}

async fn callback(mut socket: WebSocket, state: AppState) {
    while let Some(Ok(msg)) = socket.recv().await {
        let packet = match Packet::try_from(msg) {
            Ok(packet) => packet,
            Err(e) => {
                // client disconnected on failure
                let _ = socket.send(Message::Text(e.to_string())).await;
                continue;
            }
        };
        let reply = packet.process(&state);
        let text = serde_json::to_string(&reply).unwrap();
        let _ = socket.send(Message::Text(text)).await;
    }
}
