pub use board::Piece;
pub use game::Game;
use serde::{Deserialize, Serialize};

mod board;
mod game;
pub mod server;

#[derive(thiserror::Error, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceError {
    #[error("board square ({0}, {1}) is occupied")]
    Occupied(usize, usize),
    #[error("it is not {0:?}'s turn")]
    Turn(Piece),
    #[error("board square ({0}, {1}) is not adjacent to any other piece")]
    NotAdjacent(usize, usize),
    #[error("board square ({0}, {1}) is out of bounds")]
    OutOfBounds(usize, usize),
    #[error("no pieces were flipped from board square ({0}, {1})")]
    NoFlips(usize, usize),
}
