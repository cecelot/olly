use self::{packet::Packet, state::AppState};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::Response as AxumResponse,
    routing::get,
    Router,
};

mod packet;
mod state;

pub use packet::Response; // is this poor design?

pub fn app() -> Router {
    Router::new().route("/live", get(handler).with_state(AppState::new()))
}

async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> AxumResponse {
    ws.on_upgrade(|socket| callback(socket, state))
}

async fn callback(mut socket: WebSocket, state: AppState) {
    while let Some(Ok(msg)) = socket.recv().await {
        let resp = Packet::try_from(msg).map_or_else(
            |e| Response::Error {
                message: e.to_string(),
                code: StatusCode::BAD_REQUEST.into(),
            },
            |packet| packet.process(&state),
        );
        let text = serde_json::to_string(&resp).unwrap();
        let _ = socket.send(Message::Text(text)).await;
    }
}
