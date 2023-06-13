use bevy::prelude::*;

use crate::{chess_utility::{GameTextures, SquareDimensions, GameState, SideColor}, board_layout::BoardLayout, board::Board, fen, board_spawns::spawn_squares, piece_spawns::PieceSpawner};
pub struct ChessSetupPlugin;

impl Plugin for ChessSetupPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system(chess_setup_system);
    }
}

fn chess_setup_system( 
    windows: Query<&mut Window>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default()); 

    let game_textures = GameTextures{
        piece_size: 0.75,
        b_p: asset_server.load("b_pawn.png"),
        b_b: asset_server.load("b_bishop.png"),
        b_n: asset_server.load("b_knight.png"),
        b_r: asset_server.load("b_rook.png"),
        b_q: asset_server.load("b_queen.png"),
        b_k: asset_server.load("b_king.png"),
        w_p: asset_server.load("w_pawn.png"),
        w_b: asset_server.load("w_bishop.png"),
        w_n: asset_server.load("w_knight.png"),
        w_r: asset_server.load("w_rook.png"),
        w_q: asset_server.load("w_queen.png"),
        w_k: asset_server.load("w_king.png"),
    };

    let  square_dimensions = SquareDimensions{width: 100, height: 100};
    let board_layout: BoardLayout = BoardLayout::new(windows, square_dimensions);
    spawn_squares(&board_layout.square_xy_positions, &mut commands, &board_layout.square_dimensions);

    let fen_string = String::from("rnbqkbnr/pppp1ppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    let board: Board = Board{squares: fen::extract_pieces_from_fen(&fen_string)};

    let piece_spawner = PieceSpawner::new(game_textures.clone(), board_layout.square_xy_positions);
    piece_spawner.spawn_pieces(&mut commands, &board);

    let game_state = GameState{board: board, selected_square: None, next_side_color_to_move: SideColor::White};
    
    commands.insert_resource(game_state);
    commands.insert_resource(board_layout);
    commands.insert_resource(game_textures);
} 