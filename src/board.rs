use std::collections::HashMap;
use regex::Regex;
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::pieces::{Side, PieceTypeData};
use crate::occupied_squares::{ square_to_bit, bit_to_square, print_ray_string, generate_ray_path };
use crate::compass_groups::{ DirectionNumber, VERTICALS };

// #[derive(Debug)]
pub struct Board {
    pieces: HashMap<String, Piece>,
    occupied: u64
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: HashMap::new(),
            occupied: 0,
        }
    }

    pub fn create_and_place_piece(&mut self, piece_identifier: &str) {
        if let Some(piece) = Piece::new(&piece_identifier) {
            let square = &piece_identifier[0..2].to_string();
            self.pieces.insert(square.clone(), piece);
            
            if let Some(bit) = square_to_bit(&square) {
                self.occupied |= 1u64 << bit;
            }        
        } else {
            println!("Unable to create piece with identifier: {}.", &piece_identifier.to_string());
        }
    }

    pub fn build_all_xchngrs (&self) -> () {
        let
            d = 0;
        for (square, piece) in &self.pieces {
            let piece_type_char = piece.get_piece_type_as_char();
            match PieceType::get_piece_type_data(piece_type_char) {
                Some(piece_type_ref) => { // piece_type_ref is &'static PieceType
                    let data: &'static PieceTypeData = piece_type_ref.get_data();
                    println!("Key: {}, Value: {:?}, sliding: {}, side: {:?}", square, piece_type_char, data.sliding, data.side);
                    let path = generate_ray_path(square, d, self.occupied);
                    print_ray_string(square, 8, &path);
                    let xchngr_str = Board::extract_sqid_seq(&self, &path, d);
                    println!("xchngr_str: {}", xchngr_str); 
                }
                None => {
                    println!("Character '{}' did not match a piece type.", piece_type_char);
                    // Handle invalid piece character if necessary
                }
            }
        }
    }

    fn extract_sqid_seq(&self, sqid_seq: &str, d: DirectionNumber) -> String {
        let od = if d < 8 { d + 8 } else { d - 8 };
        let _re = Regex::new(r"(_)?([a-h][1-8])").unwrap();
        let mut sqids: String = "".to_string();
        let sqid_seq_len = sqid_seq.len();
        let mut llmt;
        let mut ulmt;
        let mut sliding_only = false;

        println!("extract_sqid_seq with s: {}", sqid_seq);

        if sqid_seq_len > 0 {
            if sqid_seq.starts_with("_") {
            // underscore in the first place -> if first occupied sqid can exchange it must be a slider
                println!("sqid_seq: {sqid_seq}, starts_with underscore/");
                sliding_only = true;
                llmt = 1;
                ulmt = 3;
            } else {
                llmt = 0;
                ulmt = 2;
            }

            while ulmt < sqid_seq_len {
                let sq = &sqid_seq[llmt..ulmt];
                let piece = &self.pieces.get(sq);
                match piece {
                    Some(piece) =>{
                        let piece_type_char = piece.get_piece_type_as_char();
                        if let Some(piece_type_ref) = PieceType::get_piece_type_data(piece_type_char) {
                            let data: &'static PieceTypeData = piece_type_ref.get_data();
                            let pid = piece.get_pid();
                            // let ptype = piece.get_piece_fen();
                            let pchar = piece.get_piece_type_as_char();
                            if llmt == 0 && data.directions.contains(&od) {
                                // the piece mayn be sliding or non-sliding because it only one step away from origin
                                // if let Some(pchar) = ptype.chars().next() {
                                    if pchar == 'p' || pchar == 'P' { // PAWN
                                        if VERTICALS.contains(&od) {
                                            return sqids;
                                        } 
                                    }
                                // }
                            } else if !data.sliding {
                                return sqids;
                            } else if !data.directions.contains(&od) {
                                return sqids;
                            }
                
                            sqids.push_str(&pid);
                            llmt += 2;
                            ulmt += 2;            
                        }


                        // let pid = piece.get_pid();
                        // let ptype = piece.get_piece_fen();
                        // if llmt == 0 && piece.get_legal_directions().contains(&od) {
                        //     // the piece mayn be sliding or non-sliding because it only one step away from origin
                        //     if let Some(pchar) = ptype.chars().next() {
                        //         if pchar == 'p' || pchar == 'P' { // PAWN
                        //             if VERTICALS.contains(&od) {
                        //                 return sqids;
                        //             } 
                        //         }
                        //     }
                        // } else if !piece.is_sliding() {
                        //     return sqids;
                        // } else if !piece.get_legal_directions().contains(&od) {
                        //     return sqids;
                        // }
            
                        // sqids.push_str(&pid);
                        // llmt += 2;
                        // ulmt += 2;
                    }
                    None => {
                        println!("No matching piece");
                    }
                }
            }
        }
        sqids
    }

    fn extract_xchngrs (&self, ss: &str) -> () {
        // let mut path = String::new();

        println!("exs on ray: {:?}", ss);
        // path.push('z')
    }

    pub fn get_piece_on(&self, square: &str) -> Option<&Piece> {
        self.pieces.get(square)
    }

    pub fn remove_piece_from(&mut self, square: &str) {
        let piece = self.pieces.remove(square);
        
        // Update bitboard
        if piece.is_some() {
            if let Some(bit) = square_to_bit(square) {
                self.occupied &= !(1u64 << bit);
            }
        }
    }

    pub fn is_square_occupied(&self, square: &str) -> bool {
        self.pieces.contains_key(square)
    }

    pub fn iter_pieces(&self) -> std::collections::hash_map::Iter<'_, String, Piece> {
        self.pieces.iter()
    }

    pub fn clear(&mut self) {
        self.pieces.clear();
        self.occupied = 0;
    }

    pub fn len(&self) -> usize {
        self.pieces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }

    pub fn initialize_standard(&mut self) {
        // White pieces
        self.create_and_place_piece("a1R");
        self.create_and_place_piece("b1N");
        self.create_and_place_piece("c1B");
        self.create_and_place_piece("d1Q");
        self.create_and_place_piece("e1K");
        self.create_and_place_piece("f1B");
        self.create_and_place_piece("g1N");
        self.create_and_place_piece("h1R");
        self.create_and_place_piece("a2P");
        self.create_and_place_piece("b2P");
        self.create_and_place_piece("c2P");
        self.create_and_place_piece("d2P");
        self.create_and_place_piece("e2P");
        self.create_and_place_piece("f2P");
        self.create_and_place_piece("g2P");
        self.create_and_place_piece("h2P");

        // Black pieces
        self.create_and_place_piece("a8r");
        self.create_and_place_piece("b8n");
        self.create_and_place_piece("c8b");
        self.create_and_place_piece("d8q");
        self.create_and_place_piece("e8k");
        self.create_and_place_piece("f8b");
        self.create_and_place_piece("g8n");
        self.create_and_place_piece("h8r");
        self.create_and_place_piece("a7p");
        self.create_and_place_piece("b7p");
        self.create_and_place_piece("c7p");
        self.create_and_place_piece("d7p");
        self.create_and_place_piece("e7p");
        self.create_and_place_piece("f7p");
        self.create_and_place_piece("g7p");
        self.create_and_place_piece("h7p");
    }

    // New method to get the bitboard
    pub fn get_occupied_bitboard(&self) -> u64 {
        self.occupied
    }

    // New method to print occupied squares
    pub fn print_occupied_squares(&self) {
        println!("Occupied squares (from bitboard):");
        for bit in 0..64 {
            if (self.occupied & (1u64 << bit)) != 0 {
                if let Some(square) = bit_to_square(bit) {
                    println!("- {}", square);
                }
            }
        }
    }
} 