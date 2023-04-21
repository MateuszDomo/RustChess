use bevy::prelude::*;
use crate::{board::Board, SquareXYPositions, PieceType, GameTextures};



// pub struct PieceSpawner{
//     xy_positions: [(f32,f32); 64],
//     textures: GameTextures,
//     pawn: u8,
//     bishop: u8,
//     knight: u8,
//     rook: u8,
//     queen: u8,
//     king: u8,
// }

// impl PieceSpawner{
//     pub fn new(game_textures: GameTextures) -> PieceSpawner{
//         let pawn = PieceType::Pawn as u8;
//         let bishop = PieceType::Bishop as u8;
//         let knight = PieceType::Knight as u8;
//         let rook = PieceType::Rook as u8;
//         let queen = PieceType::Queen as u8;
//         let king = PieceType::King as u8;
//         return PieceSpawner { 
//             textures: game_textures, 
//             pawn: pawn, 
//             bishop: bishop, 
//             knight: knight, 
//             rook: rook, 
//             queen: queen, 
//             king: king 
//         }
//     }
// }

pub fn spawn_pieces(board: &Board, square_xy_positions: [(f32, f32); 64], commands: &mut Commands, game_textures: &GameTextures) {

    let mut square_number = 0;
    println!("{:?}",board.squares);
    for piece in board.squares{
        let is_white: bool = (0b00001000 & piece) == 0b00001000;

        println!("{}",is_white);
        println!("{:b}",piece);
        println!("{:b}",piece & 0b00000111);
        println!();
        let pawn = PieceType::Pawn as u8;
        let bishop = PieceType::Bishop as u8;
        let knight = PieceType::Knight as u8;
        let rook = PieceType::Rook as u8;
        let queen = PieceType::Queen as u8;
        let king = PieceType::King as u8;
   
        match piece & 0b00000111{
            1 => spawn_pawn(commands, square_number, &game_textures, square_xy_positions, is_white),
            2 => spawn_bishop(commands, square_number, &game_textures, square_xy_positions, is_white ),
            3 => spawn_knight(commands, square_number, &game_textures, square_xy_positions, is_white ),
            5 => spawn_rook(commands, square_number, &game_textures, square_xy_positions, is_white ),
            6 => spawn_queen(commands, square_number, &game_textures, square_xy_positions, is_white ),
            7 => spawn_king(commands, square_number, &game_textures, square_xy_positions, is_white ),
            _ => (),
        }
        square_number += 1;

     
    }
}
// make game textures and square positions in struct
pub fn spawn_pawn(commands: &mut Commands, square_number: u32, game_textures: &GameTextures, xy_positions: [(f32, f32); 64], is_white: bool) {
    let (x,y) = xy_positions[square_number as usize];
    let pawn_texture = if is_white{
        &game_textures.w_p
    } else{
        &game_textures.b_p
    };
    commands.spawn(SpriteBundle{
        texture: pawn_texture.clone(),
        transform: Transform{
            translation: Vec3::new(x, y, 2.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_bishop(commands: &mut Commands, square_number: u32, game_textures: &GameTextures, xy_positions: [(f32, f32); 64], is_white: bool) {
    let (x,y) = xy_positions[square_number as usize];
    let pawn_texture = if is_white{
        &game_textures.w_b
    } else{
        &game_textures.b_b
    };
    commands.spawn(SpriteBundle{
        texture: pawn_texture.clone(),
        transform: Transform{
            translation: Vec3::new(x, y, 2.0),
            ..default()
        },
        ..default()
    });
} 

fn spawn_knight(commands: &mut Commands, square_number: u32, game_textures: &GameTextures, xy_positions: [(f32, f32); 64], is_white: bool) {
    let (x,y) = xy_positions[square_number as usize];
    let pawn_texture = if is_white{
        &game_textures.w_n
    } else{
        &game_textures.b_n
    };
    commands.spawn(SpriteBundle{
        texture: pawn_texture.clone(),
        transform: Transform{
            translation: Vec3::new(x, y, 2.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_rook(commands: &mut Commands, square_number: u32, game_textures: &GameTextures, xy_positions: [(f32, f32); 64], is_white: bool) {
    let (x,y) = xy_positions[square_number as usize];
    let pawn_texture = if is_white{
        &game_textures.w_r
    } else{
        &game_textures.b_r
    };
    commands.spawn(SpriteBundle{
        texture: pawn_texture.clone(),
        transform: Transform{
            translation: Vec3::new(x, y, 2.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_queen(commands: &mut Commands, square_number: u32, game_textures: &GameTextures, xy_positions: [(f32, f32); 64], is_white: bool){
    let (x,y) = xy_positions[square_number as usize];
    let pawn_texture = if is_white{
        &game_textures.w_q
    } else{
        &game_textures.b_q
    };
    commands.spawn(SpriteBundle{
        texture: pawn_texture.clone(),
        transform: Transform{
            translation: Vec3::new(x, y, 2.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_king(commands: &mut Commands, square_number: u32, game_textures: &GameTextures, xy_positions: [(f32, f32); 64], is_white: bool){
    let (x,y) = xy_positions[square_number as usize];
    let pawn_texture = if is_white{
        &game_textures.w_k
    } else{
        &game_textures.b_k
    };
    commands.spawn(SpriteBundle{
        texture: pawn_texture.clone(),
        transform: Transform{
            translation: Vec3::new(x, y, 2.0),
            ..default()
        },
        ..default()
    });
}
