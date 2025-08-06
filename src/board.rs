use crate::{CARDINALS, ORDINALS};
use crate::compass_groups::{Direction, HALF_WINDS, VERTICALS, get_direction};
use crate::occupied_squares::{bit_to_string_square, generate_ray_path, square_to_bit};
use crate::pieces::BasicPieceType;
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::pieces::PieceTypeData;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write; // for write! macro
use std::str::FromStr;
use strum::{AsRefStr, Display, EnumString};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Copy, Clone)] // These are useful traits to derive
#[derive(Display, AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Square {
    a1,
    b1,
    c1,
    d1,
    e1,
    f1,
    g1,
    h1,
    a2,
    b2,
    c2,
    d2,
    e2,
    f2,
    g2,
    h2,
    a3,
    b3,
    c3,
    d3,
    e3,
    f3,
    g3,
    h3,
    a4,
    b4,
    c4,
    d4,
    e4,
    f4,
    g4,
    h4,
    a5,
    b5,
    c5,
    d5,
    e5,
    f5,
    g5,
    h5,
    a6,
    b6,
    c6,
    d6,
    e6,
    f6,
    g6,
    h6,
    a7,
    b7,
    c7,
    d7,
    e7,
    f7,
    g7,
    h7,
    a8,
    b8,
    c8,
    d8,
    e8,
    f8,
    g8,
    h8,
}

