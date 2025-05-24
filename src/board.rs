use std::collections::HashMap;
use regex::Regex;
//use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::pieces::PieceTypeData;
use crate::pieces::BasicPieceType;
use crate::occupied_squares::{ square_to_bit, bit_to_square, print_ray_string, generate_ray_path };
use crate::compass_groups::{ Direction, VERTICALS };

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

    pub fn build_all_xchngrs (&mut self) -> () {
        println!("DEBUG: Entering build_all_xchngrs");
        // First collect all the paths and directions we need to process
        let mut updates: Vec<(String, Direction, String)> = Vec::new();
        
        println!("DEBUG: Starting piece iteration");
        for (square, piece) in &self.pieces {
            println!("DEBUG: Processing piece at square {}", square);
            let piece_type_char = piece.get_piece_type_as_char();
            if let Some(piece_type_ref) = PieceType::get_piece_type_data(piece_type_char) {
                let data: &'static PieceTypeData = piece_type_ref.get_data();
                println!("Key: {}, Value: {:?}, sliding: {}, side: {:?}", square, piece_type_char, data.sliding, data.side);

                for drctn in Direction::iter() {
                    let path = generate_ray_path(square, drctn, self.occupied);
                    if path != "" {
                        let xchngr_str = Board::extract_pid_seq(self, &path, drctn);
                        updates.push((square.clone(), drctn, xchngr_str));
                    }
                }
            }
        }

        // Now apply all the updates
        for (square, drctn, xchngr_str) in updates {
            if let Some(piece) = self.pieces.get_mut(&square) {
                piece.exchangers.insert(drctn, xchngr_str.clone());
                if xchngr_str != "" {
                    println!("Exchangers: {}, {:?}: {}", square, drctn, &xchngr_str);

                    let xv = piece.exchangers.get(&drctn);
                    if let Some(v) = xv {
                        println!("{}, {:?}: {}", square, drctn, v);
                    } else {
                        println!("no exchanges on {} for direction {:?}", square, drctn);
                    }
                }
            }
        }
    }

    fn extract_pid_seq(&self, sqid_seq: &str, drctn: Direction) -> String {
        let odrctn = drctn.opposite();
        let _re = Regex::new(r"(_)?([a-h][1-8])").unwrap();
        let mut pids: String = "".to_string();
        let sqid_seq_len = sqid_seq.len();
        let mut llmt;
        let mut ulmt;
        let mut sliding_only = false;

        // println!("extract_sqid_seq with s: {}", sqid_seq);

        if sqid_seq_len > 0 {
            if sqid_seq.starts_with("_") {
            // underscore in the first place -> if first occupied sqid can exchange it must be a slider
                // println!("sqid_seq: {sqid_seq}, starts_with underscore/");
                sliding_only = true;
                llmt = 1;
                ulmt = 3;
            } else {
                llmt = 0;
                ulmt = 2;
            }

            while ulmt <= sqid_seq_len {
                let sq = &sqid_seq[llmt..ulmt];
                let piece = &self.pieces.get(sq);
                match piece {
                    Some(piece) =>{
                        let piece_type_char = piece.get_piece_type_as_char();
                        if let Some(piece_type_ref) = PieceType::get_piece_type_data(piece_type_char) {
                            let data: &'static PieceTypeData = piece_type_ref.get_data();
                            let pid = piece.get_pid();
                            // let pt = PieceType::from_char(piece_type_char);
                            let basic_piece_type = BasicPieceType::from_char(piece_type_char);
                            let pchar = piece.get_piece_type_as_char();

                            if sliding_only == false {
                                // first piece encountered only one step away, to allow single step pieces
                                if !data.directions.contains(&odrctn) {
                                    return pids;
                                } else if basic_piece_type == Some(BasicPieceType::Pawn) && VERTICALS.contains(&drctn) {
                                    return pids;
                                }
                            } else { //sliding only
                                if !data.sliding || !data.directions.contains(&odrctn) {
                                    return pids;
                                }
                            }
                            
                            pids.push_str(&pid);
                            sliding_only = true;
                            llmt += 2;
                            ulmt += 2;            
                        }
                    }
                    None => {
                        println!("No matching piece");
                    }
                }
            }
        }
        pids
    }

    // fn extract_xchngrs (&self, ss: &str) -> () {
    //     // let mut path = String::new();

    //     println!("exs on ray: {:?}", ss);
    //     // path.push('z')
    // }

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

    pub fn initialise_custom(&mut self) {
        self.create_and_place_piece("a8R");
        self.create_and_place_piece("b8n");
        self.create_and_place_piece("d8k");
        self.create_and_place_piece("d7r");
        self.create_and_place_piece("f7p");
        self.create_and_place_piece("f6n");
        self.create_and_place_piece("d5q");
        self.create_and_place_piece("e5p");
        self.create_and_place_piece("g5p");
        self.create_and_place_piece("c4N");
        self.create_and_place_piece("d4P");
        self.create_and_place_piece("f4N");
        self.create_and_place_piece("b3P");
        self.create_and_place_piece("d3r");
        self.create_and_place_piece("f3p");
        self.create_and_place_piece("h3K");
        self.create_and_place_piece("a2B");        
        self.create_and_place_piece("e2P");        
    }

    pub fn initialise_standard(&mut self) {
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