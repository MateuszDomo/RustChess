use bevy::prelude::*;

use crate::{chess_utility::{SquareDimensions, Square}};

pub fn spawn_squares(square_xy_positions: &[(f32, f32); 64], commands: &mut Commands, square_dimensions: &SquareDimensions) {
    let square_height: f32 = square_dimensions.height as f32;
    let square_width: f32 = square_dimensions.width as f32;
    for row in 0..8{
        for col in 0..8{
            let square_number = row * 8 + col;
            let (x_pos, y_pos) = square_xy_positions[square_number];
            println!("POS: ");
            println!("{}", x_pos);
            println!("{}", y_pos);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: determine_square_color(row as u32,col as u32),
                    custom_size: Some(Vec2::new(square_width, square_height)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x_pos, y_pos, 0.)),
                ..default()
            }).insert(Square{square_number: square_number as u32});
        }
    }
}

fn determine_square_color(row: u32, col: u32) -> Color{
    let color = if (row+col) % 2 != 0{
        Color::rgb(0.8157, 0.8745, 0.9569)
    }else{
        Color::rgb(0.2941, 0.3922, 0.5412)
    };
    return color;
}