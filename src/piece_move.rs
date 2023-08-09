pub enum Flag {None}

impl From<Flag> for u16 {
    fn from(value: Flag) -> Self {
        match value {
            Flag::None => 0b0000
        }
    }
}

pub struct PieceMove {
    move_bits: u16,
}

impl PieceMove {
    pub fn new(starting_square: u32, target_square: u32, flag: u16) -> Self {
        return PieceMove {
            move_bits: starting_square as u16 | (target_square as u16) << 6 | (flag as u16) << 12
        }
    }

    pub fn starting_square(&self) -> u32 {
        let starting_square_mask: u16 = 0b0000000000111111;
        return (self.move_bits & starting_square_mask) as u32;
    }
    
    pub fn target_square(&self) -> u32 {
        let target_square_mask: u16 = 0b0000111111000000;
        return ((self.move_bits & target_square_mask) >> 6) as u32;
    }

    pub fn flag(&self) -> u16 {
        let flag_mask: u16 = 0b1111000000000000;
        return (self.move_bits & flag_mask) >> 12;
    }
}

