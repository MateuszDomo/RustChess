use crate::{chess_utility::{GameState, squareFromRankFile}, attack_bitmap::{AttackBitmap}, attack_data::AttackData}; 


pub fn legal_move_generator(game_state: &GameState, square_number: u32) -> Vec<u32>{
    let piece: u8 = game_state.board.squares[square_number as usize];
    let mut attack_data = AttackData::new(&game_state.board,game_state.next_side_color_to_move.side_color_to_u8());
    attack_data.calculate_attack_data(&game_state.board, game_state.next_side_color_to_move.side_color_to_u8());

    match piece & 0b00000111{
        1 => {
            return pawn_move_generation(square_number, game_state, &attack_data);
        }
        2 | 5 | 6 => {
            return sliding_pieces_move_generation(square_number, game_state, &attack_data, piece);
        }
        3 => {
            return knight_move_generation(square_number, game_state, &attack_data);
        }
        7 => {
            return king_move_generation(square_number, game_state, attack_data.attack_bitmaps);
        }
        _ => {
            panic!("Piece should have a valid piece type");
        }
    }
}

// TODO en passant
fn pawn_move_generation(selected_square: u32, game_state: &GameState, attack_data: &AttackData) -> Vec<u32>{
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let selected_piece_color: u8 = board.squares[selected_square as usize] & 0b00011000;
    let starting_rank: u32 = (selected_square / 8) + 1;
    let starting_file: u32 = (selected_square % 8) + 1;

    let direction: i32 = if selected_piece_color == 0b00001000 { 1 } else { -1 };

    // Advance one square
    let single_square_advance_rank = starting_rank as i32 + direction;
    let single_square_advance_square = squareFromRankFile(single_square_advance_rank as u32, starting_file);
    if can_move_with_check_and_pin(attack_data, selected_square, single_square_advance_square) && single_square_advance_rank.in_range(1, 8)  && board.squares[single_square_advance_square as usize] == 0{
        legal_moves.push_square_from_rank_and_file(single_square_advance_rank as u32, starting_file);
    }
    
    // Attack left and right diagonal squares
    for file_dir in [1,-1] {
        let square_capture_file: u32 = (starting_file as i32 + file_dir) as u32;
        let square_capture_square: u32 = (single_square_advance_square as i32 + file_dir) as u32;
        if can_move_with_check_and_pin(attack_data, selected_square, square_capture_square) && single_square_advance_rank.in_range(1,8) {
            if (square_capture_file).in_range(1,8) && board.squares[square_capture_square as usize] != 0 && selected_piece_color != (board.squares[square_capture_square as usize] & 0b00011000) {
                legal_moves.push_square_from_rank_and_file(single_square_advance_rank as u32, square_capture_file);
            }
            if (square_capture_file).in_range(1,8) && board.squares[square_capture_square as usize] != 0 && selected_piece_color != (board.squares[square_capture_square as usize] & 0b00011000) {
                legal_moves.push_square_from_rank_and_file(single_square_advance_rank as u32, square_capture_file);
            }
        }
    }

    if !is_pawn_starting_position(starting_rank, selected_piece_color) || board.squares[single_square_advance_square as usize] != 0 {
        return legal_moves;
    }

    // Advance two squares
    let double_square_advance_rank = single_square_advance_rank + direction;
    let double_square_advance_square = (double_square_advance_rank as u32 - 1) * 8 + (starting_file - 1);
    if can_move_with_check_and_pin(attack_data, selected_square, double_square_advance_square) && double_square_advance_rank.in_range(1, 8)  && board.squares[double_square_advance_square as usize] == 0{
        legal_moves.push_square_from_rank_and_file(double_square_advance_rank as u32, starting_file);
    }
    
    return legal_moves;
}



fn is_pawn_starting_position(rank: u32, selected_piece_color: u8) -> bool {
    match (rank,selected_piece_color){
        (2,0b00001000) | (7,0b00010000) => return true,
        _ => return false,
    }
}

