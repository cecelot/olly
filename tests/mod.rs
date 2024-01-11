use futures::{SinkExt, StreamExt};
use othello::server::Reply;
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

async fn send(socket: &mut WebSocket, msg: &str) -> Reply {
    socket.send(Message::text(msg)).await.unwrap();
    match socket.next().await.unwrap().unwrap() {
        Message::Text(msg) => serde_json::from_str(&msg).unwrap(),
        other => panic!("expected a text message but got {other:?}"),
    }
}

#[tokio::test]
async fn create() {
    let mut socket = init().await;
    let msg = send(&mut socket, "{\"op\":\"Create\"}").await;
    assert!(matches!(msg, Reply::Created { .. }));
}

#[tokio::test]
async fn bad_placement() {
    let mut socket = init().await;
    let id = match send(&mut socket, "{\"op\":\"Create\"}").await {
        Reply::Created { id } => id,
        msg => panic!("expected a created message but got {msg:?}"),
    };
    let data = format!(
            "{{\"op\":\"Place\", \"data\":{{\"id\": \"{}\", \"x\": 3, \"y\": 3, \"piece\": \"Black\"}}}}",
            id
        );
    let msg = send(&mut socket, &data).await;
    assert_eq!(
        msg,
        Reply::Error {
            message: "board square (3, 3) is occupied".into(),
            code: 400
        }
    );
}

#[tokio::test]
async fn valid_placement() {
    let mut socket = init().await;
    let id = match send(&mut socket, "{\"op\":\"Create\"}").await {
        Reply::Created { id } => id,
        msg => panic!("expected a created message but got {msg:?}"),
    };
    let data = format!(
            "{{\"op\":\"Place\", \"data\":{{\"id\": \"{}\", \"x\": 2, \"y\": 3, \"piece\": \"Black\"}}}}",
            id
        );
    let msg = send(&mut socket, &data).await;
    assert!(matches!(msg, Reply::State(..)));
}
