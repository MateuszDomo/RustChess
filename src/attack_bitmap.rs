use crate::{board::Board, legal_move_generator::InRangeI32};


pub struct AttackBitmap {
    pub attack_bitmap: u64,
}

impl AttackBitmap {

    pub fn new(board: &Board, friendly_color: u8) -> Self {
        let mut sliding_pieces_bitmap: u64 = 0;
        let mut pawn_pieces_bitmap: u64 = 0;
        let mut knight_pieces_bitmap: u64 = 0;
        let mut king_pieces_bitmap: u64 = 0;
        for i in 0..64 {
            let piece: u8 = board.squares[i as usize];
            if piece & 0b00011000 == friendly_color {
                continue
            }
            match piece & 0b00000111 {
                1 => {
                    pawn_pieces_bitmap |= Self::generate_pawn_pieces_bitmap(piece, i / 8 + 1, i % 8 + 1)
                }
                2 | 5 | 6 => {
                    sliding_pieces_bitmap |= Self::generate_sliding_pieces_bitmap(piece, i / 8 + 1, i % 8 + 1, board);
                }
                3 => {
                    knight_pieces_bitmap |= Self::generate_knight_pieces_bitmap(i / 8 + 1, i % 8 + 1);
                }
                7 => {
                    king_pieces_bitmap |= Self::generate_king_piece_bitmap(i / 8 + 1, i % 8 + 1);
                }
                _ => continue,
            }
        }
        return AttackBitmap {attack_bitmap: sliding_pieces_bitmap | pawn_pieces_bitmap | knight_pieces_bitmap | king_pieces_bitmap};
    }

    fn generate_sliding_pieces_bitmap(piece: u8, starting_rank: i32, starting_file: i32, board: &Board) -> u64 {
        let piece_type: u8 = piece & 0b00000111;
        let attacking_directions: Vec<(i32,i32)> = if piece_type == 2 {
            vec![(1, -1), (1, 1), (-1, -1), (-1, 1)]
        } else if piece_type == 5 {
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
        } else if piece_type == 6 {
            vec![(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)]
        } else {
            vec![]
        };
        let mut sliding_pieces_bitmap: u64 = 0;
        for (rank_dir,file_dir) in attacking_directions {
            let mut i: i32 = 1; 
            while (starting_rank + i * rank_dir).in_range(1, 8) && (starting_file + i * file_dir).in_range(1, 8) {
                let square_number: i32  = ((starting_rank + i * rank_dir - 1) * 8) + (starting_file + i * file_dir - 1);
                sliding_pieces_bitmap |= 0x01 << square_number;
                if board.squares[square_number as usize] != 0 {
                    break;
                }
                i += 1;
            }
        }
        return sliding_pieces_bitmap;
    }
    fn generate_pawn_pieces_bitmap(piece: u8, starting_rank: i32, starting_file: i32) -> u64 {
        let attacking_direction: i32 = if (piece & 0b0001100) == 0b00001000 { 1 } else { -1 };
        let mut pawn_pieces_bitmap: u64 = 0;

        let new_attacking_rank: i32 = starting_rank + attacking_direction;
        // Left diagonal capture
        let left_attacking_file: i32 = starting_file - 1;
        if new_attacking_rank.in_range(1, 8) && left_attacking_file.in_range(1, 8) {
            let square_number: i32 = ((new_attacking_rank - 1) * 8) + (left_attacking_file - 1);
            pawn_pieces_bitmap |= 0x01 << square_number;
        }

        // Right diagonal capture
        let right_attacking_file: i32 = starting_file + 1;
        if new_attacking_rank.in_range(1, 8) && right_attacking_file.in_range(1, 8) {
            let square_number: i32 = ((new_attacking_rank - 1) * 8) + (right_attacking_file - 1);
            pawn_pieces_bitmap |= 0x01 << square_number;
        }
        return pawn_pieces_bitmap;
    }

    fn generate_knight_pieces_bitmap(starting_rank: i32, starting_file: i32) -> u64 {
        let attacking_directions: [(i32,i32);8] =  [(2,-1), (2,1), (-2,1), (-2,-1), (1,2), (1,-2), (-1,2), (-1,-2)];
        let mut knight_piece_bitmap: u64 = 0;
        for (rank_dir, file_dir) in attacking_directions {
            let new_attacking_rank: i32 = starting_rank + rank_dir;
            let new_attacking_file: i32 = starting_file + file_dir;
            if new_attacking_rank.in_range(1, 8) && new_attacking_file.in_range(1, 8) {
                let square_number: i32 = ((new_attacking_rank - 1) * 8) + (new_attacking_file - 1);
                knight_piece_bitmap |= 0x01 << square_number;
            }
        }
        return knight_piece_bitmap;
    }

    fn generate_king_piece_bitmap(starting_rank: i32, starting_file: i32) -> u64 {
        let attacking_directions: [(i32,i32);8] = [(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)];  
        let mut king_piece_bitmap: u64 = 0;
        for (rank_dir, file_dir) in attacking_directions {
            let new_attacking_rank: i32 = starting_rank + rank_dir;
            let new_attacking_file: i32 = starting_file + file_dir;
            if new_attacking_rank.in_range(1, 8) && new_attacking_file.in_range(1, 8) {
                let square_number: i32 = ((new_attacking_rank - 1) * 8) + (new_attacking_file - 1);
                king_piece_bitmap |= 0x01 << square_number;
            }
        }
        return king_piece_bitmap;
    }

    pub fn print_bitmap(&self) {
        let value: u64 = self.attack_bitmap;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index: i32 = rank * 8 + file;
                let mask: u64 = 1 << index;
                let piece: u64 = (value & mask) >> index;

                print!("{} ", piece);
            }
            println!();
        }
      println!()
    }
    
    pub fn is_square_being_attacked(&self, target_square: u32) -> bool {
        let target_square_bit_mask: u64 = 0x01 << target_square;
        return self.attack_bitmap & target_square_bit_mask != 0x0;
    }
}


