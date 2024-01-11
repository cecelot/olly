use othello::server;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server::app()).await.unwrap();
    Ok(())
}
