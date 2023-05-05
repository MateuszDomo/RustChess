use bevy::prelude::*;

use crate::chess_utility::{HighlightLegalMovesEvent, GameState, Square};
pub struct EventsPlugin ;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(highlight_legal_moves_system);
    }
}

fn highlight_legal_moves_system(
    mut events: EventReader<HighlightLegalMovesEvent>, 
    game_state: Res<GameState>,
    mut query: Query<(&Square,  &mut Sprite)>
) {

    for event in events.iter() {
        for (square, mut sprite) in query.iter_mut() {
            let row = square.square_number / 8;
            let col = square.square_number % 8;
            sprite.color = determine_square_color(row, col);
        }
        // Able to call same event system for both highlighting and unhighlighting moves
        if !event.highlight_new_moves {
            break;
        }
        let legal_moves = &game_state.selected_square.as_ref().expect("Should be selected square if trying to highlight moves").legal_moves;
        for (square, mut sprite) in query.iter_mut() {
            if legal_moves.contains(&square.square_number) {
                sprite.color = Color::rgb(1.0, 0.0, 0.5);
            }
        }
    }
}

fn determine_square_color(row: u32, col: u32) -> Color{
    let color = if (row+col) % 2 == 0{
        Color::rgb(0.5, 0.0, 0.5)
    }else{
        Color::rgb(1.0, 1.0, 0.0)
    };
    return color;
}