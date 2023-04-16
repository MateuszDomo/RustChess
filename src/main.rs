use bevy::prelude::*;

#[derive(Component)]
struct Square{
    square_number: u32
}

#[derive(Resource)]
struct SquareXYPositions{
    square_positions: [(f32,f32); 65], 
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

struct SquareDimensions{
    width: u32,
    height: u32,
}

fn setup_system( mut windows: Query<&mut Window>,mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); 

    let  square_dimensions = SquareDimensions{width: 100, height: 100};
    let window = windows.single_mut();

    let square_xy_positions_array: [(f32, f32); 65] = calculate_square_positions(&window,&square_dimensions);
    spawn_squares(&square_xy_positions_array, &mut commands, &square_dimensions);
    // Initialize resources
    let square_xy_positions = SquareXYPositions{square_positions: square_xy_positions_array};
    commands.insert_resource(square_xy_positions);
}   

fn calculate_square_positions(window: &Window, square_dimensions: &SquareDimensions) -> [(f32, f32); 65] {

    let square_height: f32 = square_dimensions.height as f32;
    let square_width: f32 = square_dimensions.width as f32;
    let bottom_left_y: f32 = (-window.width()/2.)+(square_height/2.);
    let bottom_left_x: f32  = (-window.height()/2.)+(square_width/2.);

    let mut square_positions: [(f32,f32); 65] = [(0.,0.); 65];
    for row in 0..8{
        for col in 0..8{
            square_positions[row * 8 + col + 1] = (bottom_left_x + (col as f32 * square_width),bottom_left_y + (row as f32 * square_height));
        }
    }

    return square_positions;
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
