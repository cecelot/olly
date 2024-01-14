// // TODO: Figure out a robust testing solution.
// // use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
// use futures::{SinkExt, StreamExt};
// use othello::server::{/* member, */ Response};
// // use rand::rngs::OsRng;
// // use reqwest::StatusCode;
// use sea_orm::{Database /* DatabaseBackend, MockDatabase, MockExecResult */};
// use serde_json::{json, Value};
// use std::{
//     future::IntoFuture,
//     net::{Ipv4Addr, SocketAddr},
// };
// use tokio::net::{TcpListener, TcpStream};
// use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
// // use uuid::Uuid;

// type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

// struct TestContext {
//     socket: WebSocket,
//     token: Option<String>,
// }

// impl TestContext {
//     async fn new() -> Self {
//         let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
//             .await
//             .unwrap();
//         let addr = listener.local_addr().unwrap();
//         let database =
//             Database::connect("postgres://othello-server:password@0.0.0.0:5432/othello-server")
//                 .await
//                 .unwrap();
//         // let (query_results, exec_results) = Self::mock_results().await;
//         // let database = MockDatabase::new(DatabaseBackend::Postgres)
//         //     .append_query_results(query_results)
//         //     .append_exec_results(exec_results)
//         //     .into_connection();
//         tokio::spawn(axum::serve(listener, othello::server::app(database)).into_future());
//         let (socket, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/live"))
//             .await
//             .unwrap();
//         // Self::create_user(addr).await;
//         Self {
//             socket,
//             token: None,
//         }
//     }

//     // async fn mock_results() -> (Vec<Vec<member::Model>>, Vec<MockExecResult>) {
//     //     let id = Uuid::now_v7();
//     //     let query_results = vec![vec![member::Model {
//     //         id,
//     //         username: "alaidriel".into(),
//     //         password: {
//     //             let salt = SaltString::generate(&mut OsRng);
//     //             let argon2 = Argon2::default();
//     //             argon2.hash_password(b"meow", &salt).unwrap().to_string()
//     //         },
//     //     }]];
//     //     let exec_results = vec![
//     //         MockExecResult {
//     //             last_insert_id: 1,
//     //             rows_affected: 1,
//     //         },
//     //         MockExecResult {
//     //             last_insert_id: 1,
//     //             rows_affected: 1,
//     //         },
//     //     ];
//     //     (query_results, exec_results)
//     // }

//     // async fn create_user(addr: SocketAddr) -> String {
//     //     let body = json!({"username": "unicorn", "password": "rainbow"});
//     //     let body = serde_json::to_string(&body).unwrap();
//     //     let resp = reqwest::Client::new()
//     //         .post(format!("http://{addr}/register"))
//     //         .header("Content-Type", "application/json")
//     //         .body(body)
//     //         .send()
//     //         .await;
//     //     let status = resp.as_ref().unwrap().status();
//     //     let text = resp.unwrap().text().await.unwrap();
//     //     let resp: Response = serde_json::from_str(&text).unwrap();
//     //     assert_eq!(status, StatusCode::CREATED);
//     //     assert!(matches!(resp, Response::Created { .. }));
//     //     match resp {
//     //         Response::Created { id } => id,
//     //         _ => panic!("expected a created message but got {resp:?}"),
//     //     }
//     // }

//     async fn create_game(&mut self) -> String {
//         let msg = self
//             .send(json!({ "op": 1, "d": {
//                 "guest": "unicorn"
//             }, "t": self.token.clone().unwrap() }))
//             .await;
//         assert!(matches!(msg, Response::Created { .. }));
//         match msg {
//             Response::Created { id } => id,
//             _ => panic!("expected a created message but got {msg:?}"),
//         }
//     }

//     async fn authenticate(&mut self) {
//         let msg = self
//             .send(json!({
//                 "op": 6,
//                 "d": {
//                     "username": "alaidriel",
//                     "password": "meow"
//                 }
//             }))
//             .await;
//         assert!(matches!(msg, Response::Ready { .. }));
//         self.token = match msg {
//             Response::Ready { token } => Some(token),
//             _ => panic!("expected a ready message but got {msg:?}"),
//         };
//     }

//     async fn send(&mut self, msg: Value) -> Response {
//         let string = serde_json::to_string(&msg).unwrap();
//         self.socket.send(Message::text(string)).await.unwrap();
//         self.receive().await
//     }

//     async fn receive(&mut self) -> Response {
//         match self.socket.next().await.unwrap().unwrap() {
//             Message::Text(msg) => serde_json::from_str(&msg).unwrap(),
//             other => panic!("expected a text message but got {other:?}"),
//         }
//     }
// }

// #[tokio::test]
// async fn register() {
//     TestContext::new().await;
// }

// #[tokio::test]
// async fn auth_timeout() {
//     let mut context = TestContext::new().await;
//     let msg = context.receive().await;
//     assert_eq!(
//         msg,
//         Response::Error {
//             message: "connection timed out".into(),
//             code: 408
//         }
//     );
// }

// #[tokio::test]
// async fn out_of_order() {
//     let mut context = TestContext::new().await;
//     let msg = context
//         .send(json!({
//             "op": 1,
//             "d": {
//                 "guest": "unicorn",
//             },
//             "t": "hello"
//         }))
//         .await;
//     dbg!(&msg);
// }

// #[tokio::test]
// async fn auth() {
//     let mut context = TestContext::new().await;
//     context.authenticate().await;
//     assert!(matches!(context.token, Some(..)));
// }

// #[tokio::test]
// async fn create() {
//     let mut context = TestContext::new().await;
//     context.authenticate().await;
//     context.create_game().await;
// }

// #[tokio::test]
// async fn bad_placement() {
//     let mut context = TestContext::new().await;
//     context.authenticate().await;
//     let id = context.create_game().await;
//     let msg = context
//         .send(json!({
//             "op": 2,
//             "t": context.token.clone().unwrap(),
//             "d": {
//                 "id": id,
//                 "x": 3,
//                 "y": 3,
//                 "piece": "Black"
//             }
//         }))
//         .await;
//     assert_eq!(
//         msg,
//         Response::Error {
//             message: "board square (3, 3) is occupied".into(),
//             code: 400
//         }
//     );
// }

// #[tokio::test]
// async fn valid_placement() {
//     let mut context = TestContext::new().await;
//     context.authenticate().await;
//     let id = context.create_game().await;
//     let initial = context
//         .send(json!({
//             "op": 3,
//             "t": context.token.clone().unwrap(),
//             "d": {
//                 "id": id
//             }
//         }))
//         .await;
//     dbg!(&initial);
//     assert!(matches!(initial, Response::State(..)));
//     let placed = context
//         .send(json!({
//             "op": 2,
//             "t": context.token.clone().unwrap(),
//             "d": {
//                 "id": id,
//                 "x": 2,
//                 "y": 3,
//                 "piece": "Black"
//             }
//         }))
//         .await;
//     assert!(matches!(placed, Response::Ok));
//     let updated = context.receive().await;
//     dbg!(&updated);
//     assert!(matches!(updated, Response::State(..)));
// }
