use super::Response;
use crate::Game;
use sea_orm::DatabaseConnection;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub(super) games: Arc<Mutex<HashMap<Uuid, Game>>>,
    pub(super) rooms: Arc<Mutex<HashMap<Uuid, broadcast::Sender<Response>>>>,
    pub(super) database: Arc<DatabaseConnection>,
}

impl AppState {
    pub fn new(database: DatabaseConnection) -> Self {
        Self {
            games: Arc::new(Mutex::new(HashMap::new())),
            rooms: Arc::new(Mutex::new(HashMap::new())),
            database: Arc::new(database),
        }
    }
}
