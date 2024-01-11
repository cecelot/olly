use futures::{SinkExt, StreamExt};
use othello::server::Response;
use serde_json::{json, Value};
use std::{
    future::IntoFuture,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

async fn init() -> WebSocket {
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(axum::serve(listener, othello::server::app()).into_future());
    let (socket, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/live"))
        .await
        .unwrap();
    socket
}

async fn send(socket: &mut WebSocket, msg: Value) -> Response {
    let string = serde_json::to_string(&msg).unwrap();
    socket.send(Message::text(string)).await.unwrap();
    match socket.next().await.unwrap().unwrap() {
        Message::Text(msg) => serde_json::from_str(&msg).unwrap(),
        other => panic!("expected a text message but got {other:?}"),
    }
}

async fn create_state(socket: &mut WebSocket) -> String {
    let msg = send(socket, json!({ "op": 1 })).await;
    match msg {
        Response::Created { id } => id,
        msg => panic!("expected a created message but got {msg:?}"),
    }
}

/// This is an edge case with serde_json's deserialization of packets that
/// causes deserialization to succeed with an empty data field (resulting in
/// a panic when the packet is processed).
#[tokio::test]
async fn malformed_packet() {
    let mut socket = init().await;
    let msg = send(
        &mut socket,
        json!({
            "op": 2,
            "t": "Place",
            "data": {}
        }),
    )
    .await;
    assert_eq!(
        msg,
        Response::Error {
            message: "missing field `d`".into(),
            code: 400
        }
    )
}

#[tokio::test]
async fn create() {
    let mut socket = init().await;
    let msg = send(&mut socket, json!({ "op": 1 })).await;
    assert!(matches!(msg, Response::Created { .. }));
}

#[tokio::test]
async fn bad_placement() {
    let mut socket = init().await;
    let id = create_state(&mut socket).await;
    let data = json!({
        "op": 2,
        "t": "Place",
        "d": {
            "id": id,
            "x": 3,
            "y": 3,
            "piece": "Black"
        }
    });
    let msg = send(&mut socket, data).await;
    assert_eq!(
        msg,
        Response::Error {
            message: "board square (3, 3) is occupied".into(),
            code: 400
        }
    );
}

#[tokio::test]
async fn valid_placement() {
    let mut socket = init().await;
    let id = create_state(&mut socket).await;
    let data = json!({
        "op": 2,
        "t": "Place",
        "d": {
            "id": id,
            "x": 2,
            "y": 3,
            "piece": "Black"
        }
    });
    let msg = send(&mut socket, data).await;
    assert!(matches!(msg, Response::State(..)));
}
