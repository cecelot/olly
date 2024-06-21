use crate::{server::packet::Event, Game};
use sea_orm::DatabaseConnection;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)] // This seems fine
pub struct AppState {
    pub(super) games: Arc<Mutex<HashMap<Uuid, Game>>>,
    pub(super) rooms: Arc<Mutex<HashMap<Uuid, broadcast::Sender<Event>>>>,
    pub(super) database: Arc<DatabaseConnection>,
    pub(super) redis: Arc<redis::Client>,
}

impl AppState {
    #[must_use]
    pub fn new(database: DatabaseConnection, redis: redis::Client) -> Self {
        Self {
            games: Arc::new(Mutex::new(HashMap::new())),
            rooms: Arc::new(Mutex::new(HashMap::new())),
            database: Arc::new(database),
            redis: Arc::new(redis),
        }
    }
}
