use bevy::prelude::*;

use crate::piece_move::PieceMove;

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
    pub legal_moves: Option<Vec<PieceMove>>,
}

pub enum MoveSounds  {Move, Capture, Check, Checkmate, Promote, Castle}

#[derive(Event)]
pub struct MoveSoundEvent {
    pub move_sound: MoveSounds,
}

pub fn square_from_rank_file(rank: u32, file: u32) -> u32 {
    return (rank - 1) * 8 + (file - 1);
}

pub fn print_bitmap(bitmap: u64) {
    let value: u64 = bitmap;
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index: i32 = rank * 8 + file;
            let mask: u64 = 1 << index;
            let piece: u64 = (value & mask) >> index;

            print!("{} ", piece);
        }
        println!();
    }
    println!()
}