use crate::chess_utility::GameState; 

pub fn legal_move_generator(game_state: &GameState, square_number: u32) -> Vec<u32>{
    let piece = game_state.board.squares[square_number as usize];
    let filler: Vec<u32> = Vec::new();
    match piece & 0b00000111{
        1 => {
            return pawn_move_generation(square_number, game_state);
        }
        2 => {
            return bishop_move_generation(square_number, game_state);
        }
        3 => {
            return filler;
        }
        5 => {
            return filler;
        }
        6 => {
            return filler;
        }
        7 => {
            return filler;
        }
        _ => {
            println!("Should not reach here");
            panic!();
        }
    }
}

// TODO en passant / diagonal capture
fn pawn_move_generation(square: u32, game_state: &GameState) -> Vec<u32>{
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let selected_piece_color: u8 = board.squares[square as usize] & 0b00011000;
    let starting_rank: u32 = (square / 8) + 1;
    let starting_file: u32 = (square % 8) + 1;

    // Single pawn advance
    let square_move: i32 = if selected_piece_color == 0b00001000 {
        square as i32 + 8
    } else {
        square as i32 -8
    };
    if (square_move).in_range(0, 63) && board.squares[(square_move) as usize] == 0 {
        legal_moves.push(square_move as u32);
    }

    // Diagonal pawn capture
    let capture_square_rank: i32 = if selected_piece_color == 0b00001000 {
        starting_rank as i32 + 1
    } else {
        starting_rank as i32 - 1
    };

    let left_capture_square: i32 = if selected_piece_color == 0b00001000 {
        square as i32 + 7
    } else {
        square as i32 -  7
    };

    let right_capture_square: i32 = if selected_piece_color == 0b00001000 {
        square as i32 + 9
    } else {
        square as i32 - 9
    };
 
    if capture_square_rank.in_range(1,8) {
        if (starting_file - 1).in_range(1,8) && board.squares[left_capture_square as usize] != 0 && selected_piece_color != (board.squares[left_capture_square as usize] & 0b00011000) {
            legal_moves.push(left_capture_square as u32);
        }
        if (starting_file + 1).in_range(1,8) && board.squares[right_capture_square as usize] != 0 && selected_piece_color != (board.squares[right_capture_square as usize] & 0b00011000) {
            legal_moves.push(right_capture_square as u32);
        }
    }
    
    let starting_rank = (square / 8) + 1;
    if !is_pawn_starting_position(starting_rank, selected_piece_color) {
        return legal_moves;
    }

    // Double square advance    
    let double_square_move: i32 = if selected_piece_color == 0b00001000 {
        square as i32 + 16
    } else {
        square as i32 -16
    };
    if (double_square_move).in_range(0, 63) && board.squares[(double_square_move) as usize] == 0 {
        legal_moves.push(double_square_move as u32);
    }
    

    return legal_moves;
}

fn is_pawn_starting_position(rank: u32, selected_piece_color: u8) -> bool {
    match (rank,selected_piece_color){
        (2,0b00001000) | (7,0b00010000) => return true,
        _ => return false,
    }
}

fn bishop_move_generation(square: u32, game_state: &GameState) -> Vec<u32>{
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (square / 8) as i32 + 1;
    let starting_file: i32 = (square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[square as usize] & 0b00011000;

    let directions: [(i32,i32); 4] = [(1, -1), (1, 1), (-1, -1), (-1, 1)];
    for (rank_dir, file_dir) in directions.iter() {
        let mut i: i32  = 1; 
        while (starting_rank + i*rank_dir).in_range(1, 8) && (starting_file + i * file_dir).in_range(1, 8) {
            let index: i32 = ((starting_rank + i * rank_dir - 1) * 8) + (starting_file + i * file_dir - 1);
            let piece_color = board.squares[index as usize] & 0b00011000;
            if piece_color == selected_piece_color {
                break;
            }
            legal_moves.push(index as u32);
            // Not continue when piece found because legal bishop moves cannot phase through pieces
            let piece_type = board.squares[index as usize] & 0b00011000;
            if piece_type != 0 {
                break;
            }
            i += 1;
        }
    }
    return legal_moves;
}

trait InRangeI32{
    fn in_range(self, a: Self, b: Self) -> bool;
}

impl InRangeI32 for i32 {
    fn in_range(self, a: Self, b: Self) -> bool {
        return self >= a && self <= b;
    }
}

trait InRangeU32{
    fn in_range(self, a: Self, b: Self) -> bool;
}

impl InRangeU32 for u32 {
    fn in_range(self, a: Self, b: Self) -> bool {
        return self >= a && self <= b;
    }
}