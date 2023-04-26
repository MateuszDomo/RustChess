use bevy::prelude::*;

use crate::{BoardLayout, board, GameState};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(mouse_input_system);
    }
}

fn mouse_input_system(
    kb: Res<Input<MouseButton>>, 
    board_layout: Res<BoardLayout>,
    mut windows: Query<&mut Window>, 
    mut game_state: ResMut<GameState>,
    
){
    let square_xy_positions = board_layout.square_positions;
    let square_width = board_layout.square_dimensions.width as f32;
    let square_height = board_layout.square_dimensions.height as f32;
    if kb.just_pressed(MouseButton::Left){
        let window = windows.single_mut();
        let mut square: u32 = 0;
    
        if let Some(pos) = window.cursor_position(){
            let pos: Vec2 = Vec2 { x: pos.x - 400., y: pos.y - 400.};
            for (index,square_xy) in square_xy_positions.iter().enumerate() {
                let x_bounds_met = (((*square_xy).0 - square_width/2.) < (pos.x)) &&  ((pos.x) < ((*square_xy).0 + square_width/2.));
                let y_bounds_met = (((*square_xy).1 - square_height/2.) < (pos.y)) &&  ((pos.y) < ((*square_xy).1 + square_height/2.));
                if x_bounds_met && y_bounds_met{
                    square = index as u32;
                }
            }
        }
        match game_state.selected_square{
            None => {
                game_state.selected_square = Some(square);
                println!("New Selected Square is {:?}",game_state.selected_square);
                // check if piece is there and generate moves if slected ppiece then do something else
            },
            Some(val) => {
                let old_square = game_state.selected_square.unwrap();
                game_state.selected_square = Some(square);
                println!("moving piece at {} to {:?}",old_square, game_state.selected_square);
                // move piece if valid
            },
        }
        
    } else if kb.just_pressed(MouseButton::Right) {
        game_state.selected_square = None;
        println!("Deselected Square. Val of Square is now {:?}", game_state.selected_square);
        // delect piece
    }
}
