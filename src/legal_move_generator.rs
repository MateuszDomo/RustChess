
use crate::{chess_utility::{square_from_rank_file, SideColor}, attack_data::AttackData, piece_move::{PieceMove, Flag}, game_state::GameState}; 


pub fn legal_move_generator(game_state: &GameState, square_number: u32) -> Vec<PieceMove>{
    let piece: u8 = game_state.board.squares[square_number as usize];
    let mut attack_data = AttackData::new(&game_state.board,game_state.next_color_to_move.side_color_to_u8());
    attack_data.calculate_attack_data(&game_state.board, game_state.next_color_to_move.side_color_to_u8());

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
            return king_move_generation(square_number, game_state, &attack_data);
        }
        _ => Vec::new()
    }
}

// TODO en passant
fn pawn_move_generation(selected_square: u32, game_state: &GameState, attack_data: &AttackData) -> Vec<PieceMove>{
    let mut legal_moves: Vec<PieceMove> = Vec::new();
    let board = &game_state.board;
    let selected_piece_color: u8 = board.squares[selected_square as usize] & 0b00011000;
    let starting_rank: u32 = (selected_square / 8) + 1;
    let starting_file: u32 = (selected_square % 8) + 1;

    let direction: i32 = if selected_piece_color == 0b00001000 { 1 } else { -1 };

    // Advance one square
    let single_square_advance_rank = starting_rank as i32 + direction;
    let single_square_advance_square = square_from_rank_file(single_square_advance_rank as u32, starting_file);
    if can_move_with_check_and_pin(attack_data, selected_square, single_square_advance_square) && single_square_advance_rank.in_range(1, 8)  && board.squares[single_square_advance_square as usize] == 0{
        legal_moves.push_square_from_rank_and_file(starting_rank, starting_file, single_square_advance_rank as u32, starting_file, Flag::None.into());
    }
    
    // Attack left and right diagonal squares
    for file_dir in [1,-1] {
        let square_capture_file: u32 = (starting_file as i32 + file_dir) as u32;
        let square_capture_square: u32 = (single_square_advance_square as i32 + file_dir) as u32;
        if can_move_with_check_and_pin(attack_data, selected_square, square_capture_square) && single_square_advance_rank.in_range(1,8) {
            if (square_capture_file).in_range(1,8) && board.squares[square_capture_square as usize] != 0 && selected_piece_color != (board.squares[square_capture_square as usize] & 0b00011000) {
                legal_moves.push_square_from_rank_and_file(starting_rank, starting_file, single_square_advance_rank as u32, square_capture_file, Flag::None.into());
            }
            if (square_capture_file).in_range(1,8) && board.squares[square_capture_square as usize] != 0 && selected_piece_color != (board.squares[square_capture_square as usize] & 0b00011000) {
                legal_moves.push_square_from_rank_and_file(starting_rank, starting_file, single_square_advance_rank as u32, square_capture_file, Flag::None.into());
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
        legal_moves.push_square_from_rank_and_file(starting_rank, starting_file, double_square_advance_rank as u32, starting_file, Flag::None.into());
    }
    
    return legal_moves;
}



fn is_pawn_starting_position(rank: u32, selected_piece_color: u8) -> bool {
    match (rank,selected_piece_color){
        (2,0b00001000) | (7,0b00010000) => return true,
        _ => return false,
    }
}

fn sliding_pieces_move_generation(selected_square: u32, game_state: &GameState, attack_data: &AttackData, piece: u8) -> Vec<PieceMove> {
    let mut legal_moves: Vec<PieceMove> = Vec::new();
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

            let target_rank: u32 = (starting_rank + rank_dir * i) as u32;
            let target_file: u32 = (starting_file + file_dir * i) as u32;
            legal_moves.push_square_from_rank_and_file(starting_rank as u32, starting_file as u32, target_rank, target_file, Flag::None.into());
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

fn knight_move_generation(selected_square: u32, game_state: &GameState, attack_data: &AttackData) -> Vec<PieceMove> {
    let mut legal_moves: Vec<PieceMove> = Vec::new();
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

            let target_rank: u32 = (starting_rank + rank_dir) as u32;
            let target_file: u32 = (starting_file + file_dir) as u32;
            legal_moves.push_square_from_rank_and_file(starting_rank as u32, starting_file as u32, target_rank, target_file, Flag::None.into());
        }
    }

    return legal_moves;
}

fn king_move_generation(selected_square: u32, game_state: &GameState, attack_data: &AttackData) -> Vec<PieceMove> {
    
    let mut legal_moves: Vec<PieceMove> = Vec::new();
    let board = &game_state.board;
    let starting_rank: i32 = (selected_square / 8) as i32 + 1;
    let starting_file: i32 = (selected_square % 8) as i32 + 1;
    let selected_piece_color: u8 = board.squares[selected_square as usize] & 0b00011000;
    attack_data.attack_bitmaps.print_bitmap();
    let directions: [(i32,i32); 8] = [(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)];
    for (rank_dir,file_dir) in directions {

        if (starting_rank + rank_dir).in_range(1, 8) && (starting_file + file_dir).in_range(1, 8) {
            let target_square: i32 = ((starting_rank + rank_dir - 1) * 8) + (starting_file + file_dir - 1);
            let piece_color = board.squares[target_square as usize] & 0b00011000;
            if piece_color == selected_piece_color || attack_data.attack_bitmaps.is_square_being_attacked(target_square as u32) || attack_data.is_square_in_check_ray(target_square as u32) {
                continue;
            }
            let target_rank: u32 = (starting_rank + rank_dir) as u32;
            let target_file: u32 = (starting_file + file_dir) as u32;
            legal_moves.push_square_from_rank_and_file(starting_rank as u32, starting_file as u32, target_rank, target_file, Flag::None.into());
        }
    }

    // Castling logic
    let (long_rights, short_rights) = get_castle_rights(selected_piece_color, game_state);
    if long_rights {
        let mut can_long_castle: bool = true;
        for file_dir in 1..=3 {
            let square_number = ((starting_rank - 1) * 8) as u32 + (starting_file as u32 - 1 - file_dir);
            let piece = board.squares[square_number as usize] & 0b00000111;
            if piece != 0 || attack_data.attack_bitmaps.is_square_being_attacked(square_number) {
                can_long_castle = false;
                break;
            }
        }
        if can_long_castle {
            legal_moves.push_square_from_rank_and_file(starting_rank as u32, starting_file as u32, starting_rank as u32, (starting_file as u32) - 2, Flag::Castle.into());
        }
    }
    if short_rights {
        let mut can_short_castle: bool = true;
        for file_dir in 1..=2 {
            let square_number = ((starting_rank - 1) * 8) as u32 + (starting_file as u32 - 1 + file_dir);
            let piece = board.squares[square_number as usize] & 0b00000111;
            if piece != 0 || attack_data.attack_bitmaps.is_square_being_attacked(square_number) {
                can_short_castle = false;
                break;
            }
        }
        if can_short_castle {
            legal_moves.push_square_from_rank_and_file(starting_rank as u32, starting_file as u32, starting_rank as u32, (starting_file as u32) + 2, Flag::Castle.into());
        }
    }
    return legal_moves;
}

fn get_castle_rights(selected_piece_color: u8, game_state: &GameState) -> (bool, bool){
    if selected_piece_color == SideColor::White.side_color_to_u8() {
        return (game_state.castling_rights.w_long, game_state.castling_rights.w_short);
    }
    return (game_state.castling_rights.b_long, game_state.castling_rights.b_short);
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
    fn push_square_from_rank_and_file(&mut self, s_rank: u32, s_file: u32, t_rank: u32, t_file: u32, flag: u16);
}

impl PushSquareFromRankAndFile for Vec<PieceMove> {
    fn push_square_from_rank_and_file(&mut self, s_rank: u32, s_file: u32, t_rank: u32, t_file: u32, flag: u16) {
        let s_square: u32 = (s_rank - 1) * 8 + (s_file - 1);
        let t_square: u32 = (t_rank - 1) * 8 + (t_file - 1);
        self.push(PieceMove::new(s_square, t_square, flag));
    }
}