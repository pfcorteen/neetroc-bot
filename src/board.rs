use crate::compass_groups::{Direction, HALF_WINDS, VERTICALS};
use crate::occupied_squares::{bit_to_square, generate_ray_path, square_to_bit};
use crate::pieces::BasicPieceType;
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::pieces::PieceTypeData;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Board {
    occupied: u64, // representation of pieces as bits in 8 bytes according to piece position
    pieces: HashMap<String, Piece>,
}

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
            let square = &piece_identifier[0..2].to_string();
            self.pieces.insert(square.clone(), piece);

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
        println!("DEBUG: Entering build_all_xchngrs");
        // First collect all the paths and directions we need to process
        let mut updates: Vec<(String, Direction, String)> = Vec::new();

        println!("DEBUG: Starting piece iteration");
        for (square, piece) in &self.pieces {
            println!("DEBUG: Processing piece at square {square}");
            let piece_type_char = piece.get_piece_type_as_char();
            if let Some(piece_type_ref) = PieceType::get_piece_type_data(piece_type_char) {
                let data: &'static PieceTypeData = piece_type_ref.get_data();
                println!(
                    "Key: {}, Value: {:?}, sliding: {}, side: {:?}",
                    square, piece_type_char, data.sliding, data.side
                );

                for drctn in Direction::iter() {
                    let ray_path_opt = generate_ray_path(square, drctn, self.occupied);
                    match ray_path_opt {
                        None => {
                            println!("No ray path from {square}, {drctn:?}");
                        }
                        Some(ray_path) => {
                            let xchngr_opt = Board::extract_pid_seq(self, data, &ray_path, drctn);
                            match xchngr_opt {
                                None => {
                                    println!(
                                        "No sequence was extracted from {}, {:?}, {}",
                                        square, drctn, &ray_path
                                    );
                                }
                                Some(xchngrs) => {
                                    updates.push((square.clone(), drctn, xchngrs));
                                }
                            }
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
                    let xv = piece.exchangers.get(&drctn);
                    if let Some(v) = xv {
                        println!("Piece.exchangers: {square}, {drctn:?}: {v}");
                    } else {
                        println!("no exchanges on {square} for direction {drctn:?}: {xv:?}");
                    }
                } else {
                    println!("Zero length xchnger_str on {square} for direction {drctn:?}");
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
                let piece = &self.pieces.get(sq);
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
                let piece = &self.pieces.get(sq);
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

    pub fn get_piece_on(&self, square: &str) -> Option<&Piece> {
        self.pieces.get(square)
    }

    pub fn remove_piece_from(&mut self, square: &str) {
        let piece = self.pieces.remove(square);

        if piece.is_some()
            && let Some(bit) = square_to_bit(square)
        {
            self.occupied &= !(1u64 << bit);
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

    pub fn initialise_custom0(&mut self) {
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

    pub fn initialise_custom1(&mut self) {
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

    pub fn get_occupied_bitboard(&self) -> u64 {
        self.occupied
    }

    pub fn print_occupied_squares(&self) {
        println!("Occupied squares (from bitboard):");
        for bit in 0..64 {
            if (self.occupied & (1u64 << bit)) != 0
                && let Some(square) = bit_to_square(bit)
            {
                println!("-{square}");
            }
        }
    }
}
