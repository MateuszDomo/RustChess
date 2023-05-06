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

fn pawn_move_generation(square: u32, game_state: &GameState) -> Vec<u32>{
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let selected_piece_color: u8 = board.squares[square as usize] & 0b00011000;
    
    let square_move: i32 = if selected_piece_color == 0b00001000 {
        square as i32 + 8
    } else {
        square as i32 -8
    };

    if (square_move).in_range(0, 63) && board.squares[(square_move) as usize] == 0 {
        legal_moves.push(square_move as u32);
    }
    println!("{:?}", legal_moves);
    return legal_moves;
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
    println!("{:?}", legal_moves);
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