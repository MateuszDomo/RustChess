

mod fen;
mod board;

use bevy::prelude::*;
use board::Board;

#[derive(Clone, Copy, Debug)]
 
pub enum PieceType {
    None  = 0, Pawn = 1, Bishop = 2, Knight= 3, Rook = 5, Queen = 6, King = 7
}

#[derive(Component)]
pub struct Square{
    square_number: u32
}

#[derive(Resource)]
pub struct SquareXYPositions{
    square_positions: [(f32,f32); 65], 
}
#[derive(Resource)]
pub struct GameTextures{
    piece_size: u32,
    b_p: Handle<Image>,
    b_b: Handle<Image>,
    b_n: Handle<Image>, 
    b_r: Handle<Image>, 
    b_q: Handle<Image>, 
    b_k: Handle<Image>, 
    w_p: Handle<Image>,
    w_b: Handle<Image>, 
    w_n: Handle<Image>,
    w_r: Handle<Image>, 
    w_q: Handle<Image>,
    w_k: Handle<Image>, 
}
struct SquareDimensions{
    width: u32,
    height: u32,
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            title: "Main Window".into(),
            resolution: (800., 800.).into(),
            ..default()
        }),
        ..default()}))
    .add_startup_system(setup_system)
    .run();
}


fn setup_system( 
    mut windows: Query<&mut Window>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default()); 

    let game_textures = GameTextures{
        piece_size: 5,
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
    let window = windows.single_mut();

    let square_xy_positions_array: [(f32, f32); 65] = calculate_square_positions(&window,&square_dimensions);
    spawn_squares(&square_xy_positions_array, &mut commands, &square_dimensions);

    let fen_string = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    let board: Board = Board{squares: fen::extract_pieces_from_fen(&fen_string)};

    for num in board.squares {
        println!("{:b}", num);
    }
    // Initialize resourcesW
    let square_xy_positions = SquareXYPositions{square_positions: square_xy_positions_array};
    commands.insert_resource(square_xy_positions);
    commands.insert_resource(game_textures);
}   



fn calculate_square_positions(window: &Window, square_dimensions: &SquareDimensions) -> [(f32, f32); 65] {

    let square_height: f32 = square_dimensions.height as f32;
    let square_width: f32 = square_dimensions.width as f32;
    let bottom_left_y: f32 = (-window.width()/2.)+(square_height/2.);
    let bottom_left_x: f32  = (-window.height()/2.)+(square_width/2.);

    let mut square_xy_positions: [(f32,f32); 65] = [(0.,0.); 65];
    for row in 0..8{
        for col in 0..8{
            square_xy_positions[row * 8 + col + 1] = (bottom_left_x + (col as f32 * square_width),bottom_left_y + (row as f32 * square_height));
        }
    }

    return square_xy_positions;
}

fn spawn_squares(square_xy_positions: &[(f32, f32); 65], commands: &mut Commands, square_dimensions: &SquareDimensions) {
    let square_height: f32 = square_dimensions.height as f32;
    let square_width: f32 = square_dimensions.width as f32;
    for row in 0..8{
        for col in 0..8{
            let square_number = row * 8 + col + 1;
            let (x_pos, y_pos) = square_xy_positions[square_number];
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

    let color = if (row+col) % 2 == 0{
        Color::rgb(0.5, 0.0, 0.5)
    }else{
        Color::rgb(1.0, 1.0, 0.0)
    };
    return color;
}
