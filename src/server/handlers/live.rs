use crate::server::{errors, packet::Packet, state::AppState, Response};
use axum::{
    extract::ws::{Message, WebSocket},
    http::StatusCode,
};
use futures::{SinkExt, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;

async fn send(socket: &mut (impl SinkExt<Message> + Unpin), resp: Response) {
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
            Response::Ready { token } => Some(token),
            // TODO: this will show errors from other handlers in addition to the identify handler.
            Response::Error { message, code } => {
                let resp = Response::error(&message, StatusCode::from_u16(code).unwrap());
                send(socket, resp).await;
                None
            }
            _ => {
                // The packet was processed by a handler that wasn't the identify handler.
                let _ = send(
                    socket,
                    Response::error(errors::UNAUTHORIZED_CONNECTION, StatusCode::UNAUTHORIZED),
                )
                .await;
                None
            }
        },
        Err(e) => {
            let resp = Response::error(&e.to_string(), StatusCode::BAD_REQUEST);
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
                let (sender, mut receiver) = mpsc::channel::<Response>(16);
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
                let _ = sender.send(Response::Ready { token }).await;
                // Listen for incoming messages from the client.
                while let Some(Ok(msg)) = rx.next().await {
                    let resp = match Packet::try_from(&msg) {
                        Ok(packet) => packet.process(&state, Some(sender.clone())).await,
                        Err(e) => Response::error(&e.to_string(), StatusCode::BAD_REQUEST),
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
                Response::error(errors::IDENTIFY_TIMEOUT, StatusCode::REQUEST_TIMEOUT),
            )
            .await;
            let _ = socket.close().await;
        }
    }
}
