use crate::compass_groups::Direction;
use crate::compass_groups::Direction::*;
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock as Lazy;
use std::vec::Vec;
use std::fmt;
use strum::IntoEnumIterator;

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
            'K' | 'k' => Some(BasicPieceType::King),
            'Q' | 'q' => Some(BasicPieceType::Queen),
            'R' | 'r' => Some(BasicPieceType::Rook),
            'B' | 'b' => Some(BasicPieceType::Bishop),
            'N' | 'n' => Some(BasicPieceType::Knight),
            'P' | 'p' => Some(BasicPieceType::Pawn),
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

#[derive(Debug)]
pub struct PieceTypeData {
    pub basic_piece_type: BasicPieceType,
    pub side: Side,
    pub is_sliding: bool,
    pub directions: Vec<Direction>,
}

pub static WHITE_KING_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::King,
    side: Side::White,
    is_sliding: false,
    directions: [N, NE, E, SE, S, SW, W, NW].to_vec(),
});
pub static BLACK_KING_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::King,
    side: Side::Black,
    is_sliding: false,
    directions: [N, NE, E, SE, S, SW, W, NW].to_vec(),
});
pub static WHITE_QUEEN_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Queen,
    side: Side::White,
    is_sliding: true,
    directions: [N, NE, E, SE, S, SW, W, NW].to_vec(),
});
pub static BLACK_QUEEN_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Queen,
    side: Side::Black,
    is_sliding: true,
    directions: [N, NE, E, SE, S, SW, W, NW].to_vec(),
});
pub static WHITE_ROOK_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Rook,
    side: Side::White,
    is_sliding: true,
    directions: [N, E, S, W].to_vec(),
});
pub static BLACK_ROOK_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Rook,
    side: Side::Black,
    is_sliding: true,
    directions: [N, E, S, W].to_vec(),
});
pub static WHITE_BISHOP_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Bishop,
    side: Side::White,
    is_sliding: true,
    directions: [NE, SE, SW, NW].to_vec(),
});
pub static BLACK_BISHOP_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Bishop,
    side: Side::Black,
    is_sliding: true,
    directions: [NE, SE, SW, NW].to_vec(),
});
pub static WHITE_KNIGHT_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Knight,
    side: Side::White,
    is_sliding: false,
    directions: [NNE, ENE, ESE, SSE, SSW, WSW, WNW, NNW].to_vec(),
});
pub static BLACK_KNIGHT_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Knight,
    side: Side::Black,
    is_sliding: false,
    directions: [NNE, ENE, ESE, SSE, SSW, WSW, WNW, NNW].to_vec(),
});
pub static WHITE_PAWN_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Pawn,
    side: Side::White,
    is_sliding: false,
    directions: [N, NE, NW].to_vec(),
});
pub static BLACK_PAWN_DATA: Lazy<PieceTypeData> = Lazy::new(|| PieceTypeData {
    basic_piece_type: BasicPieceType::Pawn,
    side: Side::Black,
    is_sliding: false,
    directions: [S, SW, SE].to_vec(),
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

    // pub fn get_piece_type_data(c: char) -> Option<&'static PieceType> {
    pub fn get_piece_type(c: char) -> Option<&'static PieceType> {
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

#[derive(Debug, Clone)]
pub struct Piece {
    pub pid: String,
    pub exchangers: HashMap<Direction, String>,
}
impl Piece {
    pub(crate) fn new(piece_id: &str) -> Option<Self> {
        let pid_regex = Regex::new(r"^[a-h][1-8][PpNnBbRrQqKk]$").unwrap();
        if piece_id.len() == 3 && pid_regex.is_match(piece_id) {
            Some(Piece {
                pid: piece_id.to_string(),
                exchangers: HashMap::new(),
            })
        } else {
            None
        }
    }
    pub fn get_piece_data(&self) -> &'static PieceTypeData {
        let piece_type_char = self.get_piece_type_as_char();
        let piece_type_ref = PieceType::get_piece_type(piece_type_char);
        if let Some(piece_type) = piece_type_ref {
            piece_type.get_data()
        } else {
            panic!("Invalid piece type")
        }
    }

    pub fn get_pid(&self) -> &str {
        &self.pid
    }
    pub fn get_square(&self) -> &str {
        &self.pid[0..2]
    }
    pub fn get_piece_type_as_char(&self) -> char {
        self.pid.chars().nth(2).unwrap()
    }
    pub fn get_piece_side(&self) -> Side {
        match self.pid.chars().nth(2).unwrap().is_uppercase() {
            true => Side::White,
            _ => Side::Black,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} |", self.pid)?;
        let mut xchngr_strs = Vec::new();
        for dir in Direction::iter() {
            if let Some(xchngrs) = self.exchangers.get(&dir) {
                xchngr_strs.push(format!("{}:{}", dir.as_ref(), xchngrs));
            }
        }
        let exch_joined = xchngr_strs.join(", ");
        if !exch_joined.is_empty() {
            write!(f, " {}", exch_joined)?;
        }
        Ok(())
    }
}
