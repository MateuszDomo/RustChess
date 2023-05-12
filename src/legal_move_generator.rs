use crate::chess_utility::GameState; 

pub fn legal_move_generator(game_state: &GameState, square_number: u32) -> Vec<u32>{
    let piece = game_state.board.squares[square_number as usize];
    match piece & 0b00000111{
        1 => {
            return pawn_move_generation(square_number, game_state);
        }
        2 => {
            return bishop_move_generation(square_number, game_state);
        }
        3 => {
            return knight_move_generation(square_number, game_state);
        }
        5 => {
            return rook_move_generation(square_number, game_state);
        }
        6 => {
            return queen_move_generation(square_number, game_state);
        }
        7 => {
            return king_move_generation(square_number, game_state);
        }
        _ => {
            panic!("Piece should have a valid piece type");
        }
    }
}

// TODO en passant
fn pawn_move_generation(square: u32, game_state: &GameState) -> Vec<u32>{
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let selected_piece_color: u8 = board.squares[square as usize] & 0b00011000;
    let starting_rank: u32 = (square / 8) + 1;
    let starting_file: u32 = (square % 8) + 1;

    let direction: i32 = if selected_piece_color == 0b00001000 { 1 } else { -1 };

    let single_square_advance_rank = starting_rank as i32 + direction;
    let single_square_advance_square = (single_square_advance_rank as u32 - 1) * 8 + (starting_file - 1);
    
    if single_square_advance_rank.in_range(1, 8)  && board.squares[single_square_advance_square as usize] == 0{
        legal_moves.push_square_from_rank_and_file(single_square_advance_rank as u32, starting_file);
    }
    
    let left_square_capture_file = starting_file - 1;
    let left_square_capture_square = single_square_advance_square - 1;
    let right_square_capture_file = starting_file + 1;
    let right_square_capture_square = single_square_advance_square + 1;
  
    if  single_square_advance_rank.in_range(1,8) {
        if (left_square_capture_file).in_range(1,8) && board.squares[left_square_capture_square as usize] != 0 && selected_piece_color != (board.squares[left_square_capture_square as usize] & 0b00011000) {
            legal_moves.push_square_from_rank_and_file(single_square_advance_rank as u32, left_square_capture_file);
        }
        if (right_square_capture_file).in_range(1,8) && board.squares[right_square_capture_square as usize] != 0 && selected_piece_color != (board.squares[right_square_capture_square as usize] & 0b00011000) {
            legal_moves.push_square_from_rank_and_file(single_square_advance_rank as u32, right_square_capture_file);
        }
    }
    
    if !is_pawn_starting_position(starting_rank, selected_piece_color) {
        return legal_moves;
    }

    let double_square_advance_rank = single_square_advance_rank + direction;
    let double_square_advance_square = (double_square_advance_rank as u32 - 1) * 8 + (starting_file - 1);

    if double_square_advance_rank.in_range(1, 8)  && board.squares[double_square_advance_square as usize] == 0{
        legal_moves.push_square_from_rank_and_file(double_square_advance_rank as u32, starting_file);
    }
    
    println!("{:?}",legal_moves);
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

fn knight_move_generation(square: u32, game_state: &GameState) -> Vec<u32> {
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (square / 8) as i32 + 1;
    let starting_file: i32 = (square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[square as usize] & 0b00011000;

    let directions: [(i32,i32); 4] = [(2,-1), (2,1), (-2,1), (-2,-1)];
    for (rank_dir,file_dir) in directions {

        if (starting_rank + rank_dir).in_range(1, 8) && (starting_file + file_dir).in_range(1, 8) {
            let index: i32 = ((starting_rank + rank_dir - 1) * 8) + (starting_file + file_dir - 1);
            let piece_color = board.squares[index as usize] & 0b00011000;
            if piece_color == selected_piece_color {
                continue;
            }
            legal_moves.push_square_from_rank_and_file((starting_rank + rank_dir) as u32, (starting_file + file_dir) as u32);
        }
    }


    return legal_moves;
}

fn rook_move_generation(square: u32, game_state: &GameState) -> Vec<u32> {
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (square / 8) as i32 + 1;
    let starting_file: i32 = (square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[square as usize] & 0b00011000;

let directions: [(i32,i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
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

fn queen_move_generation(square: u32, game_state: &GameState) -> Vec<u32> {
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (square / 8) as i32 + 1;
    let starting_file: i32 = (square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[square as usize] & 0b00011000;

    let directions: [(i32,i32); 8] = [(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)];
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

fn king_move_generation(square: u32, game_state: &GameState) -> Vec<u32> {
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (square / 8) as i32 + 1;
    let starting_file: i32 = (square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[square as usize] & 0b00011000;

    let directions: [(i32,i32); 8] = [(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)];
    for (rank_dir,file_dir) in directions {

        if (starting_rank + rank_dir).in_range(1, 8) && (starting_file + file_dir).in_range(1, 8) {
            let index: i32 = ((starting_rank + rank_dir - 1) * 8) + (starting_file + file_dir - 1);
            let piece_color = board.squares[index as usize] & 0b00011000;
            if piece_color == selected_piece_color {
                continue;
            }
            legal_moves.push_square_from_rank_and_file((starting_rank + rank_dir) as u32, (starting_file + file_dir) as u32);
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

trait InRangeU32 {
    fn in_range(self, a: Self, b: Self) -> bool;
}

impl InRangeU32 for u32 {
    fn in_range(self, a: Self, b: Self) -> bool {
        return self >= a && self <= b;
    }
}

trait PushSquareFromRankAndFile {
    fn push_square_from_rank_and_file(&mut self, rank: u32, file: u32);
}

impl PushSquareFromRankAndFile for Vec<u32> {
    fn push_square_from_rank_and_file(&mut self, rank: u32, file: u32) {
        let square: u32 = (rank - 1) * 8 + (file - 1);
        self.push(square);
    }
}