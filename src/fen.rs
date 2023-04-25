use std::collections::HashMap;

use crate::{PieceType, GameTextures};


pub fn extract_pieces_from_fen(fen_string: &String) -> [u8; 64] {
    let piece_map: HashMap<char, PieceType> = vec![
        ('p', PieceType::Pawn),('b', PieceType::Bishop), 
        ('n', PieceType::Knight),('r', PieceType::Rook),
        ('q', PieceType::Queen),('k', PieceType::King),
    ].into_iter().collect();

    let reversed_fen_string: Vec<&str> = fen_string.split('/').rev().collect();
    
    let mut pieces: [u8; 64] = [0; 64];
    let mut square_number = 0;
    for fen_row in reversed_fen_string{
        for fen_piece in fen_row.chars() {
            match fen_piece{
                '1'..='8' => {
                    square_number += fen_piece.to_digit(10).unwrap();
                }
                '/' => continue,
                fen_piece if fen_piece.is_uppercase() => {
                    let fen_piece =fen_piece.to_ascii_lowercase();
                    pieces[square_number as usize] = *piece_map.get(&fen_piece).unwrap() as u8 | 0b00001000 ;
                    square_number += 1;
                }
                _ => {
                    pieces[square_number as usize] = *piece_map.get(&fen_piece).unwrap() as u8;
                    square_number += 1
                }
            }
        }
    }
    return pieces;
}


