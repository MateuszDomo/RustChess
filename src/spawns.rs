use bevy::prelude::*;
use crate::{board::Board, SquareXYPositions, GameTextures};

#[derive(Component)]
pub struct Piece{
    square_pos_number: u32,
}

pub struct PieceSpawner{
    pub xy_positions: [(f32,f32); 64],
    pub game_textures: GameTextures,
    pub board: Board,
}

impl PieceSpawner{
    pub fn new(game_textures: GameTextures, xy_positions: [(f32, f32); 64], board: Board) -> Self{
        return Self { 
            game_textures: game_textures, 
            xy_positions: xy_positions,
            board: board,
        }
    }
    pub fn spawn_pieces(&self, commands: &mut Commands) {

        let mut square_number = 0;
        for piece in self.board.squares{
            let is_white: bool = (0b00001000 & piece) == 0b00001000;
            match piece & 0b00000111{
                1 => self.spawn_pawn(commands, square_number, is_white),
                2 => self.spawn_bishop(commands, square_number, is_white ),
                3 => self.spawn_knight(commands, square_number, is_white ),
                5 => self.spawn_rook(commands, square_number, is_white ),
                6 => self.spawn_queen(commands, square_number, is_white ),
                7 => self.spawn_king(commands, square_number, is_white ),
                _ => (),
            }
            square_number += 1;
  
         
        }
    }
    // make game textures and square positions in struct
    pub fn spawn_pawn(&self, commands: &mut Commands, square_number: u32, is_white: bool) {
        let (x,y) = self.xy_positions[square_number as usize];
        let pawn_texture = if is_white{
            &self.game_textures.w_p
        } else{
            &self.game_textures.b_p
        };
        commands.spawn(SpriteBundle{
            texture: pawn_texture.clone(),
            transform: Transform{
                scale: Vec3::new(self.game_textures.piece_size, self.game_textures.piece_size, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..default()
            },
            ..default()
        }).insert(Piece{square_pos_number: square_number});
    }

    fn spawn_bishop(&self, commands: &mut Commands, square_number: u32, is_white: bool) {
        let (x,y) = self.xy_positions[square_number as usize];
        let pawn_texture = if is_white{
            &self.game_textures.w_b
        } else{
            &self.game_textures.b_b
        };
        commands.spawn(SpriteBundle{
            texture: pawn_texture.clone(),
            transform: Transform{
                scale: Vec3::new(self.game_textures.piece_size, self.game_textures.piece_size, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..default()
            },
            ..default()
        }).insert(Piece{square_pos_number: square_number});
    } 

    fn spawn_knight(&self, commands: &mut Commands, square_number: u32, is_white: bool) {
        let (x,y) = self.xy_positions[square_number as usize];
        let pawn_texture = if is_white{
            &self.game_textures.w_n
        } else{
            &self.game_textures.b_n
        };
        commands.spawn(SpriteBundle{
            texture: pawn_texture.clone(),
            transform: Transform{
                scale: Vec3::new(self.game_textures.piece_size, self.game_textures.piece_size, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..default()
            },
            ..default()
        }).insert(Piece{square_pos_number: square_number});
    }

    fn spawn_rook(&self, commands: &mut Commands, square_number: u32, is_white: bool) {
        let (x,y) = self.xy_positions[square_number as usize];
        let pawn_texture = if is_white{
            &self.game_textures.w_r
        } else{
            &self.game_textures.b_r
        };
        commands.spawn(SpriteBundle{
            texture: pawn_texture.clone(),
            transform: Transform{
                scale: Vec3::new(self.game_textures.piece_size, self.game_textures.piece_size, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..default()
            },
            ..default()
        }).insert(Piece{square_pos_number: square_number});
    }

    fn spawn_queen(&self, commands: &mut Commands, square_number: u32, is_white: bool){
        let (x,y) = self.xy_positions[square_number as usize];
        let pawn_texture = if is_white{
            &self.game_textures.w_q
        } else{
            &self.game_textures.b_q
        };
        commands.spawn(SpriteBundle{
            texture: pawn_texture.clone(),
            transform: Transform{
                scale: Vec3::new(self.game_textures.piece_size, self.game_textures.piece_size, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..default()
            },
            ..default()
        }).insert(Piece{square_pos_number: square_number});
    }

    fn spawn_king(&self, commands: &mut Commands, square_number: u32, is_white: bool){
        let (x,y) = self.xy_positions[square_number as usize];
        let pawn_texture = if is_white{
            &self.game_textures.w_k
        } else{
            &self.game_textures.b_k
        };
        commands.spawn(SpriteBundle{
            texture: pawn_texture.clone(),
            transform: Transform{
                scale: Vec3::new(self.game_textures.piece_size, self.game_textures.piece_size, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..default()
            },
            ..default()
        }).insert(Piece{square_pos_number: square_number});
    }
}


