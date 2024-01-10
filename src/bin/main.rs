use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::Response,
    routing::get,
    Router,
};
use othello::{Game, Piece};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use uuid::{timestamp::context, Timestamp, Uuid};

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("axum: {0}")]
    Axum(#[from] axum::Error),
    #[error("serde_json: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
enum Opcode {
    Create,
    Join,
    Place,
    Reset,
    Leave,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Data {
    Place {
        id: String,
        x: usize,
        y: usize,
        piece: Piece,
    },
    Join {
        id: String,
    },
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum Reply {
    Error { message: String, code: u16 },
    Created { id: String },
    State(Game),
    Ok,
}

#[derive(Debug, Serialize, Deserialize)]
struct Packet {
    op: Opcode,
    data: Option<Data>,
}

#[derive(Clone)]
struct AppState {
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
}

async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| callback(socket, state))
}

fn parse(msg: &Message) -> Result<Packet, ParseError> {
    let s = msg.to_text().map_err(ParseError::Axum)?;
    let packet = serde_json::from_str(s).map_err(ParseError::Serde)?;
    Ok(packet)
}

async fn callback(mut socket: WebSocket, state: AppState) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };
        let packet = match parse(&msg) {
            Ok(packet) => packet,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        dbg!(&packet);
        let reply = match packet.op {
            Opcode::Create => {
                let game = Game::new();
                let id = Uuid::new_v7(Timestamp::now(context::NoContext));
                let mut games = state.games.lock().expect("mutex was poisoned");
                games.insert(id, game);
                Reply::Created { id: id.to_string() }
            }
            Opcode::Join => todo!(),
            Opcode::Place => {
                let mut games = state.games.lock().expect("mutex was poisoned");
                let (id, x, y, piece) = match packet.data.unwrap() {
                    Data::Place { id, x, y, piece } => (id, x, y, piece),
                    _ => continue,
                };
                let game = games.get_mut(&Uuid::from_str(&id).unwrap()).unwrap();
                if let Err(e) = game.place(x, y, piece) {
                    Reply::Error {
                        message: e.to_string(),
                        code: StatusCode::BAD_REQUEST.into(),
                    }
                } else {
                    Reply::State(game.clone())
                }
            }
            Opcode::Reset => todo!(),
            Opcode::Leave => todo!(),
        };
        let reply = serde_json::to_string(&reply).unwrap();
        if socket.send(Message::Text(reply)).await.is_err() {
            // client disconnected
            return;
        }
    }
}

fn app() -> Router {
    Router::new().route(
        "/live",
        get(handler).with_state(AppState {
            games: Arc::new(Mutex::new(HashMap::new())),
        }),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Reply;
    use futures::{SinkExt, StreamExt};
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
        tokio::spawn(axum::serve(listener, super::app()).into_future());
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
    async fn placement() {
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
}
