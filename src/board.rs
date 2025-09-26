use crate::{Side, CARDINALS};
use crate::compass_groups::{Direction, DIRECTION_PAIRS, HALF_WINDS, VERTICALS, get_direction};
use crate::occupied_squares::{bit_to_string_square, generate_ray_path, square_to_bit, get_next_sqid, first_occpd_square};
use crate::pid::Pid;
use crate::pieces::BasicPieceType;
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::pieces::PieceTypeData;
use crate::pieces::King_Locations;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write; // for write! macro
use std::str::FromStr;
use strum::{AsRefStr, Display, EnumString};
use strum::{EnumIter, IntoEnumIterator};
use std::time::{Duration, Instant};

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
    pieces: HashMap<Square, Piece>,
    occupied: u64,
    moves: Vec<(Square, Square)>, // representation of pieces as bits in 8 bytes according to piece position
    turn: Side,
    checks: Vec<Pid>,
    capture_square_en_passant: Option<Square>,
    white_king_location: Option<Square>,
    black_king_location: Option<Square>,
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
            moves: [].to_vec(),
            turn:  Side::White,
            checks: [].to_vec(),
            capture_square_en_passant: None,
            white_king_location: None,
            black_king_location: None,
        }
    }

    pub fn place_piece(&mut self, piece: Piece) {
        let square = piece.get_square();
        self.pieces.insert(square, piece);
        let bit = square_to_bit(square);
        self.occupied |= 1u64 << bit;
    }

    pub fn create_and_place_piece(&mut self, piece_identifier: &str) {
        match Pid::new(piece_identifier) {
            Ok(pid) => {
                let piece = Piece::new(pid);
                let sq = &piece_identifier[0..2];
                let square = Square::from_str(sq).unwrap();
                self.pieces.insert(square, piece);

                let bit = square_to_bit(square);
                self.occupied |= 1u64 << bit;
            }
            Err(e) => {
                println!("Unable to create piece with identifier '{}': {}", piece_identifier, e);
            }
        }
    }
    pub fn updates_per_piece( // GATHER THE EXCHANGERS FOR THE PIECE
        &self,
        square: &Square,
        focus_piece_data: &'static PieceTypeData,
    ) -> Vec<(Square, Direction, String)> {
        let mut updates: Vec<(Square, Direction, String)> = Vec::new();
        // let piece_type_char = piece.get_piece_type_as_char();
        // if let Some(piece_type) = PieceType::get_piece_type(piece_type_char) {
            // let data: &'static PieceTypeData = piece_type.get_data();
        for d in Direction::iter() {
            // let od = d.opposite();
            if let Some(ray_path) = generate_ray_path(*square, d, self.occupied) {
                // println!("Square: {square}, d: {d}, raypath: {ray_path}");
                if let Some(xchngrs) = Board::extract_pid_seq(self, &focus_piece_data, &ray_path, d) {

                    updates.push((*square, d, xchngrs.clone()))

                    // if let Some(d_pid) = dir_xrs {
                    //     let d_sq = Square::from_str(&d_pid[0..=1]).unwrap();
                    //     let d_piece = self.pieces.get(&d_sq).expect("Expected hanger-on piece to exist at the given square");
                    //     let exstng_xrs = d_piece.exchangers.get(&opp_dir).cloned();
                    //     if let Some(exstng_xrs) = exstng_xrs {
                    //         // println!("transfer_exchanger - the hanger-on {dir}: {d_pid}");
                    //         updts.extend([(
                    //             d_sq,
                    //             opp_dir,
                    //             Some(exstng_xrs.to_string() + &opp_xrs.clone()),
                    //         )]);
                    //     }                        
                    // }
                } 
            } else { // no ray, so hopefully remove any hanging exchangers for piece that has moved.
                updates.push((*square, d, String::new()));
            }
        }
        updates
    }
    pub fn build_all_xchngrs(&mut self) {
        // First collect all the paths and directions we need to process
        let mut updates: Vec<(Square, Direction, String)> = Vec::new();
        let mut piece_info_to_process: Vec<(Square, &'static PieceTypeData)> = Vec::new();

        // First pass: Collect necessary immutable data from pieces.
        // This avoids holding an immutable borrow on `self.pieces` while we need to mutate it.
        for (square, piece) in &self.pieces {
            let piece_data = piece.get_piece_data();
            piece_info_to_process.push((*square, piece_data));
        }

        // Second pass: Clear all existing exchangers for each piece.
        // This requires a mutable borrow of `self.pieces`.
        for piece in self.pieces.values_mut() {
            piece.exchangers = HashMap::new();
        }

        // Third pass: Calculate updates for each piece using the collected data.
        // This requires an immutable borrow of `self` (for `updates_per_piece`).
        for (square, piece_data) in piece_info_to_process {
            let updates_for_piece = self.updates_per_piece(&square, piece_data);
            updates.extend(updates_for_piece);
        }

        for (square, drctn, xchngrs) in updates {
            if let Some(piece) = self.pieces.get_mut(&square) {
                if !xchngrs.is_empty() {
                    piece.exchangers.insert(drctn, xchngrs.clone());
                // } else {
                //     piece.exchangers.remove(&drctn);
                }
            }
        }
    }

    fn extract_pid_seq(
        &self,
        focus_piece_data: &PieceTypeData,
        // focus_piece_data: &PieceTypeData,
        sqid_seq: &str,
        drctn: Direction,
    ) -> Option<String> {
        // let focus_piece_data = focus_piece.get_piece_data();
        let odrctn = drctn.opposite();
        let mut pids = String::new(); // NB: pins will be indicated with:
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
                let piece_opt = &self.pieces.get(&square);
                match piece_opt  {
                    Some(piece) => {
                        let piece_type_char = piece.get_piece_type_as_char();
                        if let Some(piece_type_ref) = PieceType::get_piece_type(piece_type_char) {
                            let xr_data: &'static PieceTypeData = piece_type_ref.get_data();
                            let pid = piece.get_pid();
                            if !sliding_only {
                                // first piece encountered only one step away, so allow single step pieces
                                if !xr_data.directions.contains(&odrctn) {
                                    if !HALF_WINDS.contains(&drctn)
                                                && focus_piece_data.basic_piece_type == BasicPieceType::King {
                                        return Board::extract_pin_seq(self,focus_piece_data, sqid_seq, drctn);
                                    } else {
                                        if focus_piece_data.basic_piece_type == BasicPieceType::Pawn
                                                && focus_piece_data.side != xr_data.side
                                                    && self.capture_square_en_passant.is_some() {
                                            let epcapture_square = self.capture_square_en_passant.unwrap();
                                            // we need the focus piece square (via pid) to check files are adjacent
                                            // println!("Pawn focal piece - ep capture square: {:?}", self.capture_square_en_passant);
                                            return Board::extract_pin_seq(self,focus_piece_data, sqid_seq, drctn);
                                    } else {
                                            return None;
                                        }
                                    }
                                } else if xr_data.basic_piece_type == BasicPieceType::Pawn
                                    && VERTICALS.contains(&drctn) {
                                    // return None;
                                    return Board::extract_pin_seq(self,focus_piece_data, sqid_seq, drctn);
                                }
                            } else {
                                //sliding only
                                if !xr_data.is_sliding || !xr_data.directions.contains(&odrctn) {
                                    if !pids.is_empty() {
                                        return Some(pids);
                                    } else if !HALF_WINDS.contains(&drctn)
                                        && focus_piece_data.basic_piece_type == BasicPieceType::King {
                                            return Board::extract_pin_seq(self,focus_piece_data, sqid_seq, drctn)
                                    } else {
                                        return None;
                                    }
                                }
                            }

                            pids.push_str(&pid.to_string());
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
        // else {
        //     println!("This shows that a zero length raypath was going to be processed - stop that");
        // }

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
                                    if piece_data.is_sliding && piece_data.directions.contains(&odrctn)
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
                                    || !piece_data.is_sliding
                                    || piece_data.side == focus_king_piece_data.side
                                {
                                    return None;
                                }
                                pin_established = true;
                            } else if !piece_data.directions.contains(&odrctn)
                                || !piece_data.is_sliding
                            {
                                break;
                            }

                            pins.push_str(&pid.to_string());
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

    pub fn full_process_move(&self, from: Square, to: Square) -> Board {
        // assume a legal move - but some checks anyway
        let start = Instant::now();

        let mut new_board = self.clone();
        // let mut is_en_passant_capture = false;
        // let mut ep_square: Square;
        if self.capture_square_en_passant.is_some() {
            let ep_square = self.capture_square_en_passant.unwrap();
            if ep_square == to { 
                // is_en_passant_capture = true;
                let sqid_file = ep_square.to_string().chars().nth(0).unwrap();
                let sqid_rank = ep_square.to_string().chars().nth(1).unwrap();

                let new_rank;
                if sqid_rank == '6' {
                    new_rank = '5';
                } else {
                    new_rank = '4';
                }

                let ep_captured_pawn_sq = format!("{}{}", sqid_file, new_rank);
                let ep_captured_square = Square::from_str(&ep_captured_pawn_sq).unwrap();
                new_board.remove_piece_from(ep_captured_square);

                new_board.capture_square_en_passant = None;
            }
        }

        let piece = self.get_piece_on(from).unwrap();
        let pchar = piece.get_piece_type_as_char();
        let pid = format!("{}{}", to, pchar);

        new_board.remove_piece_from(from);
        new_board.create_and_place_piece(&pid);

        new_board.build_all_xchngrs();
        new_board.moves.push((from, to));
        new_board.update_status(from, to, pchar);

        let duration = start.elapsed();
        println!("Fully processed move {from}-{to} took {} nanos.", duration.as_nanos());

        new_board
    }

    pub fn pre_processed_move(&self, from: Square, to: Square) -> Board {
        let start = Instant::now();

        let mut xr_updates: Vec<(Square, Direction, Option<String>)> = Vec::new();

        let from_piece = self.get_piece_on(from).unwrap();
        let pchar = from_piece.get_piece_type_as_char();
        let new_pid = format!("{}{}", to, pchar);
        let new_piece = Piece::new(Pid::new(&new_pid).unwrap());
        let move_drctn = get_direction(from, to)
                .expect("pre_processed_move: surely a legal move must have a valid direction?");
        let drctn_back = move_drctn.opposite();
        println!("Pre_processed move '{from}-{to}', with pid '{new_pid:?}'");
        // assume a legal move - but some checks anyway

        let mut prpsd_board = self.clone();
        if self.capture_square_en_passant.is_some() {
            let ep_square = self.capture_square_en_passant.unwrap();
            if ep_square == to { 
                
                let sqid_file = ep_square.to_string().chars().nth(0).unwrap();
                let sqid_rank = ep_square.to_string().chars().nth(1).unwrap();

                let new_rank;
                if sqid_rank == '6' {
                    new_rank = '5';
                } else {
                    new_rank = '4';
                }

                let ep_captured = format!("{}{}", sqid_file, new_rank);
                let ep_captured_square = Square::from_str(&ep_captured).unwrap();
                let ep_captured_piece = self.get_piece_on(ep_captured_square).unwrap();
                prpsd_board.remove_piece_from(ep_captured_square);
                prpsd_board.assess_vacated(ep_captured_piece.clone(), new_pid.clone(), &mut xr_updates);

                prpsd_board.capture_square_en_passant = None;
            }
        }

        prpsd_board.remove_piece_from(from);
        prpsd_board.place_piece(new_piece);

        prpsd_board.assess_vacated(from_piece.clone(), new_pid.clone(), & mut xr_updates);
        prpsd_board.assess_landed(new_pid.clone(), drctn_back, & mut xr_updates);

        // Sort updates by Direction enum before processing
        xr_updates.sort_by(|a, b| a.1.cmp(&b.1));
        
        for (sq, dir, xrs_opt) in xr_updates {
            if let Some(rpiece) = prpsd_board.pieces.get_mut(&sq) {
                match xrs_opt {
                    None => {
                        rpiece.exchangers.remove(&dir);
                    }
                    Some(xrs) => {
                        rpiece.exchangers.insert(dir, xrs);
                    }
                }
            }
        }

        prpsd_board.moves.push((from, to));
        prpsd_board.update_status(from, to, pchar);

        let duration = start.elapsed();
        println!("Efficiently processed move {from}-{to} took {} nanos.", duration.as_nanos());

        prpsd_board
    }

    // pub fn assess_vacated(&mut self, from: Square, new_pid: String, updates: & mut Vec<(Square, Direction, Option<String>)>) {
    pub fn assess_vacated(&mut self, from_piece: Piece, new_pid: String, updates: & mut Vec<(Square, Direction, Option<String>)>) {
        // let mvng_piece = self.pieces.get(&from)
        //         .expect("assess_vacated: expected a piece to exist at the 'from' square");
        let from_data = from_piece.get_piece_data();
        let pchar = from_piece.get_piece_type_as_char();
        let from = from_piece.get_square();
        let to = Square::from_str(&new_pid[0..=1]).unwrap();
        let mdir = get_direction(from, to)
                .expect("assess_vacated: surely a legal move must have a valid direction?");
        println!("Direction of {from}-{to} is {mdir:?}");
        let _pid = format!("{}{}", to, pchar);
        let mut transfer_exchangers 
                = | opp_dir: Direction, xrs: &String, opp_xrs: &String |
                -> Vec<(Square, Direction, Option<String>)> {

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
                    if od_piece_data.is_sliding == false {
                        // only possible for first exchanger in list - do not continue with list
                        updts.extend([(od_sq, opp_dir, Some(opp_xrs.clone()))]);
                        break;
                    } else if let Some(exstng_xrs) = od_piece.exchangers.get(&opp_dir) {
                        // println!("tramsfer_exchangers exstng_xrs opp_xrs");
                        updts.extend([(
                            od_sq,
                            opp_dir,
                            Some(exstng_xrs.to_string() + &opp_xrs.clone()),
                        )]);                       
                    } else {
                        println!("ESCHEWING..ESCHEWING tramsfer_exchangers to piece of {od_sq} | {opp_dir}: {opp_xrs}");
                        // updts.extend([(od_sq, opp_dir, Some(opp_xrs.clone()))]);
                    }
                }
            }
            
            // Handle hanger-on after the loop (when mutable borrow is dropped)
            if let Some(d_pid) = dir_xrs {
                let d_sq = Square::from_str(&d_pid[0..=1]).unwrap();
                let d_piece = self.pieces.get(&d_sq).expect("Expected hanger-on piece to exist at the given square");
                let exstng_xrs = d_piece.exchangers.get(&opp_dir).cloned();
                if let Some(exstng_xrs) = exstng_xrs {
                    // println!("transfer_exchanger - the hanger-on {dir}: {d_pid}");
                    updts.extend([(
                        d_sq,
                        opp_dir,
                        Some(exstng_xrs.to_string() + &opp_xrs.clone()),
                    )]);
                }                        
            }

            updts
        };

        let undo_exchangers = | d: Direction |
                    -> Vec<(Square, Direction, Option<String>)> {

            let od = d.opposite();
            let mut updts: Vec<(Square, Direction, Option<String>)> = Vec::new();

            if let Some(d_xr) = first_occpd_square(from, d, self.occupied) {
                let llmt;
                let ulmt;
                let mut sliding_rqrd = false;
                if d_xr.starts_with("_") {
                    sliding_rqrd = true;
                    llmt = 1;
                    ulmt = 3;
                } else {
                    llmt = 0;
                    ulmt = 2;
                }
    
                let sq = &d_xr[llmt..ulmt];
                let d_square = Square::from_str(sq).unwrap();
                if (from_data.is_sliding || !sliding_rqrd)
                    && !(from_data.basic_piece_type == BasicPieceType::Pawn && VERTICALS.contains(&od)) {
                        updts.extend([(d_square, od, None)]);
                }
            }

            updts
        };


        for (d, od) in DIRECTION_PAIRS.iter() { 
            let d_xrs_opt = from_piece.exchangers.get(d);
            let od_xrs_opt = from_piece.exchangers.get(od);
            if d_xrs_opt.is_some() && od_xrs_opt.is_some() {
                let d_xrs = d_xrs_opt.unwrap();
                let od_xrs = od_xrs_opt.unwrap();
                if HALF_WINDS.contains(d) {
                    let d_sq = Square::from_str(&d_xrs[0..=1]).unwrap();
                    let od_sq = Square::from_str(&od_xrs[0..=1]).unwrap();
                    updates.extend([(d_sq, *d, None)]);
                    updates.extend([(od_sq, *od, None)]);
                } else  {
                    let od_trnsfr_updates = transfer_exchangers(*od, d_xrs, od_xrs);
                    let d_trnsfr_updates = transfer_exchangers(*d, od_xrs, d_xrs);
                    updates.extend(od_trnsfr_updates);
                    updates.extend(d_trnsfr_updates);
                }
                println!("assess_vacated we got some d_xrs and some od_xrs");
            } else if d_xrs_opt.is_some() {
                let d_xrs = d_xrs_opt.unwrap();
                let d_sq = Square::from_str(&d_xrs[0..=1]).unwrap();

                if HALF_WINDS.contains(d) {
                    updates.extend([(d_sq, *od, None)]);
                } else  {
                    updates.extend([(d_sq, *od, None)]);
                }
                println!("assess_vacated we got some d_xrs but none of the other");
            } else if od_xrs_opt.is_some() {
                let od_xrs = od_xrs_opt.unwrap();
                let od_sq = Square::from_str(&od_xrs[0..=1]).unwrap();

                if HALF_WINDS.contains(od) {
                    updates.extend([(od_sq, *d, None)]);
                } else  {
                    updates.extend([(od_sq, *d, None)]);
                }
                println!("assess_vacated we got some od_xrs but none of the other");
            } else {

                if from_data.directions.contains(d) {
                    let d_updates = undo_exchangers(*d);
                    updates.extend(d_updates);
                }

                if from_data.directions.contains(od) {
                    let od_updates = undo_exchangers(*od);
                    updates.extend(od_updates);
                }
            }
        }
    }

    pub fn assess_landed(&mut self, landed_pid: String, drctn_back: Direction, updates: & mut Vec<(Square, Direction, Option<String>)>)  {

        let landed_square = Square::from_str(&landed_pid[0..=1]).unwrap();
        let landed_piece = Piece::new(Pid::new(&landed_pid).unwrap());
        let landed_piece_data = landed_piece.get_piece_data();
        let impose 
                = | drctn: Direction | -> Vec<(Square, Direction, Option<String>)> {

            let opp_drctn = drctn.opposite();
            let mut xr_updates: Vec<(Square, Direction, Option<String>)> = Vec::new();

            if let Some(ray) = first_occpd_square(landed_square, drctn, self.occupied) {
                let llmt;
                let ulmt;
                let mut sliding_rqrd = false;
                if ray.starts_with("_") {
                    sliding_rqrd = true;
                    llmt = 1;
                    ulmt = 3;
                } else {
                    llmt = 0;
                    ulmt = 2;
                }

                let sq = &ray[llmt..ulmt];
                let d_square = Square::from_str(sq).unwrap();
                let d_piece = self.pieces.get(&d_square).unwrap(); // panic if no piece on square!
                let d_piece_pid = d_piece.get_pid().to_string();
                let d_piece_data = d_piece.get_piece_data();
                if d_piece_data.directions.contains(&opp_drctn) { // don't forget pawn anomolies
                    if (d_piece_data.is_sliding || !sliding_rqrd)
                            && !(d_piece_data.basic_piece_type == BasicPieceType::Pawn && VERTICALS.contains(&opp_drctn)) {
                        match d_piece.exchangers.get(&drctn) {
                            Some(onward_d_xrs) => {
                                xr_updates.push((landed_square, drctn, Some(d_piece_pid + onward_d_xrs)));                                
                            },
                            None => {
                                if d_piece_data.basic_piece_type == BasicPieceType::King && !HALF_WINDS.contains(&drctn)  {
                                    let ray = generate_ray_path(d_square, opp_drctn, self.occupied).unwrap();

                                    // ??????

                                    let pin_seq = Board::extract_pin_seq(self,d_piece_data, &ray, opp_drctn);
                                    if pin_seq.is_some() {
                                        let pin = pin_seq.unwrap();
                                        xr_updates.push((d_square, opp_drctn, Some(pin)));
                                        // print!("Just detected a King pin {pin}")
                                    }
                                } //else {
                                    xr_updates.push((landed_square, drctn, Some(d_piece_pid.clone())));
                                //}
                            }
                        }
                    } else if landed_piece_data.directions.contains(&drctn) {
                        if landed_piece_data.is_sliding || !sliding_rqrd {
                            xr_updates.push((d_square, opp_drctn, Some(landed_pid.clone())));
                        }
                    } else {
                        println!("Not sure of why this situation should be processed!")
                    }


                }
            } else {
                print!("no ray found in 'impose'");
            }
            xr_updates
        };        

        for (d, od) in DIRECTION_PAIRS.iter() { 
            let d_updates = impose(*d);
            updates.extend(d_updates);
    
            let od_updates = impose(*od);
            updates.extend(od_updates);
        }
    }

    pub fn update_status(&mut self, from: Square, to: Square, piece_type: char){
        // the current player who's turn it is has not finished the move yet...
        // here is the final accounting!
        let mut turn_king;
        if self.turn == Side::White {
            turn_king = 'K';
        } else {
            turn_king = 'k';
        }

        let mut opp_king;
        if turn_king == 'K' {
            opp_king = 'k';
        } else {
            opp_king = 'K';
        }

        // check for checks
        let mut checks: Vec<Pid> = Vec::new();
        for (_sq, piece) in self.pieces.iter() {
            let piece_type_char = piece.get_piece_type_as_char();
            if piece_type_char == opp_king { // we search for the opposite side's king
                let piece_side = piece.get_side();
                let exchangers_clone = piece.exchangers.clone();
                for (_d, xrs) in exchangers_clone {
                    let xr = &xrs[0..3]; // take first three characters
                    if xr.chars().all(char::is_alphanumeric) { // Check if all characters are alphabetic i.e. don't include '<' or '>'
                        let xr_pid = Pid::new(xr).unwrap();
                        let xr_pid_for_check = xr_pid.clone(); // Clone xr_pid before it's potentially moved
                        let xr_side = xr_pid.get_side(); // This line likely moves `xr_pid`
                        if xr_side != piece_side {
                            checks.push(xr_pid_for_check); // Use the cloned value
                        }
                    }
                }
            }
        }
        if checks.len() > 0 {
            self.checks.extend(checks);
        } else {
            self.checks.clear();
        }

        let set_en_passant = || -> Option<Square> {
            // ep_capture_sqid = file: wsqid.file, rank: (wsqid_rank == 5) ? 6 : 3 
            let sqid_file = to.to_string().chars().nth(0).unwrap();
            let sqid_rank = to.to_string().chars().nth(1).unwrap();
            let new_rank;
            if sqid_rank == '5' {
                new_rank = '6';
            } else {
                new_rank = '3';
            }
            let ep_capture_sqid = format!("{}{}", sqid_file, new_rank);
            let ep_capture_square = Square::from_str(&ep_capture_sqid).expect("Failed to parse Square from string");
            println!("set_en_passant: {}{} - epcapturesqid: {}, epcapturesquare: {}",
                        sqid_file, new_rank, ep_capture_sqid, ep_capture_square);
            Some(ep_capture_square)
        };

        // check for en-passant
        if 'P' == piece_type.to_ascii_uppercase() {
            let from_rank = from.to_string().chars().nth(1).unwrap();
            let to_rank = to.to_string().chars().nth(1).unwrap();
            let to_file = to.to_string().chars().nth(0).unwrap();

            // Cast both characters to u32 and find the absolute difference.
            if u32::abs_diff(from_rank as u32, to_rank as u32) == 2 {
                let wsqid;
                if to_file != 'a' {
                    wsqid = get_next_sqid(to, Direction::W).unwrap();
                    let wsquare = Square::from_str(&wsqid).unwrap();
                    let wpiece_opt = self.pieces.get(&wsquare);
                    if wpiece_opt.is_some() {
                        let wpiece = wpiece_opt.unwrap();
                        if wpiece.get_piece_type_as_char().to_ascii_uppercase() == 'P'
                                && wpiece.get_side() != self.turn {
                            // self.capture_square_en_passant = set_en_passant(wsqid);
                            self.capture_square_en_passant = set_en_passant();
                        }
                    }
                }
                let esqid;
                if to_file != 'h' {
                    esqid = get_next_sqid(to, Direction::E).unwrap();
                    let esquare = Square::from_str(&esqid).unwrap();
                    let epiece_opt = self.pieces.get(&esquare);
                    if epiece_opt.is_some() {
                        let epiece = epiece_opt.unwrap();
                        if epiece.get_piece_type_as_char().to_ascii_uppercase() == 'P'
                                && epiece.get_side() != self.turn {
                            // self.capture_square_en_passant = set_en_passant(esqid);
                            self.capture_square_en_passant = set_en_passant();
                        }
                    }
                }
            }
        }

        if self.turn == Side::White {
            self.turn = Side::Black;
        } else {
            self.turn = Side::White;
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

    pub fn init_from_pid_list(&mut self, pids: Vec<&str>) {
        self.occupied = 0;
        for pid in pids {
            self.create_and_place_piece(&pid.to_string());
        }
    }
        
    pub fn init_double_discovered_check(&mut self) {
        self.occupied = 0;
        self.create_and_place_piece("e1K");
        self.create_and_place_piece("a6k");
        self.create_and_place_piece("a5P");
        self.create_and_place_piece("b7p");
        self.create_and_place_piece("a1R");
        self.create_and_place_piece("g2B");
        self.create_and_place_piece("g1B");
        self.create_and_place_piece("d8N");
    }

    pub fn init_custom_from(&mut self) {
        self.occupied = 0;
        self.create_and_place_piece("e1K");
        self.create_and_place_piece("e2Q");
        self.create_and_place_piece("e3R");
        self.create_and_place_piece("e4R");

        self.create_and_place_piece("b2B");
        self.create_and_place_piece("c6N");
        self.create_and_place_piece("e5P");
        self.create_and_place_piece("g4n");
        self.create_and_place_piece("g7b");

        self.create_and_place_piece("e6r");
        self.create_and_place_piece("e7r");
        self.create_and_place_piece("e8q");
        self.create_and_place_piece("f8k");
        self.create_and_place_piece("f6p");
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
        let mut checks = String::new();
        match self.checks.len() {
            2 => {
                let f_check = self.checks[0].to_string();
                let s_check = self.checks[1].to_string();              
                checks = format!("[{f_check},{s_check}]");
            }
            1 => {
                let f_check = self.checks[0].to_string();
                checks = format!("[{f_check}]");
            }
            _ => {
                format!("_");
            }
        }
        writeln!(&mut out, "occupied: {}, moves: {:?}, turn: {:?}, checks: {}, en_passant: {:?}",
                    self.occupied, self.moves, self.turn, checks, self.capture_square_en_passant);
        for (_square, piece) in &self.pieces {
            writeln!(&mut out, "{}", piece).unwrap();
        }
        out
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ordered_string())
    }
}



pub fn move_framework(pids: Vec<&str>, moves: Vec<(Square, Square)>) {

    println!("=== Using move_test_framework for position: {pids:?} and moves: {moves:?} ===");
    
    let mut board = Board::new();
    board.init_from_pid_list(pids);
    board.build_all_xchngrs();
    println!("Original board:\n{board}");
    
    // let next_board = board.clone();
    // let prpsd_board = board.clone();

    for (from,  to) in moves {
        let mut next_board = board.clone();
        next_board = next_board
            .full_process_move(from, to);

        println!("Maximally processed exchangers post move:\n{}", next_board);
        
        let mut prpsd_board = board.clone();
        prpsd_board = prpsd_board
            .pre_processed_move(from, to);

        println!("Pre-processed exchangers post move:\n{}", prpsd_board);
    
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
            println!("next_board:\n {}", next_board);
            println!("prpsd_board:\n {}", prpsd_board);
        }
        
        assert_eq!(
            next_str, prpsd_str,
            "Board string representations differ, see diff above"
        );
        
        println!("=================================== TEST PASSED! ====================================");

        // board = prpsd_board;
        board = next_board.clone();
    }
}







#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Square::*;

    fn move_test_framework(pids: Vec<&str>, moves: Vec<(Square, Square)>) {

        println!("=== Using move_test_framework for position: {pids:?} and moves: {moves:?} ===");
        
        let mut board = Board::new();
        board.init_from_pid_list(pids);
        board.build_all_xchngrs();
        
        for (from,  to) in moves {
            let next_board = board
                .full_process_move(from, to);
            println!("Maximally processed exchangers post move:\n {}", next_board);
            
            let prpsd_board = board
                .pre_processed_move(from, to);
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
                println!("next_board:\n {}", next_board);
                println!("\nprpsd_board:\n {}", prpsd_board);
            }
            
            assert_eq!(
                next_str, prpsd_str,
                "Board string representations differ, see diff above"
            );
            
            println!("=== Test passed! ===");

            board = prpsd_board;
        }
    }

    #[test]
    fn test_double_discovered_mate_move_sequence() {
        move_test_framework(
            vec!["e1K", "a6k", "a5P", "b7p", "a1R", "g2B", "g1B", "d8N"],
            vec![(g2, f1), (b7, b5)]
        );
    }
}
