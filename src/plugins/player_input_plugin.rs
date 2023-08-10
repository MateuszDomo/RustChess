use bevy::prelude::*;
use crate::{BoardLayout, piece_spawns::Piece, chess_utility::{HighlightLegalMovesEvent, MoveSoundEvent, SideColor}, legal_move_generator::legal_move_generator, board::Board, piece_move::{PieceMove, Flag}, game_state::{GameState, self, CastlingRights}};

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
    let square_xy_positions = board_layout.square_xy_positions;
    let square_width = board_layout.square_dimensions.width as f32;
    let square_height = board_layout.square_dimensions.height as f32;
    
    if kb.just_pressed(MouseButton::Left) {
        
        let selected_square: u32 = find_selected_square(windows, square_width, square_height, square_xy_positions);
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
                    move_pieces(pieces,  square_xy_positions, commands, game_state.as_mut(), legal_move);
                    game_state.flip_turn(sound_event);
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

fn move_pieces(mut query:  Query<(Entity, &mut Piece, &mut Transform)>, square_xy_positions: [(f32, f32); 64], mut commands: Commands, game_state: &mut GameState, piece_move: PieceMove) {

    let from_square: u32 = piece_move.starting_square();
    let to_square: u32 = piece_move.target_square();
    let flag: Flag = piece_move.flag();
    let board: &mut Board = &mut game_state.board;
    let active_color: &SideColor = &game_state.next_color_to_move;
    match flag {
        Flag::Castle => {
            if to_square < from_square {
                move_piece(&mut query, square_xy_positions, &mut commands, board, from_square-4, from_square-1);
            } else {
                move_piece(&mut query, square_xy_positions, &mut commands, board, from_square+3, from_square+1);
            }
        },
        _ => (),
    }
    
    move_piece(&mut query, square_xy_positions, &mut commands, board, from_square, to_square);
    scan_castling_rights(to_square, board, &mut game_state.castling_rights, active_color);
}

fn move_piece(query:  &mut Query<(Entity, &mut Piece, &mut Transform)>, square_xy_positions: [(f32, f32); 64], commands: &mut Commands, board: &mut Board, from_square: u32, to_square: u32) {
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

fn scan_castling_rights(to_square: u32, board: &Board, castling_rights: &mut CastlingRights, active_color: &SideColor) {
    if board.contains_king(to_square) {
        castling_rights.revoke_all(active_color)
    } else if board.contains_rook(to_square) {
        if (to_square % 8) + 1 == 1 {
            castling_rights.revoke_long(active_color);
        } else if (to_square % 8) + 1 == 8 {
            castling_rights.revoke_short(active_color);
        }
    }
}

fn find_selected_square(mut windows: Query<&mut Window>, square_width: f32, square_height: f32, square_xy_positions: [(f32, f32); 64]) -> u32{
    let window = windows.single_mut();
    let mut square: u32 = 0;

    if let Some(pos) = window.cursor_position(){
        // Cursor position (x,y) of the bottom left of the window is (0,height of window)
        // While window coordinate (x,y) of the bottom left of the window is (-width/2,-height/2)
        let pos: Vec2 = Vec2 { x: pos.x - 400., y: 400. - pos.y};
        
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

pub fn get_legal_move(legal_moves: Vec<PieceMove>,  square: &u32) -> Option<PieceMove> {
    for legal_move in legal_moves {
        if legal_move.target_square() == *square {
            return Some(legal_move);
        }
    }
    return None
}