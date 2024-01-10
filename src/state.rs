use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    board::{Board, Piece},
    PlaceError,
};

#[derive(Serialize, Deserialize)]
pub struct State {
    board: Board,
    turn: Piece,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Piece::Black,
        }
    }

    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> Result<(), PlaceError> {
        self.validate(x, y, piece)?;
        self.board[(x, y)] = Some(piece);
        self.board.flip(x, y, piece);
        self.turn = match self.turn {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        };
        Ok(())
    }

    fn validate(&self, x: usize, y: usize, piece: Piece) -> Result<(), PlaceError> {
        if x >= Board::width() || y >= Board::width() {
            Err(PlaceError::OutOfBounds(x, y))
        } else {
            match (
                self.turn == piece,
                !self.board.adjacent(x, y)?,
                self.board[(x, y)].is_some(),
            ) {
                (false, _, _) => Err(PlaceError::Turn(piece)),
                (_, true, _) => Err(PlaceError::NotAdjacent(x, y)),
                (_, _, true) => Err(PlaceError::Occupied(x, y)),
                _ if self.board.flips(x, y, piece) == 0 => Err(PlaceError::NoFlips(x, y)),
                _ => Ok(()),
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Turn: {:?}", self.turn)?;
        writeln!(f, "Board:")?;
        writeln!(f, "{:?}", self.board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let state = State::new();
        assert_eq!(state.turn, Piece::Black);
    }

    #[test]
    fn valid_placement() {
        let mut state = State::new();
        assert_eq!(state.turn, Piece::Black);
        println!("{:?}", state.board);
        let outcome = state.place(2, 3, Piece::Black);
        assert!(outcome.is_ok());
        assert_eq!(state.turn, Piece::White);
        println!("{:?}", state.board);
    }

    #[test]
    fn out_of_turn() {
        let mut state = State::new();
        assert_eq!(state.turn, Piece::Black);
        let outcome = state.place(2, 3, Piece::White);
        assert_eq!(outcome.unwrap_err(), PlaceError::Turn(Piece::White));
    }

    #[test]
    fn occupied() {
        let mut state = State::new();
        assert_eq!(state.turn, Piece::Black);
        let outcome = state.place(3, 3, Piece::Black);
        assert_eq!(outcome.unwrap_err(), PlaceError::Occupied(3, 3));
    }

    #[test]
    fn not_adjacent() {
        let mut state = State::new();
        assert_eq!(state.turn, Piece::Black);
        let outcome = state.place(0, 0, Piece::Black);
        assert_eq!(outcome.unwrap_err(), PlaceError::NotAdjacent(0, 0));
    }

    #[test]
    fn out_of_bounds() {
        let mut state = State::new();
        assert_eq!(state.turn, Piece::Black);
        let outcome = state.place(8, 8, Piece::Black);
        assert_eq!(outcome.unwrap_err(), PlaceError::OutOfBounds(8, 8));
    }
}
