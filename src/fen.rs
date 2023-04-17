use std::collections::HashMap;
#[derive(Clone, Copy, Debug)]

pub enum PieceType {
    Pawn, Bishop, Knight, Rook, Queen, King, None
}

pub struct Fen{
    pub fen_string: String,
    pub white_piece_square_pos: [PieceType; 65],
    pub black_piece_square_pos: [PieceType; 65],
}

impl Fen{
    
    pub fn new(fen_string: String) -> Fen{
        
        let (white_pieces, black_pieces) = Self::extract_pieces_from_fen(&fen_string);
        return Fen{
            fen_string: fen_string,
            white_piece_square_pos: white_pieces,
            black_piece_square_pos: black_pieces,
        }
    }

    fn extract_pieces_from_fen(fen_string: &String) -> ([PieceType; 65],[PieceType ; 65]) {
        let mut piece_map: HashMap<char, PieceType> = vec![
            ('p', PieceType::Pawn),('b', PieceType::Bishop), 
            ('n', PieceType::Knight),('r', PieceType::Rook),
            ('q', PieceType::Queen),('k', PieceType::King),
        ].into_iter().collect();
        let mut white_piece_square_pos: [PieceType; 65] = [PieceType::None; 65];
        let mut black_piece_square_pos: [PieceType; 65] = [PieceType::None; 65];
        let mut square_number = 1;
        for fen_piece in fen_string.chars() {
            match fen_piece{
                '1'..='8' => {
                    square_number += fen_piece.to_digit(10).unwrap();
                }
                '/' => continue,
                _ => {
                    if fen_piece.is_uppercase(){
                        let fen_piece =fen_piece.to_ascii_lowercase();
                        white_piece_square_pos[square_number as usize] = *piece_map.get(&fen_piece).unwrap();

                    }else{
                        black_piece_square_pos[square_number as usize] = *piece_map.get(&fen_piece).unwrap();
                    }   
                    square_number += 1
                }
            }

        }
        return (white_piece_square_pos,black_piece_square_pos);
    }

}

