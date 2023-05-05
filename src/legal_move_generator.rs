use crate::chess_utility::GameState; 

pub fn legal_move_generator(game_state: &GameState, square_number: u32) -> Vec<u32>{
    let piece = game_state.board.squares[square_number as usize];
    let filler: Vec<u32> = Vec::new();
    match piece & 0b00000111{
        1 => {
            return filler;
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
trait InRange{
    fn in_range(self, a: Self, b: Self) -> bool;
}

impl InRange for i32 {
    fn in_range(self, a: Self, b: Self) -> bool {
        return self >= a && self <= b;
    }
}