use bevy::prelude::*;

use crate::{chess_utility::{HighlightLegalMovesEvent, HighlightedSquare, MoveSoundEvent, MoveSounds, GameAudio, MoveAudio}, board_layout::BoardLayout};
pub struct EventsPlugin ;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (highlight_legal_moves_event_system,move_sounds_system));
    }
}


fn highlight_legal_moves_event_system(
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
            for legal_move in legal_moves {
                let (x_pos, y_pos) = board_layout.square_xy_positions[legal_move.target_square() as usize];
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::Rgba { red: 1., green: 0., blue: 0., alpha: 0.65 },
                        custom_size: Some(Vec2::new(board_layout.square_dimensions.width as f32, board_layout.square_dimensions.height as f32)),
                        ..default()
                    },
                transform: Transform::from_translation(Vec3::new(x_pos, y_pos, 3.)),
                    ..default()
                }).insert(HighlightedSquare); 
            }
        }
    }
}

fn move_sounds_system(
    mut events: EventReader<MoveSoundEvent>, 
    mut commands: Commands,
    game_audio: Res<GameAudio>,
) {
    let move_audio = &game_audio.move_audio;
    for event in events.iter() {
        commands.spawn(AudioBundle {
            source: get_move_audio(&event.move_sound, move_audio),
            ..default()
        });
    }
}

fn get_move_audio(move_sound: &MoveSounds, game_sounds: &MoveAudio) -> Handle<AudioSource> {
    match move_sound {
        MoveSounds::Move => {
            return game_sounds.move_move.clone();
        },
        MoveSounds::Capture => {
            println!("CAPTURE");
            return game_sounds.move_capture.clone();
        },
        MoveSounds::Check => {
            return game_sounds.move_check.clone();
        },
        MoveSounds::Checkmate => {
            return game_sounds.move_checkmate.clone();
        },
        MoveSounds::Promote => {
            println!("PROMOTE");
            return game_sounds.move_promote.clone();
        },
        MoveSounds::Castle => {
            return game_sounds.move_castle.clone();
        }
    }
}