#[derive(Debug, Clone)]
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

    pub fn place_piece(&mut self, piece: Piece) {
        let sq = piece.get_square();
        let square = Square::from_str(sq).unwrap();
        self.pieces.insert(square, piece);
        let bit = square_to_bit(square);
        self.occupied |= 1u64 << bit;
    }

    pub fn create_and_place_piece(&mut self, piece_identifier: &str) {
        if let Some(piece) = Piece::new(piece_identifier) {
            let sq = &piece_identifier[0..2];
            let square = Square::from_str(sq).unwrap();
            self.pieces.insert(square, piece);

            let bit = square_to_bit(square);
            self.occupied |= 1u64 << bit;
        } else {
            println!("Unable to create piece with identifier: {}", piece_identifier);
        }
    }
    pub fn updates_per_piece(
        &self,
        square: &Square,
        piece: Piece,
    ) -> Vec<(Square, Direction, String)> {
        let mut updates: Vec<(Square, Direction, String)> = Vec::new();
        let piece_type_char = piece.get_piece_type_as_char();
        if let Some(piece_type) = PieceType::get_piece_type(piece_type_char) {
            let data: &'static PieceTypeData = piece_type.get_data();
            for drctn in Direction::iter() {
                if let Some(ray_path) = generate_ray_path(*square, drctn, self.occupied) {
                    // println!("Square: {square}, drctn: {drctn}, raypath: {ray_path}");
                    if let Some(xchngrs) = Board::extract_pid_seq(self, data, &ray_path, drctn) {
                        updates.push((*square, drctn, xchngrs));
                    }
                }
            }
        }
        updates
    }
    pub fn build_all_xchngrs(&mut self) {
        // println!("DEBUG: Entering build_all_xchngrs");
        // First collect all the paths and directions we need to process
        let mut updates: Vec<(Square, Direction, String)> = Vec::new();

        // println!("DEBUG: Starting piece iteration");
        for (square, piece) in &self.pieces {
            let updates_for_piece = self.updates_per_piece(square, piece.clone());
            updates.extend(updates_for_piece);
        }

        for (square, drctn, xchngrs) in updates {
            if let Some(piece) = self.pieces.get_mut(&square) {
                if !xchngrs.is_empty() {
                    piece.exchangers.insert(drctn, xchngrs.clone());
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
                        if let Some(piece_type_ref) = PieceType::get_piece_type(piece_type_char) {
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
                        if let Some(piece_type) = PieceType::get_piece_type(piece_type_char) {
                            let piece_data: &'static PieceTypeData = piece_type.get_data();
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

    pub fn full_process_move(&self, from: Square, to: Square) -> Option<Board> {
        // assume a legal move - but some checks anyway
        let piece = self.get_piece_on(from)?;
        let pchar = piece.get_piece_type_as_char();
        let pid = format!("{}{}", to, pchar);
        let mut new_board = self.clone();
        println!("Fully processed move '{from}-{to}', with pid '{pid}'");
        new_board.remove_piece_from(from);
        new_board.create_and_place_piece(&pid);
        return Some(new_board);
    }

    pub fn pre_processed_move(&self, from: Square, to: Square) -> Option<Board> {

        let mut xr_updates: Vec<(Square, Direction, Option<String>)> = Vec::new();

        // assume a legal move - but some checks anyway
        let piece = self.get_piece_on(from)?;
        let pchar = piece.get_piece_type_as_char();
        let pid = format!("{}{}", to, pchar);
        let mut prpsd_board = self.clone();
        println!("Pre_processed move '{from}-{to}', with pid '{pid}'");

        prpsd_board.assess_vacated(from, to, & mut xr_updates);

        prpsd_board.assess_landed(from, to, & mut xr_updates);

        for (sq, dir, xrs_opt) in xr_updates {
            if let Some(rpiece) = prpsd_board.pieces.get_mut(&sq) {
                match xrs_opt {
                    None => {
                        rpiece.exchangers.remove(&dir);
                        // println!("{}", rpiece);
                    }
                    Some(xrs) => {
                        rpiece.exchangers.insert(dir, xrs);
                        // println!("{}", rpiece);
                    }
                }
            }
        }

        return Some(prpsd_board);
    }

    pub fn assess_vacated(&mut self, from: Square, to: Square, updates: & mut Vec<(Square, Direction, Option<String>)>) {
        let mvng_piece = self.pieces.get(&from).expect("assess_vacated: expected a piece to exist at the 'from' square");
        let pchar = mvng_piece.get_piece_type_as_char();
        let mdir = get_direction(from, to).expect("assess_vacated: surely a legal move must have a valid direction?");
        println!("Direction of {from}-{to} is {mdir:?}");
        let pid = format!("{}{}", to, pchar);
        let mut used_opposites: Vec<Direction> = Vec::new();
        // First pass: collect all (Square, Direction) pairs to update
        let xr_data: Vec<(Direction, String, Option<String>)> = mvng_piece
            .exchangers
            .iter()
            .map(|(d, s)| {
                let od = d.opposite();
                let opt_ref = mvng_piece.exchangers.get(&od).cloned();
                (*d, s.clone(), opt_ref)
            })
            .collect();
        let mut transfer_exchangers 
                = | opp_dir: Direction,
                    xrs: &String,
                    opp_xrs: &String | -> Vec<(Square, Direction, Option<String>)>{
            let pattern_len = 3;
            let dir = opp_dir.opposite();
            let mut dir_xrs: Option<String> = None;
            let mut  updts: Vec<(Square, Direction, Option<String>)> = Vec::new();
            for (i, _char_chunk) in xrs.char_indices().step_by(pattern_len) {
                if let Some(chunk) = xrs.get(i..(i + pattern_len)) {
                    // println!("d chunk: {}", chunk);
                    let od_sq = Square::from_str(&chunk[0..=1]).unwrap();
                    let od_piece = self
                        .pieces
                        .get_mut(&od_sq)
                        .expect("Expected a piece to exist at the given square");
                    dir_xrs = od_piece.exchangers.get(&dir).cloned();
                    let od_piece_data = od_piece.get_piece_data();
                    if od_piece_data.sliding == false {
                        // only possible for first exchanger in list - do not continue with list
                        updts.extend([(od_sq, opp_dir, Some(opp_xrs.clone()))]);
                        break;
                    } else if let Some(exstng_xrs) = od_piece.exchangers.get(&opp_dir) {
                        println!("tramsfer_exchangers exstng_xrs opp_xrs");
                        updts.extend([(
                            od_sq,
                            opp_dir,
                            Some(exstng_xrs.to_string() + &opp_xrs.clone()),
                        )]);                       
                    } else {
                        println!("tramsfer_exchangers opp_xrs only");
                        updts.extend([(od_sq, opp_dir, Some(opp_xrs.clone()))]);
                    }
                }
            }
            
            // Handle hanger-on after the loop (when mutable borrow is dropped)
            if let Some(d_pid) = dir_xrs {
                let d_sq = Square::from_str(&d_pid[0..=1]).unwrap();
                let d_piece = self.pieces.get(&d_sq).expect("Expected hanger-on piece to exist at the given square");
                let exstng_xrs = d_piece.exchangers.get(&opp_dir).cloned();
                // let exstng_xrs = d_piece.exchangers.get(&dir).cloned();
                if let Some(exstng_xrs) = exstng_xrs {
                    println!("transfer_exchanger - the hanger-on {dir}: {d_pid}");
                    updts.extend([(
                        d_sq,
                        opp_dir,
                        Some(exstng_xrs.to_string() + &opp_xrs.clone()),
                    )]);
                }                        
            }

            updts
        };

        for (d, d_xrs, od_opt_ref) in &xr_data {
            let od = d.opposite();
            if !used_opposites.contains(d) {
                match od_opt_ref {
                    Some(od_xrs) => {
                        // let od_xs = od_xs_opt.as_ref();
                        if HALF_WINDS.contains(d) {
                            let d_sq = Square::from_str(&d_xrs[0..=1]).unwrap();
                            let od_sq = Square::from_str(&od_xrs[0..=1]).unwrap();
                            updates.extend([(d_sq, *d, None)]);
                            updates.extend([(od_sq, od, None)]);
                        } else  {
                            // if CARDINALS.contains(d) {
                            // ultimately there may be no reason to process CARDINALS seperately from ORDINALS
                            let od_trnsfr_updates = transfer_exchangers(od, d_xrs, od_xrs);
                            let d_trnsfr_updates = transfer_exchangers(*d, od_xrs, d_xrs);
                            updates.extend(od_trnsfr_updates);
                            updates.extend(d_trnsfr_updates);
                        }

                        used_opposites.push(od);
                    }
                    None => {
                        println!("No opposite exchangers for direction {od:?}");
                    }
                }
            }
        }

        // return xr_updates;
    }
    pub fn assess_landed(&mut self, from: Square, to: Square, updates: & mut Vec<(Square, Direction, Option<String>)>)  {
        let from_piece: &Piece = self.get_piece_on(from).unwrap();
        let from_pid = from_piece.get_pid();
        let from_pchar = from_piece.get_piece_type_as_char();
        let from_data = from_piece.get_piece_data();
        let from_drctns = &from_data.directions;
        let mdir = get_direction(from, to).unwrap();
        let opdir = mdir.opposite();
        println!("Direction of {from}-{to} is {mdir:?}");
        let new_pid = format!("{}{}", to, from_pchar);
        let mut used_opposites: Vec<Direction> = Vec::new();

        if self.pieces.contains_key(&to) { // NB: this is pre-move!!
            // transrer the exchangers from the captured piece to the new piece
            let mut cptrd_piece = self.pieces.get(&to).unwrap();
            // I don't need to remove the cptrd_piece from the board as it 
            // will simply be replaced at that square by the new piece

            let cptrd_piece_pid = cptrd_piece.get_pid();  // needed to be able to remove from exchangers where necessary
            let cptrd_data = cptrd_piece.get_piece_data();
            let cptrd_drctns = &cptrd_data.directions;

            let mut new_piece = Piece::new(&new_pid).unwrap();

            new_piece.exchangers = cptrd_piece.exchangers.clone();

            let op_mv_xrs = new_piece.exchangers.get_mut(&opdir).unwrap();
            let op_mv_xrs = op_mv_xrs.replace(from_pid, "");
            if op_mv_xrs.len() == 0 {
                new_piece.exchangers.remove(&opdir);
                // updates.extend([(to, opdir, None)]);
            } else {
                new_piece.exchangers.insert(opdir, op_mv_xrs);
                // updates.extend([(to, opdir, Some(op_mv_xrs))]);
            }

            let new_piece_data = new_piece.get_piece_data();
            let new_piece_drctns = &new_piece_data.directions;

            for new_drctn in new_piece_drctns {
                if let Some(ray_path) = generate_ray_path(to, *new_drctn, self.occupied) {
                    //here we are not looking to establish exchangers on the moved to square...
                    // intead we need to put exchangers information about the moved piece attacking or defending
                    println!("assess_landed attacking - Square: {to}, drctn: {new_drctn}, raypath: {ray_path}");
                    let sliding_only = ray_path.starts_with('_'); // ...at a distance therefore sliding only
                    let odir = new_drctn.opposite();

                    let llmt;
                    let ulmt;
                    if sliding_only {
                        llmt = 1;
                        ulmt = 3;
                    } else {
                        llmt = 0;
                        ulmt = 2;
                    }

                    let sq = &ray_path[llmt..ulmt];
                    let square = Square::from_str(&sq).unwrap();
                    if !sliding_only  {
                        if new_piece_data.basic_piece_type == BasicPieceType::Pawn 
                                && ORDINALS.contains(new_drctn) {
                            updates.extend([(square, odir, Some(new_pid.clone()))]);
                            println!("assess_landed: not sliding only drctn: {new_drctn}, pid_seq: {}", &new_pid);                      
                        } else { // sliding only
                            println!("assess_landed: sliding only drctn: {new_drctn}, pid_seq: {}", &new_pid);
                        }
                    } else {
                        // sliding only
                    }
                }
            }

            self.remove_piece_from(from);
            self.place_piece(new_piece);
        } else {
            // calculate the moved pieces new exchangers
            let mut new_piece = Piece::new(&new_pid).unwrap();

            let updates_for_piece = self.updates_per_piece(&to, new_piece.clone());
            // Convert (Square, Direction, String) to (Square, Direction, Option<String>)
            let converted_updates: Vec<(Square, Direction, Option<String>)> = updates_for_piece
                .into_iter()
                .map(|(sq, dir, xchngrs)| (sq, dir, Some(xchngrs)))
                .collect();
            updates.extend(converted_updates);

            self.remove_piece_from(from);
            self.place_piece(new_piece);
        }
    }

    pub fn update_xchngr(&self, xchngr: String, drctn: Direction, val: Option<String>) {
        println!("xchngr: {xchngr}, drctn: {drctn}, val: {val:?}")
    }

    pub fn get_piece_on(&self, square: Square) -> Option<&Piece> {
        // let square = Square::from_str(sq).unwrap();
        self.pieces.get(&square)
    }

    pub fn remove_piece_from(&mut self, sq: Square) {
        // let square = Square::from_str(sq).unwrap();
        if let Some(_piece) = self.pieces.remove(&sq) {
            let bit = square_to_bit(sq);
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

        self.create_and_place_piece("c6N");
        self.create_and_place_piece("e5P");
        self.create_and_place_piece("g4n");

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

    pub fn to_ordered_string(&self) -> String {
        let mut out = String::new();
        for square in Square::iter() {
            if let Some(piece) = self.pieces.get(&square) {
                writeln!(&mut out, "{}", piece).unwrap();
            }
        }
        out
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ordered_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Square::*;

    #[test]
    fn test_board_move_exchangers_equality() {
        println!("=== Starting test_board_move_exchangers_equality ===");
        
        let mut board = Board::new();
        println!("Created new board");
        
        board.init_custom_from();
        println!("Initialized custom board:");
        println!("{}", board);
        
        board.build_all_xchngrs();
        println!("Built all exchangers");
        println!("{}", board);
        
        println!("\n=== Testing full_process_move ===");
        let mut next_board = board
            .full_process_move(e5, f6)
            .expect("full_process_move failed");
        println!("Full process move completed");
        
        next_board.build_all_xchngrs();
        println!("Built exchangers for next_board");
        println!("{}", next_board);
        
        println!("\n=== Testing pre_processed_move ===");
        let prpsd_board = board
            .pre_processed_move(e5, f6)
            .expect("pre_processed_move failed");
        println!("Pre-processed move completed");

        let next_str = next_board.to_ordered_string();
        let prpsd_str = prpsd_board.to_ordered_string();
        
        println!("\n=== Comparing results ===");
        println!("next_board piece count: {}", next_board.len());
        println!("prpsd_board piece count: {}", prpsd_board.len());
        
        if next_str != prpsd_str {
            println!("\n=== DETAILED DIFF ===");
            println!("next_board string length: {}", next_str.len());
            println!("prpsd_board string length: {}", prpsd_str.len());
            
            let next_lines: Vec<_> = next_str.lines().collect();
            let prpsd_lines: Vec<_> = prpsd_str.lines().collect();
            
            println!("next_board lines: {}", next_lines.len());
            println!("prpsd_board lines: {}", prpsd_lines.len());
            
            let max_len = next_lines.len().max(prpsd_lines.len());
            for i in 0..max_len {
                let n = next_lines.get(i).unwrap_or(&"");
                let p = prpsd_lines.get(i).unwrap_or(&"");
                if n != p {
                    println!("Line {}:", i);
                    println!("- Next:   '{}'", n);
                    println!("+ Prpsd:  '{}'", p);
                }
            }
            
            println!("\n=== FULL BOARD CONTENTS ===");
            println!("next_board:");
            println!("{}", next_board);
            println!("\nprpsd_board:");
            println!("{}", prpsd_board);
        }
        
        assert_eq!(
            next_str, prpsd_str,
            "Board string representations differ, see diff above"
        );
        
        println!("=== Test passed! ===");
    }
}
