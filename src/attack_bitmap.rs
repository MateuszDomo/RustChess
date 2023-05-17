use crate::{board::Board, legal_move_generator::InRangeI32};


pub struct AttackMap {
    pub attack_bitmap: u64,
}

impl AttackMap {

    pub fn new(board: &Board, friendly_color: u8) -> Self {
        let mut sliding_pieces_bitmap: u64 = 0;
        let mut pawn_pieces_bitmap: u64 = 0;
        for i in 0..64 {
            let piece = board.squares[i as usize];
            if piece & 0b00011000 == friendly_color {
                continue
            }
            match piece & 0b00000111 {
                1 => {
                    pawn_pieces_bitmap |= Self::generate_pawn_pieces_bitmap(piece, i / 8 + 1, i % 8 + 1, board)
                }
                2 | 5 | 6 => {
                    sliding_pieces_bitmap |= Self::generate_sliding_pieces_bitmap(piece, i / 8 + 1, i % 8 + 1, board);
                }
                _ => continue,
            }
        }
        print_bitmap(pawn_pieces_bitmap);
        print_bitmap(sliding_pieces_bitmap);
        return AttackMap {attack_bitmap: sliding_pieces_bitmap | pawn_pieces_bitmap};
    }

    fn generate_sliding_pieces_bitmap(piece: u8, starting_rank: i32, starting_file: i32, board: &Board) -> u64 {
        let piece_type = piece & 0b00000111;
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
                let square_number = ((starting_rank + i * rank_dir - 1) * 8) + (starting_file + i * file_dir - 1);
                sliding_pieces_bitmap |= 0x01 << square_number;
                if board.squares[square_number as usize] != 0 {
                    break;
                }
                i += 1;
            }
        }
        return sliding_pieces_bitmap;
    }
    fn generate_pawn_pieces_bitmap(piece: u8, starting_rank: i32, starting_file: i32, board: &Board) -> u64 {
        let attacking_direction = if (piece & 0b0001100) == 0b00001000 { 1 } else { -1 };
        let mut pawn_pieces_bitmap: u64 = 0;

        let new_attacking_rank = starting_rank + attacking_direction;
        // Left diagonal capture
        let left_attacking_file = starting_file - 1;
        if new_attacking_rank.in_range(1, 8) && left_attacking_file.in_range(1, 8) {
            let square_number = ((new_attacking_rank - 1) * 8) + (left_attacking_file - 1);
            pawn_pieces_bitmap |= 0x01 << square_number;
        }

        // Right diagonal capture
        let right_attacking_file = starting_file + 1;
        if new_attacking_rank.in_range(1, 8) && right_attacking_file.in_range(1, 8) {
            let square_number = ((new_attacking_rank - 1) * 8) + (right_attacking_file - 1);
            pawn_pieces_bitmap |= 0x01 << square_number;
        }
        return pawn_pieces_bitmap;
    }
    fn generate_leaping_pieces_bitmap(piece: u8, starting_rank: i32, starting_file: i32, board: &Board) {
        let piece_type = piece & 0b00000111;
        let attacking_directions: Vec<(i32,i32)> = if piece_type == 1 {
            vec![(1, -1), ]
        } else if piece_type == 5 {
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
        } else if piece_type == 6 {
            vec![(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)]
        } else {
            vec![]
        };
    }
}



fn print_bitmap(value: u64) {
    let number = format!("{:064b}", value);
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            let mask = 1 << index;
            let piece = (value & mask) >> index;

            print!("{} ", piece);
        }
        println!();
    }
    println!()
}