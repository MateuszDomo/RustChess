use bevy::prelude::*;

use crate::{board_layout::BoardLayout, chess_utility::SideColor, game_state::GameState, piece_move::{Flag, PieceMove}, piece_spawns::Piece};

pub fn move_pieces(mut query:  Query<(Entity, &mut Piece, &mut Transform)>, board_layout: &BoardLayout, mut commands: Commands, game_state: &mut GameState, piece_move: PieceMove) {
    let from_square: u32 = piece_move.starting_square();
    let to_square: u32 = piece_move.target_square();
    let flag: Flag = piece_move.flag();
    
    game_state.enpassant_target = None; // Reset enpassant target
    match flag {
        Flag::Castle => {
            castle_move(&mut commands, game_state, board_layout, &mut query, to_square, from_square);
        },
        Flag::EnpassantTarget => {
            enpassant_target_move(game_state, to_square);
        }
        Flag::EnpassantCapture => {
            enpassant_capture_move(&mut commands, game_state, &mut query, to_square);
        }
        Flag::Promote => {
            //  move pawn to square -> Pause -> send event with info -> spawn button -> wait for selection -> replace pawn with selection -> delete button -> unpause
            // Pause using var in gamestate and disable user input on pause.
            promote_move(&mut commands, game_state, board_layout, to_square);
        }
        _ => (),
    }

    move_piece(&mut query, board_layout, &mut commands, game_state, from_square, to_square);
}

fn castle_move(commands: &mut Commands, game_state: &mut GameState, board_layout: &BoardLayout, query:  &mut Query<(Entity, &mut Piece, &mut Transform)>, to_square: u32, from_square: u32) {
    if to_square < from_square {
        move_piece(query, board_layout, commands, game_state, from_square-4, from_square-1);
    } else {
        move_piece(query, board_layout, commands, game_state, from_square+3, from_square+1);
    }

}

fn enpassant_target_move(game_state: &mut GameState, to_square: u32) {
    let enpassant_target: u32;
    if game_state.next_color_to_move == SideColor::White {
        enpassant_target = to_square - 8;
    } else {
        enpassant_target = to_square + 8;
    }
    game_state.enpassant_target = Some(enpassant_target);
}

fn enpassant_capture_move(commands: &mut Commands, game_state: &mut GameState, query:  &mut Query<(Entity, &mut Piece, &mut Transform)>, to_square: u32) {
    let enpassant_capture: u32;
    if game_state.next_color_to_move == SideColor::White {
        enpassant_capture = to_square - 8;
    } else {
        enpassant_capture = to_square + 8;
    }
    // Delete pawn captured through enpassant
    delete_piece(query, commands, enpassant_capture);
    game_state.board.squares[enpassant_capture as usize] = 0;
}

fn promote_move(commands: &mut Commands, game_state: &mut GameState, board_layout: &BoardLayout, to_square: u32) {
    let square_xy_positions: [(f32, f32); 64] = board_layout.square_xy_positions;
    let from_left = square_xy_positions[to_square as usize].0 + 350.;
    let from_bottom = (board_layout.board_height as f32 - board_layout.square_dimensions.height as f32) - (square_xy_positions[to_square as usize].1 + 350.);
    game_state.pause = true;
        commands
        .spawn(ButtonBundle {
            style: Style {
            left: Val::Px(from_left),
            top: Val::Px(from_bottom),
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            border: UiRect::all(Val::Px(5.0)),
            ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: Color::BLACK.into(),
            ..default()
        });
}

#[derive(Component)]
pub struct PromotionButton;

fn move_piece(query:  &mut Query<(Entity, &mut Piece, &mut Transform)>, board_layout: &BoardLayout, commands: &mut Commands, game_state: &mut GameState, from_square: u32, to_square: u32) {
    let board = &mut game_state.board;
    delete_piece(query, commands, to_square);
    let square_xy_positions: [(f32, f32); 64] = board_layout.square_xy_positions;
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

fn delete_piece(query:  &mut Query<(Entity, &mut Piece, &mut Transform)>, commands: &mut Commands, to_square: u32) {
    for (entity, piece, _) in query.iter_mut() {
            if piece.square_pos_number == to_square{
                commands.entity(entity).despawn();
            }
    }
}