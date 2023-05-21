use crate::{board::Board, legal_move_generator::InRangeI32};


pub struct AttackData {
    pub pinned_bitmap: u64,
    pub in_check: bool,
}

impl AttackData{
    pub fn new() -> Self {
        return AttackData {
            pinned_bitmap: 0,
            in_check: false,
        }
    }

    pub fn calculate_attack_data(&mut self, board: &Board, friendly_color: u8) {

        let mut pin_bitmap: u64 = 0;
        let mut in_check: bool = false;
        let king_square_number = Self::find_friendly_king(board, friendly_color);
        let king_rank: i32 = (king_square_number / 8 + 1) as i32;
        let king_file: i32 = (king_square_number % 8 + 1) as i32;
        let king_vulnerability_directions: [(i32,i32);8] = [(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)];
        for (rank_dir, file_dir) in king_vulnerability_directions {
            let mut i: i32 = 1;
            while (king_rank + i*rank_dir).in_range(1, 8) && (king_file + i * file_dir).in_range(1, 8) {

                let square_number: i32 = ((king_rank + i * rank_dir - 1) * 8) + (king_file + i * file_dir - 1);
                if (board.squares[square_number as usize] & 0b00000111) != 0 {
                    let piece = board.squares[square_number as usize];
                    let piece_color = piece & 0b00011000;
                    if piece_color == friendly_color {
                        pin_bitmap |= 0x01 << square_number;
                        break;
                    }else {
                        let piece_type = piece & 0b00000111;
                        match piece_type {
                            2 | 5 | 6 => {
                                in_check = true;
                            }
                            _ => (),
                        }
                    }
                }
                i += 1;
            }
        }
        println!("{}",in_check);
    }       

    fn find_friendly_king(board: &Board, friendly_color: u8) -> u32 {
        let king_square = board.squares.iter().position(|&piece| (piece & 0b00011000) == friendly_color && (piece & 0b00000111) == 7);
        match king_square {
            Some(king_square) => return king_square as u32,
            None => panic!("A king per side must always be alive"),
        }
    }
}