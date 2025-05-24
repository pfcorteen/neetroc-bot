use strum_macros::EnumIter;
//use strum::IntoEnumIterator;

// Define the groupings of direction numbers (0 to 15)
// pub type DirectionNumber = u64;

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
        = vec![Direction::N,Direction::E,Direction::S,Direction::W]
            .iter().cloned().collect();
    pub static ref ORDINALS: Vec<Direction>
        = vec![Direction::NE,Direction::SE,Direction::SW,Direction::NW]
            .iter().cloned().collect();
    pub static ref HALF_WINDS: Vec<Direction>
        = vec![Direction::NNE,Direction::ENE,Direction::ESE,Direction::SSE,Direction::SSW,Direction::WSW,Direction::WNW,Direction::NNW]
            .iter().cloned().collect();
    pub static ref HORIZONTALS: Vec<Direction>
        = vec![Direction::E,Direction::W]
            .iter().cloned().collect();
    pub static ref VERTICALS: Vec<Direction> 
        = vec![Direction::N,Direction::S]
            .iter().cloned().collect();
}