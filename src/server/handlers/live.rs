use crate::server::{errors, packet::Packet, state::AppState, SocketResponse};
use axum::{
    extract::ws::{Message, WebSocket},
    http::StatusCode,
};
use futures::{SinkExt, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;

async fn send(socket: &mut (impl SinkExt<Message> + Unpin), resp: SocketResponse) {
    let text = serde_json::to_string(&resp).unwrap();
    let _ = socket.send(Message::Text(text)).await;
}

async fn authenticate(
    socket: &mut (impl SinkExt<Message> + Unpin),
    msg: &Message,
    state: &Arc<AppState>,
) -> Option<String> {
    match Packet::try_from(msg) {
        Ok(packet) => match packet.process(state, None).await {
            SocketResponse::Ready { token } => Some(token),
            SocketResponse::Error { message, code } => {
                let resp = SocketResponse::error(&message, StatusCode::from_u16(code).unwrap());
                send(socket, resp).await;
                None
            }
            _ => panic!("packet processed by handler other than identify"),
        },
        Err(e) => {
            let resp = SocketResponse::error(&e.to_string(), StatusCode::BAD_REQUEST);
            send(socket, resp).await;
            None
        }
    }
}

pub async fn callback(mut socket: WebSocket, state: Arc<AppState>) {
    let duration = Duration::from_millis(500);
    let req = tokio::time::timeout(duration, socket.recv()).await;
    match req {
        Ok(Some(Ok(msg))) => {
            if let Some(token) = authenticate(&mut socket, &msg, &state).await {
                let (mut tx, mut rx) = socket.split();
                let (sender, mut receiver) = mpsc::channel::<SocketResponse>(16);
                // Forward messages from the mpsc channel to the websocket sink.
                tokio::spawn(async move {
                    while let Some(resp) = receiver.recv().await {
                        let text = serde_json::to_string(&resp).unwrap();
                        if tx.send(Message::Text(text)).await.is_err() {
                            break;
                        }
                    }
                });
                // Let the client know that they are ready to receive messages.
                let _ = sender.send(SocketResponse::Ready { token }).await;
                // Listen for incoming messages from the client.
                while let Some(Ok(msg)) = rx.next().await {
                    let resp = match Packet::try_from(&msg) {
                        Ok(packet) => packet.process(&state, Some(sender.clone())).await,
                        Err(e) => SocketResponse::error(&e.to_string(), StatusCode::BAD_REQUEST),
                    };
                    let _ = sender.send(resp).await;
                }
            } else {
                let _ = socket.close().await;
            }
        }
        Ok(Some(Err(_))) | Ok(None) => {
            let _ = socket.close().await;
        }
        Err(_) => {
            let _ = send(
                &mut socket,
                SocketResponse::error(errors::IDENTIFY_TIMEOUT, StatusCode::REQUEST_TIMEOUT),
            )
            .await;
            let _ = socket.close().await;
        }
    }
}
