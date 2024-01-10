use othello::{Piece, State};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = State::new();
    println!("{:?}", state);
    place(&mut state, 2, 3, Piece::Black)?;
    place(&mut state, 4, 2, Piece::White)?;
    place(&mut state, 5, 3, Piece::Black)?;
    place(&mut state, 2, 2, Piece::White)?;
    place(&mut state, 3, 2, Piece::Black)?;
    place(&mut state, 2, 4, Piece::White)?;
    place(&mut state, 3, 5, Piece::Black)?;
    place(&mut state, 2, 5, Piece::White)?;
    Ok(())
}

fn place(
    state: &mut State,
    x: usize,
    y: usize,
    piece: Piece,
) -> Result<(), Box<dyn std::error::Error>> {
    state.place(x, y, piece)?;
    println!("{:?}", state);
    Ok(())
}
