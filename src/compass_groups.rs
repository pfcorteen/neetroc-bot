use strum_macros::EnumIter;
use crate::board::{Square};

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Copy, Clone)] // These are useful traits to derive
pub enum Direction { N, NNE, NE, ENE, E, ESE, SE, SSE, S, SSW, SW, WSW, W, WNW, NW, NNW }

impl Direction {
    /// Returns the opposite direction for a given 16-point compass direction.
    pub fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::NNE => Direction::SSW,
            Direction::NE => Direction::SW,
            Direction::ENE => Direction::WSW,
            Direction::E => Direction::W,
            Direction::ESE => Direction::WNW,
            Direction::SE => Direction::NW,
            Direction::SSE => Direction::NNW,
            Direction::S => Direction::N,
            Direction::SSW => Direction::NNE,
            Direction::SW => Direction::NE,
            Direction::WSW => Direction::ENE,
            Direction::W => Direction::E,
            Direction::WNW => Direction::ESE,
            Direction::NW => Direction::SE,
            Direction::NNW => Direction::SSE,
        }
    }
}

use lazy_static::lazy_static;
lazy_static! {
    pub static ref CARDINALS: Vec<Direction>
        = [Direction::N,Direction::E,Direction::S,Direction::W].to_vec();
    pub static ref ORDINALS: Vec<Direction>
        = [Direction::NE,Direction::SE,Direction::SW,Direction::NW].to_vec();
    pub static ref HALF_WINDS: Vec<Direction>
        = [Direction::NNE,Direction::ENE,Direction::ESE,Direction::SSE,Direction::SSW,
            Direction::WSW,Direction::WNW,Direction::NNW].to_vec();
    pub static ref HORIZONTALS: Vec<Direction>
        = [Direction::E,Direction::W].to_vec();
    pub static ref VERTICALS: Vec<Direction> 
        = [Direction::N,Direction::S].to_vec();
}

pub fn get_direction(from: Square, to: Square) -> Option<Direction> {
    fn parse_square(square: &str) -> Option<(isize, isize)> {
        let mut chars = square.chars();
        let file_char = chars.next()?;
        let rank_char = chars.next()?;
    
        if chars.next().is_some()
            || !file_char.is_ascii_alphabetic()
                || !rank_char.is_ascii_digit() {
            return None;
        }
    
        let file = (file_char.to_ascii_lowercase() as isize) - ('a' as isize);
        let rank = (rank_char.to_digit(10)? as isize) - 1;
    
        if (0..=7).contains(&file) && (0..=7).contains(&rank) {
            Some((file, rank))
        } else {
            None
        }
    }
    
    let from_str: &str = from.as_ref();
    let to_str: &str = to.as_ref();

    let (from_file, from_rank) = parse_square(from_str)?;
    let (to_file, to_rank) = parse_square(to_str)?;

    let file_diff = to_file - from_file;
    let rank_diff = to_rank - from_rank;

    // Use a match statement to handle all move vectors.
    let direction = match (file_diff, rank_diff) {
        // --- Knight Moves (Half-Winds) ---
        (1, 2) => Direction::NNE,
        (2, 1) => Direction::ENE,
        (2, -1) => Direction::ESE,
        (1, -2) => Direction::SSE,
        (-1, -2) => Direction::SSW,
        (-2, -1) => Direction::WSW,
        (-2, 1) => Direction::WNW,
        (-1, 2) => Direction::NNW,

        // --- Standard Linear & Diagonal Moves ---
        (file_diff, rank_diff)
            if file_diff.abs() == rank_diff.abs() || file_diff == 0 || rank_diff == 0 => {
                match (file_diff.signum(), rank_diff.signum()) {
                    (0, 1) => Direction::N,
                    (1, 1) => Direction::NE,
                    (1, 0) => Direction::E,
                    (1, -1) => Direction::SE,
                    (0, -1) => Direction::S,
                    (-1, -1) => Direction::SW,
                    (-1, 0) => Direction::W,
                    (-1, 1) => Direction::NW,
                    _ => return None,
                }
        }

        // All other moves are indeterminate.
        _ => return None,
    };
    Some(direction)
}