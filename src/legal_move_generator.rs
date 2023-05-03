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
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let mut i = 1;
    let starting_rank = (square / 8) as i32 + 1;
    let starting_file = (square % 8) as i32 + 1;
    let selected_piece_color = board.squares[square as usize] & 0b00011000;

    println!("{}",selected_piece_color);
    while ((starting_rank + i) <= 8) && ((starting_file - i) >= 1) {
        //println!("{}",0b00000111 & game_state.board.squares[(((starting_rank + i - 1) * 8) + (starting_file - i - 1)) as usize]);
        let index = ((starting_rank + i - 1) * 8) + (starting_file - i - 1);

        // Not include capturing friendly pieces in legal moves
        if (board.squares[index as usize] & 0b00011000) == selected_piece_color {
            break;
        }
        println!("{}",index);
        legal_moves.push(index as u32);
        if ((board.squares[index as usize] & 0b00000111)) != 0 {
            break;
        }
        i += 1;
    }
    i = 1;
    while ((starting_rank + i) <= 8) && (starting_file + i) <= 8 {
        //println!("{}",0b00000111 & game_state.board.squares[(((starting_rank + i - 1) * 8) + (starting_file + i - 1)) as usize]);
        let index = ((starting_rank + i - 1) * 8) + (starting_file + i - 1);

        // Not include capturing friendly pieces in legal moves
        if (board.squares[index as usize] & 0b00011000) == selected_piece_color {
            break;
        }
        legal_moves.push(index as u32);
        if ((board.squares[index as usize] & 0b00000111)) != 0 {
            break;
        }
        i += 1;
    }
    i = 1;
    while ((starting_rank - i) >= 1) && (starting_file - i) >= 1 {
        //println!("{}",0b00000111 & game_state.board.squares[(((starting_rank - i - 1) * 8) + (starting_file - i - 1)) as usize]);
        let index = ((starting_rank - i - 1) * 8) + (starting_file - i - 1);

        // Not include capturing friendly pieces in legal moves
        if (board.squares[index as usize] & 0b00011000) == selected_piece_color {
            break;
        }
        legal_moves.push(index as u32);
        if ((board.squares[index as usize] & 0b00000111)) != 0 {
            break;
        }
        i += 1;
    }
    i = 1;
    while ((starting_rank - i) >= 1) && (starting_file + i) <= 8 {
        //println!("{}",0b00000111 & game_state.board.squares[(((starting_rank + i - 1) * 8) + (starting_file + i - 1)) as usize]);
        let index = ((starting_rank - i - 1) * 8) + (starting_file + i - 1);

        // Not include capturing friendly pieces in legal moves
        if (board.squares[index as usize] & 0b00011000) == selected_piece_color {
            break;
        }
        legal_moves.push(index as u32);
        if ((board.squares[index as usize] & 0b00000111)) != 0 {
            break;
        }
        i += 1;
    }
    println!("{:?}",legal_moves);
}