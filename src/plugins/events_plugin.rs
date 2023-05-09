use bevy::prelude::*;

use crate::{chess_utility::{HighlightLegalMovesEvent, Square, HighlightedSquare}, board_layout::{BoardLayout, self}};
pub struct EventsPlugin ;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(highlight_legal_moves_system);
    }
}

fn highlight_legal_moves_system(
    mut events: EventReader<HighlightLegalMovesEvent>, 
    mut query: Query<Entity, With<HighlightedSquare>>,
    board_layout: Res<BoardLayout>,
    mut commands: Commands,
) {

    for event in events.iter() {

        for highlighted_square in query.iter_mut() {
            commands.entity(highlighted_square).despawn();
        }
        // Able to call same event system for both highlighting and unhighlighting moves
        if !event.highlight_new_moves {
            break;
        }

        let legal_moves = &event.legal_moves;
        if let Some(legal_moves) = legal_moves {
            for legal_move_square in legal_moves {
                let (x_pos, y_pos) = board_layout.square_xy_positions[*legal_move_square as usize];
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::Rgba { red: 1., green: 0., blue: 0., alpha: 0.75 },
                        custom_size: Some(Vec2::new(board_layout.square_dimensions.width as f32, board_layout.square_dimensions.height as f32)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x_pos, y_pos, 0.)),
                    ..default()
                }).insert(HighlightedSquare); 
            }

                
            
        }
    }
}