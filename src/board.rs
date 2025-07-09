use crate::compass_groups::{Direction, HALF_WINDS, VERTICALS, get_direction};
use crate::occupied_squares::{bit_to_string_square, generate_ray_path, square_to_bit};
use crate::pieces::BasicPieceType;
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::pieces::PieceTypeData;
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};

// use std::fmt::Display;
use std::str::FromStr;
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Copy, Clone)] // These are useful traits to derive
#[derive(Display, AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Square {
    a8, b8, c8, d8, e8, f8, g8, h8,
    a7, b7, c7, d7, e7, f7, g7, h7,
    a6, b6, c6, d6, e6, f6, g6, h6,
    a5, b5, c5, d5, e5, f5, g5, h5,
    a4, b4, c4, d4, e4, f4, g4, h4,
    a3, b3, c3, d3, e3, f3, g3, h3,
    a2, b2, c2, d2, e2, f2, g2, h2,
    a1, b1, c1, d1, e1, f1, g1, h1,
}



#[derive(Debug,Clone)]
pub struct Board {
    occupied: u64, // representation of pieces as bits in 8 bytes according to piece position
    pieces: HashMap<Square, Piece>,
}

// impl std::fmt::Debug for Board {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Board")
//             .field("occupied", &format!("0x{:x}", self.occupied))
//             .field("piece_count", &self.pieces.len())
//             .finish()
//     }
// }

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: HashMap::new(),
            occupied: 0,
        }
    }

    pub fn create_and_place_piece(&mut self, piece_identifier: &str) {
        if let Some(piece) = Piece::new(piece_identifier) {
            let sq = &piece_identifier[0..2].to_string();
            let square = Square::from_str(sq).unwrap();
            self.pieces.insert(square, piece);

            if let Some(bit) = square_to_bit(square) {
                self.occupied |= 1u64 << bit;
            }
        } else {
            println!(
                "Unable to create piece with identifier: {}.",
                &piece_identifier.to_string()
            );
        }
    }

    pub fn build_all_xchngrs(&mut self) {
        // println!("DEBUG: Entering build_all_xchngrs");
        // First collect all the paths and directions we need to process
        let mut updates: Vec<(Square, Direction, String)> = Vec::new();

        // println!("DEBUG: Starting piece iteration");
        for (square, piece) in &self.pieces {
            // println!("DEBUG: Processing piece at square {square}");
            // let sq_str: &str = square.as_ref();
            let piece_type_char = piece.get_piece_type_as_char();
            if let Some(piece_type_ref) = PieceType::get_piece_type_data(piece_type_char) {
                let data: &'static PieceTypeData = piece_type_ref.get_data();
                for drctn in Direction::iter() {
                    if let Some(ray_path) = generate_ray_path(*square, drctn, self.occupied) {
                        if let Some(xchngrs) = Board::extract_pid_seq(self, data, &ray_path, drctn)
                        {
                            updates.push((*square, drctn, xchngrs));
                        }
                    }
                }
            }
        }

        for (square, drctn, xchngrs) in updates {
            if let Some(piece) = self.pieces.get_mut(&square) {
                if !xchngrs.is_empty() {
                    piece.exchangers.insert(drctn, xchngrs.clone());
                    // println!("Exchangers: {}, {:?}: {}", square, drctn, &xchngrs);
                    let _xv = piece.exchangers.get(&drctn);
                    // if let Some(v) = xv {
                    //     println!("Piece.exchangers: {square}, {drctn:?}: {v}");
                    // } else {
                    //     println!("no exchanges on {square} for direction {drctn:?}: {xv:?}");
                    // }
                } else {
                    // println!("Zero length xchnger_str on {square} for direction {drctn:?}");
                }
            }
        }
    }

    fn extract_pid_seq(
        &self,
        focus_piece_data: &PieceTypeData,
        sqid_seq: &str,
        drctn: Direction,
    ) -> Option<String> {
        let odrctn = drctn.opposite();
        let mut pids = String::new(); // pins will be indicated with:
        // '<' (king and pinned piece same colour)  or '>' (different colours):= pin(?) or skewer(?)
        let sqid_seq_len = sqid_seq.len();
        let mut llmt;
        let mut ulmt;
        let mut sliding_only = false;

        if sqid_seq_len > 0 {
            if sqid_seq.starts_with("_") {
                sliding_only = true;
                llmt = 1;
                ulmt = 3;
            } else {
                llmt = 0;
                ulmt = 2;
            }

            while ulmt <= sqid_seq_len {
                let sq = &sqid_seq[llmt..ulmt];
                let square = Square::from_str(sq).unwrap();
                let piece = &self.pieces.get(&square);
                match piece {
                    Some(piece) => {
                        let piece_type_char = piece.get_piece_type_as_char();
                        if let Some(piece_type_ref) =
                            PieceType::get_piece_type_data(piece_type_char)
                        {
                            let data: &'static PieceTypeData = piece_type_ref.get_data();
                            let pid = piece.get_pid();
                            if !sliding_only {
                                // first piece encountered only one step away, to allow single step pieces
                                if !data.directions.contains(&odrctn) {
                                    if !HALF_WINDS.contains(&drctn)
                                        && focus_piece_data.basic_piece_type == BasicPieceType::King
                                    {
                                        return Board::extract_pin_seq(
                                            self,
                                            focus_piece_data,
                                            sqid_seq,
                                            drctn,
                                        );
                                    } else {
                                        return None;
                                    }
                                // } else if focus_piece_data.basic_piece_type == BasicPieceType::Pawn && VERTICALS.contains(&drctn) {
                                } else if data.basic_piece_type == BasicPieceType::Pawn
                                    && VERTICALS.contains(&drctn)
                                {
                                    return None;
                                }
                            } else {
                                //sliding only
                                if !data.sliding || !data.directions.contains(&odrctn) {
                                    if !pids.is_empty() {
                                        return Some(pids);
                                    } else if !HALF_WINDS.contains(&drctn)
                                        && focus_piece_data.basic_piece_type == BasicPieceType::King
                                    {
                                        return Board::extract_pin_seq(
                                            self,
                                            focus_piece_data,
                                            sqid_seq,
                                            drctn,
                                        );
                                    } else {
                                        return None;
                                    }
                                }
                            }

                            pids.push_str(pid);
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
        Some(pids)
    }

    fn extract_pin_seq(
        &self,
        focus_king_piece_data: &PieceTypeData,
        sqid_seq: &str,
        drctn: Direction,
    ) -> Option<String> {
        let odrctn = drctn.opposite();
        let mut pins = String::new(); // pins will be indicated with '*' in the first place
        let sqid_seq_len = sqid_seq.len();
        let mut llmt;
        let mut ulmt;
        let mut sliding_only = false;
        let mut pin_candidate_found = false;
        let mut pin_established = false;

        if sqid_seq_len >= 4 {
            if sqid_seq.starts_with("_") {
                sliding_only = true;
                llmt = 1;
                ulmt = 3;
            } else {
                llmt = 0;
                ulmt = 2;
            }

            while ulmt <= sqid_seq_len {
                let sq = &sqid_seq[llmt..ulmt];
                let square = Square::from_str(sq).unwrap();            
                let piece = &self.pieces.get(&square);
                match piece {
                    Some(piece) => {
                        let piece_type_char = piece.get_piece_type_as_char();
                        if let Some(piece_type_ref) =
                            PieceType::get_piece_type_data(piece_type_char)
                        {
                            let piece_data: &'static PieceTypeData = piece_type_ref.get_data();
                            let pid = piece.get_pid();

                            if !pin_candidate_found {
                                if !sliding_only {
                                    // first piece is one step from the king!!
                                    // if current piece is not pinnable then abandon function!!
                                    if piece_data.directions.contains(&odrctn)
                                        && (!(piece_data.basic_piece_type == BasicPieceType::Pawn
                                            && VERTICALS.contains(&odrctn))
                                            || HALF_WINDS.contains(&drctn))
                                    {
                                        return None;
                                    }
                                } else {
                                    //sliding only
                                    if piece_data.sliding && piece_data.directions.contains(&odrctn)
                                    {
                                        return None;
                                    }
                                }
                                pin_candidate_found = true;
                                sliding_only = true;

                                if focus_king_piece_data.side == piece_data.side {
                                    pins.push('<');
                                } else {
                                    pins.push('>');
                                }
                                // pins.push_str("*");
                            } else if !pin_established {
                                if !piece_data.directions.contains(&odrctn)
                                    || !piece_data.sliding
                                    || piece_data.side == focus_king_piece_data.side
                                {
                                    return None;
                                }
                                pin_established = true;
                            } else if !piece_data.directions.contains(&odrctn)
                                || !piece_data.sliding
                            {
                                break;
                            }

                            pins.push_str(&pid);
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
        if !pins.is_empty() { Some(pins) } else { None }
    }

    pub fn process_move(&self, from: Square, to: Square) -> Option<Board> {
        // assume a legal move - but some checks anyway
        if self.is_square_occupied(from) && let Some(piece) = self.get_piece_on(from) {
            let pchar = piece.get_piece_type_as_char();
            let pid = format!("{}{}", to, pchar);
            let mut new_board = self.clone();
            new_board.remove_piece_from(from);
            new_board.create_and_place_piece(&pid);
            return Some(new_board);
        }
        None
    }

    pub fn assess_move(&self, from: Square, to: Square) -> Option<Board> {
        // temp mame for intro for efficient exchanger calc
        // assume a legal move - but some checks anyway
        if self.is_square_occupied(from) && let Some(piece) = self.get_piece_on(from) {
            let pchar = piece.get_piece_type_as_char();
            let mdir = get_direction(from, to);
            println!("Direction of {from}-{to} is {mdir:?}");
            let pid = format!("{}{}", to, pchar);
            let mut new_board = self.clone();
            new_board.assess_vacated(from);
            new_board.remove_piece_from(from);
            new_board.create_and_place_piece(&pid);
            return Some(new_board);
        }
        None
    }

    pub fn build_new_xchngrs(&mut self) {

    }

    pub fn assess_vacated(&self, sq: Square) {
        println!(" {sq:?}");
    }

    pub fn get_piece_on(&self, square: Square) -> Option<&Piece> {
        // let square = Square::from_str(sq).unwrap();                
        self.pieces.get(&square)
    }

    pub fn remove_piece_from(&mut self, sq: Square) {
        // let square = Square::from_str(sq).unwrap();                
        let piece = self.pieces.remove(&sq);
        if piece.is_some() && let Some(bit) = square_to_bit(sq) {
            self.occupied &= !(1u64 << bit);
        }
    }

    pub fn is_square_occupied(&self, sq: Square) -> bool {
        // let square = Square::from_str(sq).unwrap();                
        self.pieces.contains_key(&sq)
    }

    pub fn iter_pieces(&self) -> std::collections::hash_map::Iter<'_, Square, Piece> {
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

    pub fn init_custom_from(&mut self) {
        self.occupied = 0;
        self.create_and_place_piece("e1K");
        self.create_and_place_piece("e2Q");
        self.create_and_place_piece("e3R");
        self.create_and_place_piece("e4R");
        self.create_and_place_piece("e5P");
        self.create_and_place_piece("e6r");
        self.create_and_place_piece("e7r");
        self.create_and_place_piece("e8q");
        self.create_and_place_piece("f8k");
        self.create_and_place_piece("f6p");
    }

    pub fn init_custom_to(&mut self) {
        self.occupied = 0;
        self.create_and_place_piece("e1K");
        self.create_and_place_piece("e2Q");
        self.create_and_place_piece("e3R");
        self.create_and_place_piece("e4R");
        // self.create_and_place_piece("e5P");
        self.create_and_place_piece("e6r");
        self.create_and_place_piece("e7r");
        self.create_and_place_piece("e8q");
        self.create_and_place_piece("f8k");
        self.create_and_place_piece("f6P");
    }

    pub fn init_custom1(&mut self) {
        self.create_and_place_piece("f7R");
        self.create_and_place_piece("e6p");
        self.create_and_place_piece("a5r");
        self.create_and_place_piece("b5B");
        self.create_and_place_piece("d5K");
        self.create_and_place_piece("d4P");
        self.create_and_place_piece("d3Q");
        self.create_and_place_piece("f3b");
        self.create_and_place_piece("c2n");
        self.create_and_place_piece("g2B");
        self.create_and_place_piece("a1k");
    }

    pub fn init_standard(&mut self) {
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

    pub fn get_occupied_bitboard(&self) -> u64 {
        self.occupied
    }

    pub fn print_occupied_squares(&self) {
        println!("Occupied squares (from bitboard):");
        for bit in 0..64 {
            if (self.occupied & (1u64 << bit)) != 0
                && let Some(square) = bit_to_string_square(bit)
            {
                println!("-{square}");
            }
        }
    }
}
