use bevy::prelude::*;

use crate::{board::Board};

#[derive(Resource,Clone)]
pub struct GameTextures{
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
pub struct GameState{
    pub board: Board,
    pub selected_square: Option<u32>,
    pub next_side_color_to_move: SideColor,
}

impl GameState {
    pub fn flip_turn(&mut self) {
        if self.next_side_color_to_move == SideColor::White {
            self.next_side_color_to_move = SideColor::Black;
        }else{
            self.next_side_color_to_move = SideColor::White;
        }
    }
}

pub struct SquareDimensions{
    pub width: u32,
    pub height: u32,
}

#[derive(Component)]
pub struct Square{
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

pub struct HighlightLegalMovesEvent {
    pub highlight_new_moves: bool,
    pub legal_moves: Option<Vec<u32>>,
}

pub fn squareFromRankFile(rank: u32, file: u32) -> u32 {
    return (rank - 1) * 8 + (file - 1);
}