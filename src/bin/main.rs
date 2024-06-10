use olly::server;
use sea_orm::Database;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the database URL from the environment, or use the insecure default.
    let url = std::env::var("DATABASE_URL")
        .unwrap_or(String::from(server::INSECURE_DEFAULT_DATABASE_URL));
    // Connect to the database and bind a TCP listener to :3000.
    let database = Database::connect(url).await.unwrap();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // Serve the app on the port specified above.
    axum::serve(listener, server::app(database)).await.unwrap();
    Ok(())
}
