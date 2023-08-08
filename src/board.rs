use crate::chess_utility::SideColor;

#[derive(Clone)]
pub struct Board{
    pub squares: [u8; 64],
}

impl Board{
    pub fn piece_color_to_side_color(&self, square: u32) -> SideColor{
        match self.squares[square as usize] & 0b00011000 {
            0b00001000 => return SideColor::White,
            0b00010000 => return SideColor::Black,
            _ => panic!("Must be piece if trying to get color of piece"),
        }
    }

    pub fn find_friendly_king(&self, friendly_color: u8) -> u32 {
        let king_square = self.squares.iter().position(|&piece| (piece & 0b00011000) == friendly_color && (piece & 0b00000111) == 7);
        match king_square {
            Some(king_square) => return king_square as u32,
            None => panic!("One king per side must always be alive"),
        }
    }
}