use crate::{board::Board, legal_move_generator::InRangeI32, attack_bitmap::AttackBitmap,chess_utility::{print_bitmap, SideColor}};


pub struct AttackData {
    pub pinned_bitmap: u64,
    pub pinned_ray_bitmap: u64,
    pub in_check: bool,
    pub in_double_check: bool,
    pub check_ray_bitmap: u64,
    pub attack_bitmaps: AttackBitmap,
}

impl AttackData {
    pub fn new(board: &Board, friendly_color: u8) -> Self {
        let attack_bitmaps = AttackBitmap::new(board,friendly_color);
        return AttackData {
            pinned_bitmap: 0,
            pinned_ray_bitmap: 0,
            in_check: false,
            in_double_check: false,
            check_ray_bitmap: 0,
            attack_bitmaps: attack_bitmaps,
        }
    }

    pub fn calculate_attack_data(&mut self, board: &Board, friendly_color: u8) {
        
        let king_square_number = board.find_friendly_king(friendly_color);
        let king_rank: i32 = (king_square_number / 8 + 1) as i32;
        let king_file: i32 = (king_square_number % 8 + 1) as i32;
        let king_vulnerability_directions: [(i32,i32);8] = [(1, -1), (1, 1), (-1, -1), (-1, 1), (1, 0), (-1, 0), (0, 1), (0, -1)];
        for (rank_dir, file_dir) in king_vulnerability_directions {

            let mut ray_mask: u64 = 0;
            let mut i: i32 = 1;
            let mut is_friendly_piece_in_ray_mask: bool = false;
            while (king_rank + i*rank_dir).in_range(1, 8) && (king_file + i * file_dir).in_range(1, 8) {
                let square_number: i32 = ((king_rank + i * rank_dir - 1) * 8) + (king_file + i * file_dir - 1);
                ray_mask |= 0x01 << square_number;
                if (board.squares[square_number as usize] & 0b00000111) != 0 {
                    let piece = board.squares[square_number as usize];
                    let piece_color = piece & 0b00011000;

                    if piece_color == friendly_color {
                        is_friendly_piece_in_ray_mask = true;
                    }else {
                        let piece_type = piece & 0b00000111;
                        match piece_type {
                            2 | 5 | 6 => {
                                if (piece_type == 2 && !Self::is_diagonal(rank_dir, file_dir)) || (piece_type == 5 && Self::is_diagonal(rank_dir, file_dir)){
                                    break;
                                }
                                if is_friendly_piece_in_ray_mask == true {
                                    self.pinned_ray_bitmap |= ray_mask;
                                } else {
                                    self.check_ray_bitmap |= ray_mask;
                                    if self.in_check == true {
                                        self.in_double_check = true;
                                    } else {
                                        self.in_check = true;
                                    }
                                }
                                break;
                            }
                            _ => (),
                        }
                    }
                }
                i += 1;
            }
        }

        if self.attack_bitmaps.is_square_being_attacked_by_pawn(king_square_number) {
            if self.in_check == true {
                self.in_double_check = true;
            } else {
                self.in_check = true;
            }
            // Add attacking pawn to check ray bitmap
            let row_dir = if friendly_color == SideColor::White.side_color_to_u8() {1} else {-1}; 
            // Left diagonal pawn threat
            if (king_rank + row_dir).in_range(1, 8) && (king_file - 1).in_range(1, 8) {
                let square_number: i32 = ((king_rank + 1 - 1) * 8) + (king_file - 1 - 1);
                if board.squares[square_number as usize] & 0b00000111 == 1 {
                    self.check_ray_bitmap |= 0x01 << square_number;
                }
            }
            // Left diagonal pawn threat
            if (king_rank + row_dir).in_range(1, 8) && (king_file + 1).in_range(1, 8) {
                let square_number: i32 = ((king_rank + 1 - 1) * 8) + (king_file + 1 - 1);
                if board.squares[square_number as usize] & 0b00000111 == 1 {
                    self.check_ray_bitmap |= 0x01 << square_number;
                }
            }
        }
        if self.attack_bitmaps.is_square_being_attacked_by_horse(king_square_number) {
            // Add attacking horse to check ray bitmap
            if self.in_check == true {
                self.in_double_check = true;
            } else {
                self.in_check = true;
            }
            // Add attacking horse to check ray bitmap
            let knight_directions: [(i32,i32); 8] = [(2,-1), (2,1), (-2,1), (-2,-1), (1,2), (1,-2), (-1,2), (-1,-2)];
            for (rank_dir, file_dir) in knight_directions {
                if (king_rank + rank_dir).in_range(1, 8) && (king_file + file_dir).in_range(1, 8) {
                    let square_number: i32 = ((king_rank + rank_dir - 1) * 8) + (king_file + file_dir - 1);
                    if board.squares[square_number as usize] & 0b00000111 == 3 {
                        self.check_ray_bitmap |= 0x01 << square_number;
                    }
                }
            }
        }
    }       

    fn is_diagonal(rank_dir: i32, file_dir: i32) -> bool {
        return  rank_dir != 0 && file_dir != 0;
    }
    
    pub fn print_bitmap(bitmap: u64) {
        let value: u64 = bitmap;
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

    pub fn is_square_in_pinned_ray(&self, square_number: u32) -> bool {
        
        return (self.pinned_ray_bitmap & (0x01u64 << square_number)) != 0x0;
    }

    pub fn is_square_in_check_ray(&self, square_number: u32) -> bool {
        return (self.check_ray_bitmap & (0x01u64 << square_number)) != 0x0;
    }
}