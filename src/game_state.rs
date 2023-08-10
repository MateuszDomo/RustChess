use bevy::prelude::*;

use crate::{board::Board, chess_utility::{SideColor, MoveSoundEvent, MoveSounds}, attack_data::AttackData, piece_move::PieceMove, legal_move_generator::legal_move_generator, fen::extract_game_state_from_fen};

#[derive(Resource)]
pub struct GameState {
    pub board: Board,
    pub selected_square: Option<u32>,
    pub next_color_to_move: SideColor,
    pub castling_rights: CastlingRights,
}

impl GameState {

    pub fn new(fen_string: &String) -> Self {
        let (pieces, next_color_to_moves, castling_rights) = extract_game_state_from_fen(fen_string);
        return GameState { board: Board {squares: pieces}, selected_square: None, next_color_to_move: next_color_to_moves, castling_rights: castling_rights};
    }

    pub fn flip_turn(&mut self, sound_event: EventWriter<MoveSoundEvent>) {
        if self.next_color_to_move == SideColor::White {
            self.next_color_to_move = SideColor::Black;
        }else{
            self.next_color_to_move = SideColor::White;
        }
        self.scan_game_state(sound_event);
    }

    fn scan_game_state(&self, sound_event: EventWriter<MoveSoundEvent>) {
        let mut attack_data :AttackData = AttackData::new(&self.board, self.next_color_to_move.side_color_to_u8());
        attack_data.calculate_attack_data(&self.board, self.next_color_to_move.side_color_to_u8());

        Self::scan_checks_and_mates(self, sound_event, &attack_data);
    }

    fn scan_checks_and_mates(&self, mut sound_event: EventWriter<MoveSoundEvent>, attack_data: &AttackData) {
        if !attack_data.in_check {
            sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Move});
            return
        };
        for square_number in 0..64 {
            let legal_moves: Vec<PieceMove> = legal_move_generator(self, square_number);
            if !legal_moves.is_empty() {
                sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Check});
                return;
            }
        }
        sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Checkmate});
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