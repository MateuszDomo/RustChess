use bevy::prelude::*;

use crate::chess_utility::SquareDimensions;

#[derive(Resource)]
pub struct BoardLayout { // Dimensions in px
    pub square_xy_positions: [(f32,f32); 64], 
    pub square_dimensions: SquareDimensions,
    pub board_height: u32,
    pub board_width: u32,
}

impl BoardLayout{
    pub fn new(mut windows: Query<&mut Window>, square_dimensions: SquareDimensions) -> Self{
        let window: Mut<'_, Window> = windows.single_mut();
        let square_xy_positions: [(f32, f32); 64] = Self::calculate_square_positions(&window,&square_dimensions);
        let board_height: u32 = square_dimensions.height * 8; 
        let board_width: u32 = square_dimensions.width * 8; 
        return BoardLayout{
            square_xy_positions: square_xy_positions,
            square_dimensions: square_dimensions,
            board_height: board_height,
            board_width: board_width,
        }
    }

    fn calculate_square_positions(window: &Window, square_dimensions: &SquareDimensions) -> [(f32, f32); 64] {

        let square_height: f32 = square_dimensions.height as f32;
        let square_width: f32 = square_dimensions.width as f32;
        let bottom_left_y: f32 = (-window.width()/2.)+(square_height/2.);
        let bottom_left_x: f32  = (-window.height()/2.)+(square_width/2.);
    
        let mut square_xy_positions: [(f32,f32); 64] = [(0.,0.); 64];
        for row in 0..8{
            for col in 0..8{
                square_xy_positions[row * 8 + col] = (bottom_left_x + (col as f32 * square_width),bottom_left_y + (row as f32 * square_height));
            }
        }
    
        return square_xy_positions;
    }
}