use bevy::prelude::*;

use crate::chess_utility::GameState; 

pub fn legal_move_generator(game_state: &GameState) {
    let square = game_state.selected_square.expect("selected square should exist");
    let piece = game_state.board.squares[square as usize];
    match piece & 0b00000111{
        1 => {

        }
        2 => {
            bishop_move_generation(square, game_state);
        }
        3 => {

        }
        5 => {

        }
        6 => {

        }
        7 => {

        }
        _ => {
            println!("Should not reach here");
        }
    }
}

fn bishop_move_generation(square: u32, game_state: &GameState) {
    let board = &game_state.board;
    let mut i = 1;
    let starting_rank = (square / 8) as i32 + 1;
    let starting_file = (square % 8) as i32 + 1;
    while ((starting_rank + i) <= 8) && ((starting_file - i) >= 1) {
        println!("{}",((starting_rank + i - 1) * 8) + (starting_file - i - 1));
        i += 1;
    }
    // while ((starting_rank + i) <= 8) && (starting_file + i) <= 8 {

    // }
    // while ((starting_rank - i) <= 0) && (starting_file - i) > 0 {

    // }
    // while ((starting_rank + i) <= 8) && (starting_file - i) > 0 {

    // }
}