mod player_input_plugin;
mod chess_utility;
mod fen;
mod board;
mod piece_spawns;
mod board_layout;
mod board_spawns;


use bevy::prelude::*;
use board_layout::BoardLayout;
use chess_utility::ChessSetupPlugin;
use player_input_plugin::PlayerInputPlugin;

#[derive(Component)]
pub struct Square{
    square_number: u32
}

pub enum SideColor {black, white}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            title: "Main Window".into(),
            resolution: (800., 800.).into(),
            ..default()
        }),
        ..default()}))
    .add_plugin(PlayerInputPlugin)
    .add_plugin(ChessSetupPlugin)
    .run();
}


  





