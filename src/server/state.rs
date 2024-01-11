use crate::Game;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct AppState {
    pub(super) games: Arc<Mutex<HashMap<Uuid, Game>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
