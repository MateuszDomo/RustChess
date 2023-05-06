use bevy::prelude::*;
use crate::{BoardLayout, board::{Board}, piece_spawns::Piece, chess_utility::{GameState, HighlightLegalMovesEvent}, selected_square::SelectedSquare};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_input_system);
    }
}



fn mouse_input_system(
    kb: Res<Input<MouseButton>>, 
    board_layout: Res<BoardLayout>,
    windows: Query<&mut Window>, 
    mut game_state: ResMut<GameState>,
    pieces: Query<(Entity, &mut Piece, &mut Transform)>,
    commands: Commands,
    mut highlight_legal_move_event: EventWriter<HighlightLegalMovesEvent>,
) {
    let square_xy_positions = board_layout.square_xy_positions;
    let square_width = board_layout.square_dimensions.width as f32;
    let square_height = board_layout.square_dimensions.height as f32;
    if kb.just_pressed(MouseButton::Left) {
    let selected_square: u32 = find_selected_square(windows, square_width, square_height, square_xy_positions);
    let selected_square_struct = game_state.selected_square.clone();
        match selected_square_struct {
            None => {
                if game_state.board.squares[selected_square as usize] == 0 {
                    println!("no pieces selected");
                    return;
                }
                // Select Piece
                game_state.selected_square = Some(SelectedSquare::new(selected_square, game_state.as_ref()));
                highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: true});
            },
            Some(previously_selected_square) => {
                if previously_selected_square.legal_moves.contains(&selected_square) && game_state.board.squares[selected_square as usize] == 0 {
                    // Move Piece
                    game_state.selected_square = None;
                    move_piece(pieces, previously_selected_square.square_number, selected_square, square_xy_positions, commands, &mut game_state.board);
                    highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: false});
                } else if (game_state.board.squares[selected_square as usize]) != (game_state.board.squares[previously_selected_square.square_number as usize]) && (game_state.board.squares[selected_square as usize] & 0b00001000) == (game_state.board.squares[previously_selected_square.square_number as usize] & 0b00001000) {
                    // Switch Piece
                    game_state.selected_square = Some(SelectedSquare::new(selected_square, game_state.as_ref()));
                    highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: true});
                } else if previously_selected_square.legal_moves.contains(&selected_square) {
                    // Attack Piece
                    game_state.selected_square = None;
                    move_piece(pieces, previously_selected_square.square_number, selected_square, square_xy_positions, commands, &mut game_state.board);
                    highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: false});
                } else {
                    // Deselect
                    game_state.selected_square = None;
                    highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: false});
                }
                
            },
        }
    } else if kb.just_pressed(MouseButton::Right) {
        // Deselect
        game_state.selected_square = None;
        highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: false});
    }
}

fn move_piece(mut query:  Query<(Entity, &mut Piece, &mut Transform)>, from_square: u32, to_square: u32, square_xy_positions: [(f32, f32); 64], mut commands: Commands, board: &mut Board) {
    for (entity, piece, _) in query.iter_mut() {
        if piece.square_pos_number == to_square{
            commands.entity(entity).despawn();
        }
    }
    for (_, mut piece, mut transform) in query.iter_mut() {
        if piece.square_pos_number == from_square{
            let translation = &mut transform.translation;   
            translation.x = square_xy_positions[to_square as usize].0;
            translation.y = square_xy_positions[to_square as usize].1;
            piece.square_pos_number = to_square;
        }
    }
    board.squares[to_square as usize] = board.squares[from_square as usize];
    board.squares[from_square as usize] = 0;
}

fn find_selected_square(mut windows: Query<&mut Window>, square_width: f32, square_height: f32, square_xy_positions: [(f32, f32); 64]) -> u32{
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
    return square
}