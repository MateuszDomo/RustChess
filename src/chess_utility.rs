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
    pub next_to_move: SideColor,
}

pub struct SquareDimensions{
    pub width: u32,
    pub height: u32,
}

#[derive(Component)]
pub struct Square{
    pub square_number: u32
}

pub enum SideColor {black, white}