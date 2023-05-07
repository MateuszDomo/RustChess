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
}