use bevy::prelude::*;
use crate::{chess_utility::{HighlightLegalMovesEvent, MoveSoundEvent}, game_state::GameState, legal_move_generator::legal_move_generator, piece_move::PieceMove, piece_movement_utility::move_pieces, piece_spawns::Piece, BoardLayout};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_input_system);
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
    sound_event: EventWriter<MoveSoundEvent>,
) {

    if game_state.pause == true {
        return;
    }

    let square_xy_positions = board_layout.square_xy_positions;
    let square_width = board_layout.square_dimensions.width as f32;
    let square_height = board_layout.square_dimensions.height as f32;
    
    if kb.just_pressed(MouseButton::Left) {
        
        let selected_square_option: Option<u32> = find_selected_square(windows, square_width, square_height, square_xy_positions);
        let selected_square: u32;
        match selected_square_option {
            Some(square) => selected_square = square,
            None => return,
        }

        match game_state.selected_square {
            None => {
                if game_state.board.squares[selected_square as usize] == 0 {
                    println!("no pieces selected");
                    return;
                }

                // Ensures a player only makes a move if its their turn
                if game_state.board.piece_color_to_side_color(selected_square) != game_state.next_color_to_move {
                    println!("Not this side's turn!");
                    return;
                }

                // Select Pieces
                let legal_moves = legal_move_generator(game_state.as_ref(), selected_square);
                game_state.selected_square = Some(selected_square);
                highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: true, legal_moves: Some(legal_moves)});
            },
            Some(previously_selected_square) => {
                
                // Switch Pieces
                if (game_state.board.squares[selected_square as usize] & 0b00011000) == (game_state.board.squares[previously_selected_square as usize] & 0b00011000) && selected_square != previously_selected_square {
                    let legal_moves = legal_move_generator(game_state.as_ref(), selected_square);
                    game_state.selected_square = Some(selected_square);
                    highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: true, legal_moves: Some(legal_moves)});
                    return;
                }
                let legal_moves = legal_move_generator(game_state.as_ref(), previously_selected_square);

                // Capture/Move
                if let Some(legal_move) = get_legal_move(legal_moves, &selected_square) {
                    move_pieces(pieces,  board_layout.as_ref(), commands, game_state.as_mut(), legal_move.clone());
                    game_state.flip_turn(sound_event, legal_move.clone());
                }

                // Deselect
                game_state.selected_square = None;
                highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: false, legal_moves: None});
            },
        }
    } else if kb.just_pressed(MouseButton::Right) {
        // Deselect
        game_state.selected_square = None;
        highlight_legal_move_event.send(HighlightLegalMovesEvent {highlight_new_moves: false, legal_moves: None});
    }
}




fn find_selected_square(mut windows: Query<&mut Window>, square_width: f32, square_height: f32, square_xy_positions: [(f32, f32); 64]) -> Option<u32> {
    let window = windows.single_mut();
    let mut square: Option<u32> = None;

    if let Some(pos) = window.cursor_position(){
        // Cursor position (x,y) of the bottom left of the window is (0,height of window)
        // While window coordinate (x,y) of the bottom left of the window is (-width/2,-height/2)
        let pos: Vec2 = Vec2 { x: pos.x - 400., y: 400. - pos.y};
        for (index,square_xy) in square_xy_positions.iter().enumerate() {
            let x_bounds_met = (((*square_xy).0 - square_width/2.) < (pos.x)) &&  ((pos.x) < ((*square_xy).0 + square_width/2.));
            let y_bounds_met = (((*square_xy).1 - square_height/2.) < (pos.y)) &&  ((pos.y) < ((*square_xy).1 + square_height/2.));
            if x_bounds_met && y_bounds_met {
                square = Some(index as u32);
            }
        }
    }
    return square
}

pub fn get_legal_move(legal_moves: Vec<PieceMove>,  square: &u32) -> Option<PieceMove> {
    for legal_move in legal_moves {
        if legal_move.target_square() == *square {
            return Some(legal_move);
        }
    }
    return None
}