fn sliding_pieces_move_generation(selected_square: u32, game_state: &GameState, attack_data: &AttackData, piece: u8) -> Vec<u32> {
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (selected_square / 8) as i32 + 1;
    let starting_file: i32 = (selected_square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[selected_square as usize] & 0b00011000;

    let piece_type = piece & 0b00000111;
    let attacking_directions: Vec<(i32,i32)> = if piece_type == 2 {
        vec![(1, -1), (1, 1), (-1, -1), (-1, 1)]
    } else if piece_type == 5 {
        vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
    } else if piece_type == 6 {
        vec![(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)]
    } else {
        vec![]
    };

    for (rank_dir, file_dir) in attacking_directions.iter() {
        let mut i: i32  = 1; 
        while (starting_rank + i*rank_dir).in_range(1, 8) && (starting_file + i * file_dir).in_range(1, 8) {
            let target_square: i32 = ((starting_rank + i * rank_dir - 1) * 8) + (starting_file + i * file_dir - 1);
            
            let piece_color = board.squares[target_square as usize] & 0b00011000;
            if piece_color == selected_piece_color {
                break;
            }

            if !can_move_with_check_and_pin(attack_data, selected_square, target_square as u32) {
                i += 1;
                continue;
            }

            legal_moves.push_square_from_rank_and_file((starting_rank + i*rank_dir) as u32, (starting_file + i*file_dir) as u32);
            // Not continue when piece found because legal sliding moves cannot phase through pieces
            let piece_type = board.squares[target_square as usize] & 0b00011000;
            if piece_type != 0 {
                break;
            }
            i += 1;
        }
    }
    return legal_moves;

}

fn knight_move_generation(selected_square: u32, game_state: &GameState, attack_data: &AttackData) -> Vec<u32> {
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (selected_square / 8) as i32 + 1;
    let starting_file: i32 = (selected_square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[selected_square as usize] & 0b00011000;

    let directions: [(i32,i32); 8] = [(2,-1), (2,1), (-2,1), (-2,-1), (1,2), (1,-2), (-1,2), (-1,-2)];
    for (rank_dir,file_dir) in directions {

        if (starting_rank + rank_dir).in_range(1, 8) && (starting_file + file_dir).in_range(1, 8) {
            let target_square: i32 = ((starting_rank + rank_dir - 1) * 8) + (starting_file + file_dir - 1);
            
            let piece_color = board.squares[target_square as usize] & 0b00011000;
            if piece_color == selected_piece_color  {
                continue;
            }

            if !can_move_with_check_and_pin(attack_data, selected_square, target_square as u32) {
                continue;
            }

            legal_moves.push_square_from_rank_and_file((starting_rank + rank_dir) as u32, (starting_file + file_dir) as u32);
        }
    }

    return legal_moves;
}

fn king_move_generation(selected_square: u32, game_state: &GameState, attack_bitmap: AttackBitmap) -> Vec<u32> {
    let mut legal_moves: Vec<u32> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (selected_square / 8) as i32 + 1;
    let starting_file: i32 = (selected_square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[selected_square as usize] & 0b00011000;

    let directions: [(i32,i32); 8] = [(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)];
    for (rank_dir,file_dir) in directions {

        if (starting_rank + rank_dir).in_range(1, 8) && (starting_file + file_dir).in_range(1, 8) {
            let square_number: i32 = ((starting_rank + rank_dir - 1) * 8) + (starting_file + file_dir - 1);
            let piece_color = board.squares[square_number as usize] & 0b00011000;
        if piece_color == selected_piece_color || attack_bitmap.is_square_being_attacked(square_number as u32) {
                continue;
        }
            legal_moves.push_square_from_rank_and_file((starting_rank + rank_dir) as u32, (starting_file + file_dir) as u32);
        }
    }
    return legal_moves;
}

fn can_move_with_check_and_pin(attack_data: &AttackData, selected_square: u32, target_square: u32) -> bool {
    let cannot_block_check: bool =  attack_data.in_check && (!attack_data.is_square_in_check_ray(target_square) || attack_data.is_square_in_pinned_ray(target_square));
    let cannot_move_pinned: bool =  attack_data.is_square_in_pinned_ray(selected_square as u32) && !attack_data.is_square_in_pinned_ray(target_square);
    return !(cannot_block_check || cannot_move_pinned);
}

pub trait InRangeI32{
    fn in_range(self, a: Self, b: Self) -> bool;
}

impl InRangeI32 for i32 {
    fn in_range(self, a: Self, b: Self) -> bool {
        return self >= a && self <= b;
    }
}

pub trait InRangeU32 {
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