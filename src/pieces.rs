use std::collections::HashMap;
use std::vec::Vec;
use crate::compass_groups::{DirectionNumber, CARDINALS, ORDINALS, HALF_WINDS };
use regex::Regex;
use std::sync::LazyLock as Lazy;

#[derive(Debug, PartialEq)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum PieceType {
    WhiteKing,
    BlackKing,
    WhiteQueen,
    BlackQueen,
    WhiteRook,
    BlackRook,
    WhiteBishop,
    BlackBishop,
    WhiteKnight,
    BlackKnight,
    WhitePawn,
    BlackPawn,
}

pub struct PieceTypeData {
    pub side: Side,
    pub sliding: bool,
    pub directions: Vec<DirectionNumber>
}

pub static WHITE_KING_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: false, directions: vec![0,2,4,6,8,10,12,14]}
});
pub static BLACK_KING_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: false, directions: vec![0,2,4,6,8,10,12,14]}
});
pub static WHITE_QUEEN_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: true, directions: vec![0,2,4,6,8,10,12,14]}
});
pub static BLACK_QUEEN_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: true, directions: vec![0,2,4,6,8,10,12,14]}
});
pub static WHITE_ROOK_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: true, directions: vec![0,4,8,12]}
});
pub static BLACK_ROOK_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: true, directions: vec![0,4,8,12]}
});
pub static WHITE_BISHOP_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: true, directions: vec![2,6,10,14]}
});
pub static BLACK_BISHOP_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: true, directions: vec![2,6,10,14]}
});pub static WHITE_KNIGHT_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: false, directions: vec![1,3,5,7,9,11,13,15]}
});
pub static BLACK_KNIGHT_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: false, directions: vec![1,3,5,7,9,11,13,15]}
});pub static WHITE_PAWN_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: false, directions: vec![0,2,14]}
});
pub static BLACK_PAWN_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: false, directions: vec![2,6,10]}
});


impl PieceType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'K' => Some(PieceType::WhiteKing),
            'k' => Some(PieceType::BlackKing),
            'Q' => Some(PieceType::WhiteQueen),
            'q' => Some(PieceType::BlackQueen),
            'R' => Some(PieceType::WhiteRook),
            'r' => Some(PieceType::BlackRook),  
            'B' => Some(PieceType::WhiteBishop),
            'b' => Some(PieceType::BlackBishop),
            'N' => Some(PieceType::WhiteKnight),
            'n' => Some(PieceType::BlackKnight),
            'P' => Some(PieceType::WhitePawn),
            'p' => Some(PieceType::BlackPawn),
            _ => None,
        }
    }

    pub fn get_data(&self) -> &'static PieceTypeData {
        match self {
            PieceType::WhiteKing => &WHITE_KING_DATA,
            PieceType::BlackKing => &BLACK_KING_DATA,
            PieceType::WhiteQueen => &WHITE_QUEEN_DATA,
            PieceType::BlackQueen => &BLACK_QUEEN_DATA,
            PieceType::WhiteRook => &WHITE_ROOK_DATA,
            PieceType::BlackRook => &BLACK_ROOK_DATA,
            PieceType::WhiteBishop => &WHITE_BISHOP_DATA,
            PieceType::BlackBishop => &BLACK_BISHOP_DATA,
            PieceType::WhiteKnight => &WHITE_KNIGHT_DATA,
            PieceType::BlackKnight => &BLACK_KNIGHT_DATA,
            PieceType::WhitePawn => &WHITE_PAWN_DATA,
            PieceType::BlackPawn => &BLACK_PAWN_DATA,
        }
    }

    pub fn get_piece_type_data(c: char) -> Option<&'static PieceType> {
        match c {
            'K' => Some(&PieceType::WhiteKing),
            'k' => Some(&PieceType::BlackKing),
            'Q' => Some(&PieceType::WhiteQueen),
            'q' => Some(&PieceType::BlackQueen),
            'R' => Some(&PieceType::WhiteRook),
            'r' => Some(&PieceType::BlackRook),  
            'B' => Some(&PieceType::WhiteBishop),
            'b' => Some(&PieceType::BlackBishop),
            'N' => Some(&PieceType::WhiteKnight),
            'n' => Some(&PieceType::BlackKnight),
            'P' => Some(&PieceType::WhitePawn),
            'p' => Some(&PieceType::BlackPawn),
            _ => None,
        }
    }
}


#[derive(Debug)]
pub struct Piece {
    pid: String,
    exchangers: HashMap<u8, String>,
}
impl Piece {
    pub(crate) fn new(piece_id: &str) -> Option<Self> {
        let pid_regex = Regex::new(r"^[a-h][1-8][PpNnBbRrQqKk]$").unwrap();
        if piece_id.len() == 3 && pid_regex.is_match(piece_id) {
            // let piece_id: [&str; 2] = [square, piece_fen];
            Some(Piece {
                pid: piece_id.to_string(),
                exchangers: HashMap::new()
            })
        } else {
            None
        }
    }
  
    pub fn get_pid(&self) -> &str { &self.pid }
    pub fn get_square(&self) -> &str { &self.pid[0..2] }
    pub fn get_piece_fen(&self) -> &str { &self.pid[2..3] }
    pub fn get_piece_type_as_char(&self) -> char {
        self.pid.chars().nth(2).unwrap()
    }
    // pub fn get_legal_directions (&self) -> Vec<DirectionNumber> {
    //     let directions = match self.get_piece_fen() {
    //         "k" => vec![0, 4, 8, 12, 2, 6, 10, 14].iter().cloned().collect(),
    //         "K" => vec![0, 4, 8, 12, 2, 6, 10, 14].iter().cloned().collect(),
    //         "q" => vec![0, 4, 8, 12, 2, 6, 10, 14].iter().cloned().collect(),
    //         "Q" => vec![0, 4, 8, 12, 2, 6, 10, 14].iter().cloned().collect(),
    //         "r" => vec![0, 4, 8, 12].iter().cloned().collect(),
    //         "R" => vec![0, 4, 8, 12].iter().cloned().collect(),
    //         "b" => vec![2, 6, 10, 14].iter().cloned().collect(),
    //         "B" => vec![2, 6, 10, 14].iter().cloned().collect(),
    //         "n" => vec![1, 3, 5, 7, 9, 11, 13, 15].iter().cloned().collect(),
    //         "N" => vec![1, 3, 5, 7, 9, 11, 13, 15].iter().cloned().collect(),
    //         "p" => vec![8, 6, 10].iter().cloned().collect(),
    //         "P" => vec![0, 2, 14].iter().cloned().collect(),
    //         _ => panic!("invalid piece type indicator"),
    //     };
    //     directions
    // }
    // pub fn get_side(&self) -> Side {
    //     if self.get_piece_fen().chars().next().unwrap().is_uppercase() {
    //         Side::White
    //     } else {
    //         Side::Black
    //     }
    // }
    // pub fn is_sliding(&self) -> bool {
    //     let pf = self.get_piece_fen().chars().next().unwrap().to_ascii_uppercase();
    //     let sliding = match pf {
    //         'Q' | 'R' | 'B' => true,
    //         _ => false
    //     };
    //     sliding
    // }
}

