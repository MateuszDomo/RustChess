use crate::{chess_utility::GameState, legal_move_generator::legal_move_generator};

#[derive(Clone)]
pub struct SelectedSquare {
    pub square_number: u32,
    pub legal_moves: Vec<u32>,
}

impl SelectedSquare {
    pub fn new(square_number: u32, game_state: &GameState) -> Self {
        let legal_moves = legal_move_generator(game_state,square_number);
        return SelectedSquare { square_number: square_number, legal_moves: legal_moves}
    }
}

