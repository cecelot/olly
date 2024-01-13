use futures::{SinkExt, StreamExt};
use othello::server::{member, Response};
use reqwest::StatusCode;
use sea_orm::{DatabaseBackend, MockDatabase};
use serde_json::{json, Value};
use std::{
    future::IntoFuture,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

async fn init() -> (WebSocket, SocketAddr) {
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    let database = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    tokio::spawn(axum::serve(listener, othello::server::app(database)).into_future());
    let (socket, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/live"))
        .await
        .unwrap();
    (socket, addr)
}

async fn send(socket: &mut WebSocket, msg: Value) -> Response {
    let string = serde_json::to_string(&msg).unwrap();
    socket.send(Message::text(string)).await.unwrap();
    receive(socket).await
}

async fn receive(socket: &mut WebSocket) -> Response {
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

#[tokio::test]
#[ignore = "mock database not yet implemented"]
async fn register() {
    let (_, addr) = init().await;
    let body = json!({"username": "alaidriel", "password": "meow"});
    let body = serde_json::to_string(&body).unwrap();
    let resp = reqwest::Client::new()
        .post(format!("http://{addr}/register"))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    let status = resp.as_ref().unwrap().status();
    let text = resp.unwrap().text().await.unwrap();
    let user: member::Model = serde_json::from_str(&text).unwrap();
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(user.username, "alaidriel");
}

#[tokio::test]
async fn auth_timeout() {
    let (mut socket, _) = init().await;
    let msg = receive(&mut socket).await;
    assert_eq!(
        msg,
        Response::Error {
            message: "connection timed out".into(),
            code: 408
        }
    );
}

#[tokio::test]
async fn auth() {
    let (mut socket, _) = init().await;
    let msg = send(
        &mut socket,
        json!({
            "op": 6,
            "t": "Identify",
            "d": {
                "username": "test",
                "password": "test"
            }
        }),
    )
    .await;
    assert!(matches!(msg, Response::Ready { .. }))
}

/// This is an edge case with serde_json's deserialization of packets that
/// causes deserialization to succeed with an empty data field (resulting in
/// a panic when the packet is processed).
#[tokio::test]
async fn malformed_packet() {
    let (mut socket, _) = init().await;
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
#[ignore = "requires websocket authentication flow"]
async fn create() {
    let (mut socket, _) = init().await;
    let msg = send(&mut socket, json!({ "op": 1 })).await;
    assert!(matches!(msg, Response::Created { .. }));
}

#[tokio::test]
#[ignore = "requires websocket authentication flow"]
async fn bad_placement() {
    let (mut socket, _) = init().await;
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
#[ignore = "requires websocket authentication flow"]
async fn valid_placement() {
    let (mut socket, _) = init().await;
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
