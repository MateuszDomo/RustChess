use bevy::prelude::*;

use crate::{board::Board, attack_data::AttackData, legal_move_generator::legal_move_generator};

#[derive(Resource,Clone)]
pub struct GameTextures {
    pub piece_size: f32,    pub b_p: Handle<Image>,
    pub b_b: Handle<Image>,
    pub b_n: Handle<Image>, 
    pub b_r: Handle<Image>, 
    pub b_q: Handle<Image>, 
    pub b_k: Handle<Image>, 
    pub w_p: Handle<Image>,
    pub w_b: Handle<Image>, 
    pub w_n: Handle<Image>,
    pub w_r: Handle<Image>, 
    pub w_q: Handle<Image>,
    pub w_k: Handle<Image>, 
}

#[derive(Resource)]
pub struct GameAudio {
    pub move_audio: MoveAudio,
}

#[derive(Clone)]
pub struct MoveAudio {
    pub move_move: Handle<AudioSource>,
    pub move_capture: Handle<AudioSource>,
    pub move_check: Handle<AudioSource>,
    pub move_checkmate: Handle<AudioSource>,
    pub move_promote: Handle<AudioSource>,
    pub move_castle: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct GameState {
    pub board: Board,
    pub selected_square: Option<u32>,
    pub next_side_color_to_move: SideColor,
}

impl GameState {
    pub fn flip_turn(&mut self, sound_event: EventWriter<MoveSoundEvent>) {
        if self.next_side_color_to_move == SideColor::White {
            self.next_side_color_to_move = SideColor::Black;
        }else{
            self.next_side_color_to_move = SideColor::White;
        }
        self.scan_game_state(sound_event);
    }

    fn scan_game_state(&mut self, mut sound_event: EventWriter<MoveSoundEvent>) {
        let mut attack_data :AttackData = AttackData::new(&self.board, self.next_side_color_to_move.side_color_to_u8());
        attack_data.calculate_attack_data(&self.board, self.next_side_color_to_move.side_color_to_u8());
        if !attack_data.in_check {
            sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Move});
            return
        };
        for square_number in 0..64 {
            let legal_moves: Vec<u32> = legal_move_generator(self, square_number);
            if !legal_moves.is_empty() {
                sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Check});
                return;
            }
        }
        sound_event.send(MoveSoundEvent {move_sound: MoveSounds::Checkmate});
    }
}

pub struct SquareDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Component)]
pub struct Square {
    pub square_number: u32
}
#[derive(Component)]
pub struct HighlightedSquare;

#[derive(PartialEq)]
pub enum SideColor {Black, White}

impl SideColor {
    pub fn side_color_to_u8(&self) -> u8 {
        match self {
            SideColor::White => return 0b00001000,
            SideColor::Black => return 0b00010000,
        }
    }
}

#[derive(Event)]
pub struct HighlightLegalMovesEvent {
    pub highlight_new_moves: bool,
    pub legal_moves: Option<Vec<u32>>,
}

pub enum MoveSounds  {Move, Capture, Check, Checkmate, Promote, Castle}

#[derive(Event)]
pub struct MoveSoundEvent {
    pub move_sound: MoveSounds,
}

pub fn square_from_rank_file(rank: u32, file: u32) -> u32 {
    return (rank - 1) * 8 + (file - 1);
}