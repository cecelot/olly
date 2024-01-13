use othello::server;
use sea_orm::Database;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FIXME: use env vars
    let database =
        Database::connect("postgres://othello-server:password@0.0.0.0:5432/othello-server")
            .await
            .unwrap();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server::app(database)).await.unwrap();
    Ok(())
}
