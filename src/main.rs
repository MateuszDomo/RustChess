mod chess_utility;
mod fen;
mod board;
mod piece_spawns;
mod board_layout;
mod board_spawns;
mod legal_move_generator;
mod attack_bitmap;
mod attack_data;
mod piece_move;
mod game_state;
mod plugins{
    pub mod chess_startup_plugin;
    pub mod player_input_plugin;
    pub mod events_plugin;
}

use bevy::prelude::*; 
use board_layout::BoardLayout;
use chess_utility::{HighlightLegalMovesEvent, MoveSoundEvent};
use plugins::{player_input_plugin::PlayerInputPlugin, chess_startup_plugin::ChessSetupPlugin, events_plugin::EventsPlugin};



fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            title: "Main Window".into(),
            resolution: (800., 800.).into(),
            resizable: false,
            ..default()
        }),
        ..default()}),)
    .add_plugins((
        PlayerInputPlugin, 
        ChessSetupPlugin, 
        EventsPlugin
    ))
    .add_event::<HighlightLegalMovesEvent>()
    .add_event::<MoveSoundEvent>()
    .run();

}