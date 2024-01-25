use crate::{Game, Piece};

pub struct Companion<'a> {
    game: &'a Game,
    color: isize,
}

impl<'a> From<&'a Game> for Companion<'a> {
    fn from(game: &'a Game) -> Self {
        Self {
            game,
            color: if game.turn() == Piece::Black { 1 } else { -1 },
        }
    }
}

impl Companion<'_> {
    pub fn choice(&mut self, depth: usize) -> (usize, usize) {
        let mut history = vec![];
        let mut root = self.game.clone();
        Self::negamax(&mut root, &mut history, depth, self.color);
        let index = self.game.history().len();
        history.get(index).copied().unwrap()
    }

    fn negamax(
        game: &mut Game,
        history: &mut Vec<(usize, usize)>,
        depth: usize,
        color: isize,
    ) -> isize {
        if depth == 0 || game.over() {
            color * Self::heuristic(game)
        } else {
            game.moves(Self::player(color))
                .iter()
                .fold(isize::MIN, |value, &(x, y)| {
                    let mut child = game.clone();
                    child.place(x, y, Self::player(color)).unwrap();
                    let alt = -Self::negamax(&mut child, history, depth - 1, -color);
                    if alt > value {
                        *history = child.history();
                    }
                    value.max(alt)
                })
        }
    }

    fn player(color: isize) -> Piece {
        if color == 1 {
            Piece::Black
        } else {
            Piece::White
        }
    }

    #[allow(clippy::cast_possible_wrap)] // 64 <= isize::MAX
    fn heuristic(game: &mut Game) -> isize {
        let (black, _) = game.score();
        assert!(black <= 64);
        black as isize
    }
}

#[cfg(test)]
mod tests {
    use crate::{Game, Piece};

    use super::Companion;

    #[test]
    fn draw() {
        let mut game = Game::new();
        let mut piece = Piece::Black;
        // println!("{game}");
        while !game.over() {
            let mut companion = Companion::from(&game);
            let (x, y) = companion.choice(6);
            game.place(x, y, piece).unwrap();
            // println!("{game}");
            piece = !piece;
        }
        assert_eq!(game.score(), (32, 32));
    }
}
