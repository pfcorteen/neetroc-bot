// Define the groupings of direction numbers (0 to 15)
pub type DirectionNumber = u8;

use lazy_static::lazy_static;


lazy_static! {
    // Assuming a consistent mapping where 0 is North and we proceed clockwise
    pub static ref CARDINALS: Vec<DirectionNumber> = vec![0, 4, 8, 12].iter().cloned().collect();
    pub static ref ORDINALS: Vec<DirectionNumber> = vec![2, 6, 10, 14].iter().cloned().collect();
    pub static ref HALF_WINDS: Vec<DirectionNumber> = vec![1, 3, 5, 7, 9, 11, 13, 15].iter().cloned().collect();
    pub static ref HORIZONTALS: Vec<DirectionNumber> = vec![4, 12].iter().cloned().collect();
    pub static ref VERTICALS: Vec<DirectionNumber> = vec![0, 8].iter().cloned().collect();
}