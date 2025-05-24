use std::collections::HashMap;
use std::vec::Vec;
use crate::compass_groups::Direction::*;
use crate::compass_groups::{ Direction };
use regex::Regex;
use std::sync::LazyLock as Lazy;

#[derive(Debug, PartialEq)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum BasicPieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight, 
    Pawn,
}

impl BasicPieceType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'K' => Some(BasicPieceType::King),
            'k' => Some(BasicPieceType::King),
            'Q' => Some(BasicPieceType::Queen),
            'q' => Some(BasicPieceType::Queen),
            'R' => Some(BasicPieceType::Rook),
            'r' => Some(BasicPieceType::Rook),  
            'B' => Some(BasicPieceType::Bishop),
            'b' => Some(BasicPieceType::Bishop),
            'N' => Some(BasicPieceType::Knight),
            'n' => Some(BasicPieceType::Knight),
            'P' => Some(BasicPieceType::Pawn),
            'p' => Some(BasicPieceType::Pawn),
            _ => None,
        }
    }
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
    pub directions: Vec<Direction>
}

pub static WHITE_KING_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: false, directions: vec![ N, NE, E, SE, S, SW, W, NW ]}
});
pub static BLACK_KING_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: false, directions: vec![ N, NE, E, SE, S, SW, W, NW ]}
});
pub static WHITE_QUEEN_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: true, directions: vec![ N, NE, E, SE, S, SW, W, NW ]}
});
pub static BLACK_QUEEN_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: true, directions: vec![ N, NE, E, SE, S, SW, W, NW ]}
});
pub static WHITE_ROOK_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: true, directions: vec![ N, E, S, W ]}
});
pub static BLACK_ROOK_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: true, directions: vec![ N, E, S, W ]}
});
pub static WHITE_BISHOP_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: true, directions: vec![ NE, SE, SW, NW ]}
});
pub static BLACK_BISHOP_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: true, directions: vec![ NE, SE, SW, NW ]}
});pub static WHITE_KNIGHT_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: false, directions: vec![ NNE, ENE, ESE, SSE, SSW, WSW, WNW, NNW ]}
});
pub static BLACK_KNIGHT_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: false, directions: vec![ NNE, ENE, ESE, SSE, SSW, WSW, WNW, NNW ]}
});pub static WHITE_PAWN_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::White, sliding: false, directions: vec![ N, NE, NW ]}
});
pub static BLACK_PAWN_DATA: Lazy<PieceTypeData> = Lazy::new(|| {
    PieceTypeData {side: Side::Black, sliding: false, directions: vec![ S, SW, SE ]}
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
    pub pid: String,
    pub exchangers: HashMap<Direction, String>,
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
    pub fn get_piece_type_as_char(&self) -> char { self.pid.chars().nth(2).unwrap() }
}

