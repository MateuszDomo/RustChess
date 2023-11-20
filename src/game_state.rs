use bevy::prelude::*;

use crate::{board::Board, chess_utility::{SideColor, MoveSoundEvent, MoveSounds}, attack_data::AttackData, piece_move::{PieceMove, Flag}, legal_move_generator::legal_move_generator, fen::extract_game_state_from_fen};

#[derive(Resource)]
pub struct GameState {
    pub board: Board,
    pub selected_square: Option<u32>,
    pub next_color_to_move: SideColor,
    pub castling_rights: CastlingRights,
    pub enpassant_target: Option<u32>,
}

impl GameState {

    pub fn new(fen_string: &String) -> Self {
        let (pieces, next_color_to_moves, castling_rights, enpassant_target) = extract_game_state_from_fen(fen_string);
        return GameState { board: Board {squares: pieces}, selected_square: None, next_color_to_move: next_color_to_moves, castling_rights: castling_rights, enpassant_target: enpassant_target};
    }

    pub fn flip_turn(&mut self, sound_event: EventWriter<MoveSoundEvent>, turn_ending_move: PieceMove) {
        self.update_castling_rights(turn_ending_move.target_square());
        if self.next_color_to_move == SideColor::White {
            self.next_color_to_move = SideColor::Black;
        }else{
            self.next_color_to_move = SideColor::White;
        }
        self.scan_game_state(sound_event, turn_ending_move);
    }

    fn scan_game_state(&self, sound_event: EventWriter<MoveSoundEvent>, turn_ending_move: PieceMove) {
        let mut attack_data :AttackData = AttackData::new(&self.board, self.next_color_to_move.side_color_to_u8());
        attack_data.calculate_attack_data(&self.board, self.next_color_to_move.side_color_to_u8());

        self.scan_checks_and_mates(sound_event, &attack_data, turn_ending_move);
    }

    fn scan_checks_and_mates(&self, mut sound_event: EventWriter<MoveSoundEvent>, attack_data: &AttackData, turn_ending_move: PieceMove) {
        
        if self.is_possible_move() {
            if attack_data.in_check {
                sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Check});
            } else {
                match turn_ending_move.flag() {
                    Flag::Castle => sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Castle}),
                    Flag:: Capture | Flag::EnpassantCapture => sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Capture}),
                    Flag:: Promote => sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Promote}),
                    Flag::None | Flag::EnpassantTarget => sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Move}),
                }
            }
            return;
        }

        // TODO STALEMATE WHEN ONLY 2 KINGS 
        if attack_data.in_check {
            if self.next_color_to_move == SideColor::White {
                println!("Checkmate! Black Wins!")
            } else {
                println!("Checkmate! White Wins!")
            }
            sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Checkmate});
        } else {
            // Stalemate
            println!("stalemate");
        }

    }
    fn is_possible_move(&self) -> bool {
            for square_number in 0..64 {
                if self.board.contains_piece(square_number) && self.board.piece_color_to_side_color(square_number) != self.next_color_to_move {
                    continue;
                }
                let legal_moves: Vec<PieceMove> = legal_move_generator(self, square_number as u32);
                if !legal_moves.is_empty() {
                    return true;
                }
            }
            return false;
    }
    
    fn update_castling_rights(&mut self, to_square: u32) {
        if self.board.contains_king(to_square) {
            self.castling_rights.revoke_all(&self.next_color_to_move)
        } else if self.board.contains_rook(to_square) {
            if (to_square % 8) + 1 == 1 {
                self.castling_rights.revoke_long(&self.next_color_to_move);
            } else if (to_square % 8) + 1 == 8 {
                self.castling_rights.revoke_short(&self.next_color_to_move);
            }
        }
    }
}


pub struct CastlingRights {
    pub w_long: bool,
    pub w_short: bool,
    pub b_long: bool,
    pub b_short: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        return CastlingRights {w_long: false, w_short: false, b_long: false, b_short: false};
    }

    pub fn has_rights(&self, side_color: &SideColor) -> bool {
        if *side_color == SideColor::White {
            return self.w_long || self.w_short
        }
        return self.b_long || self.b_short;
    }

    pub fn revoke_all(&mut self, side_color: &SideColor) {
        if *side_color == SideColor::White {
            self.w_long = false;
            self.w_short = false;
            return;
        }
        self.b_long = false;
        self.b_short = false;
    }

    pub fn revoke_long(&mut self, side_color: &SideColor) {
        if *side_color == SideColor::White {
            self.w_long = false;
        }
        self.b_long = false;
    }

    pub fn revoke_short(&mut self, side_color: &SideColor) {
        if *side_color == SideColor::White {
            self.w_short = false;
        }
        self.b_short = false;
    }
}