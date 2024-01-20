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
        self.negamax(&mut root, &mut history, depth, self.color);
        let index = self.game.history().len();
        history.get(index).cloned().unwrap()
    }

    fn negamax(
        &self,
        game: &mut Game,
        history: &mut Vec<(usize, usize)>,
        depth: usize,
        color: isize,
    ) -> isize {
        if depth == 0 || game.over() {
            color * self.heuristic(game)
        } else {
            game.moves(self.player(color))
                .iter()
                .fold(isize::MIN, |value, &(x, y)| {
                    let mut child = game.clone();
                    child.place(x, y, self.player(color)).unwrap();
                    let alt = -self.negamax(&mut child, history, depth - 1, -color);
                    if alt > value {
                        *history = child.history();
                    }
                    value.max(alt)
                })
        }
    }

    fn player(&self, color: isize) -> Piece {
        if color == 1 {
            Piece::Black
        } else {
            Piece::White
        }
    }

    fn heuristic(&self, game: &mut Game) -> isize {
        let (black, _) = game.score();
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
