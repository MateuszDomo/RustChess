use bevy::prelude::*;

use crate::{BoardLayout, board};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(mouse_input_system);
    }
}

fn mouse_input_system(kb: Res<Input<MouseButton>>, board_layout: Res<BoardLayout>, mut windows: Query<&mut Window>){
    let square_xy_positions = board_layout.square_positions;
    let square_width = board_layout.square_dimensions.width as f32;
    let square_height = board_layout.square_dimensions.height as f32;
    // println!("{:?}",square_xy_positions);
    if kb.just_pressed(MouseButton::Left){
        let window = windows.single_mut();
        let square: i32 = 0;
    
        if let Some(pos) = window.cursor_position(){
            let pos: Vec2 = Vec2 { x: pos.x - 400., y: pos.y - 400.};
            for (index,square_xy) in square_xy_positions.iter().enumerate() {
                // println!("{:?}",square_xy);
                let x_bounds_met = (((*square_xy).0 - square_width/2.) < (pos.x)) &&  ((pos.x) < ((*square_xy).0 + square_width/2.));
                let y_bounds_met = (((*square_xy).1 - square_height/2.) < (pos.y)) &&  ((pos.y) < ((*square_xy).1 + square_height/2.));
                if x_bounds_met && y_bounds_met{
                    println!("{}",index);
                }
            }
        }

        
    }
}
