use std::collections::HashMap;

use crate::{chess_utility::SideColor, game_state::CastlingRights};

pub fn extract_game_state_from_fen(fen_string: &String) -> ([u8; 64], SideColor, CastlingRights, Option<u32>) {
    let piece_map: HashMap<char, u8> = vec![
        ('p', 1),('b', 2), 
        ('n', 3),('r', 5),
        ('q', 6),('k', 7),
    ].into_iter().collect();
    let fen_components: Vec<&str> = fen_string.split(' ').collect();
    let reversed_fen_string: Vec<&str> = fen_components[0].split('/').rev().collect();
    
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
                    let fen_piece: char = fen_piece.to_ascii_lowercase();
                    pieces[square_number as usize] = *piece_map.get(&fen_piece).unwrap() as u8 | 0b00001000 ;
                    square_number += 1;
                }
                _ => {
                    pieces[square_number as usize] = *piece_map.get(&fen_piece).unwrap() as u8 | 0b00010000;
                    square_number += 1
                }
            }
        }
    }

    let side_color: SideColor = match fen_components[1] {
       "w" => SideColor::White,
       "b" => SideColor::Black,
       _ => panic!("wrong fen notation for active color")
    };

    let mut castling_rights: CastlingRights = CastlingRights::new();
    for castling_info in fen_components[2].chars() {
        match castling_info {
            'K' => castling_rights.w_short = true,
            'Q' => castling_rights.w_long = true,
            'k' => castling_rights.b_short = true,
            'q' => castling_rights.b_long = true,
            '-' => (),
            _ => panic!("wrong fen notation for castling rights")
        }
    }

    let enpassant_string_bytes = fen_components[3].as_bytes();

    let enpassant_string_bytes_length = enpassant_string_bytes.len();
    if enpassant_string_bytes_length > 2 || enpassant_string_bytes_length < 1{
        panic!("wrong notation for possible enpassant targets");
    }

    let enpassant_square: Option<u32>;
    if enpassant_string_bytes_length == 1 {
        if enpassant_string_bytes[0] as char != '-' {
            panic!("wrong notation for possible enpassant targets");
        }

        enpassant_square = None;
    } else {
        let enpassant_file: u32 = enpassant_string_bytes[0] as u32;
        let enpassant_rank: u32 = (enpassant_string_bytes[1] as char).to_digit(10).unwrap_or_else(||panic!("hey"));
        if !(enpassant_file >= 97 && enpassant_file <= 104) || !(enpassant_rank >= 1 && enpassant_rank <= 8){
            panic!("wrong notation for possible enpassant targets");
        }

        enpassant_square = Some((enpassant_rank - 1) * 8 + (enpassant_file - 1));
    }
    
    return (pieces, side_color, castling_rights, enpassant_square);
}



