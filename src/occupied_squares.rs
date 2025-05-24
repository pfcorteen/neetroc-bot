// src/OccupiedSquare.rs
use regex::Regex; // You might want to move this here if only used here
use crate::compass_groups::{Direction, HALF_WINDS};
//use crate::compass_groups::Direction::*;

// Function to convert a square string (e.g., "a1") to a bit position (0-63)
pub fn square_to_bit(square: &str) -> Option<u64> {
    if square.len() != 2 {
        return None;
    }

    let square_regex = Regex::new(r"^[a-h][1-8]$").unwrap();
    if !square_regex.is_match(square) {
        return None;
    }

    let file = square.chars().next().unwrap() as u8 - b'a';
    let rank = square.chars().nth(1).unwrap() as u8 - b'1';

    Some((rank * 8 + file) as u64)
}

// Function to convert a bit position (0-63) back to a square string (e.g., "a1")
pub fn bit_to_square(bit: u64) -> Option<String> {
    if bit > 63 {
        return None;
    }

    let file = (bit % 8) as u8 + b'a';
    let rank = (bit / 8) as u8 + b'1';

    Some(format!("{}{}", file as char, rank as char))
}

pub fn print_ray_string(origin: &str, direction: Direction, ray: &str) {
    println!("{} {:?} ray: {}", origin, direction, ray);
}

pub fn generate_ray_path(origin: &str, direction: Direction, occupied: u64) -> String {
    let mut empty_count: u8 = 0;  // Changed from usize to u8
    let mut path = String::new();
    
    if let Some(start_bit) = square_to_bit(origin) {
        let mut current_bit = start_bit;
        let (shift, edge_check): (i8, Box<dyn Fn(u64) -> bool>) = match direction {
            Direction::N => (8, Box::new(|b| b <= 55)),
            Direction::NNE => (17, Box::new(|b| b <= 47 && b % 8 != 0)),
            Direction::NE => (9, Box::new(|b| b <= 55 && b % 8 != 7)),
            Direction::ENE => (10, Box::new(|b| b <= 40 && b % 8 > 1)),
            Direction::E => (1, Box::new(|b| b % 8 != 7)),
            Direction::ESE => (-6, Box::new(|b| b >= 16 && b % 8 < 7)), // Corrected
            Direction::SE => (-7, Box::new(|b| b >= 8 && b % 8 != 7)),
            Direction::SSE => (-15, Box::new(|b| b >= 15 && b % 8 < 7)), // Corrected
            Direction::S => (-8, Box::new(|b| b >= 8)),
            Direction::SSW => (-17, Box::new(|b| b >= 17 && b % 8 != 0)), // Corrected
            Direction::SW => (-9, Box::new(|b| b >= 8 && b % 8 != 0)),
            Direction::WSW => (-10, Box::new(|b| b >= 24 && b % 8 > 0)),
            Direction::W => (-1, Box::new(|b| b % 8 != 0)),
            Direction::WNW => (6, Box::new(|b| b <= 57 && b % 8 != 0)), // Corrected
            Direction::NW => (7, Box::new(|b| b <= 55 && b % 8 != 0)),
            Direction::NNW => (15, Box::new(|b| b <= 48 && b % 8 != 0)), // Corrected
        };

        // if the first exchanger encountered is more than one step from the origin
        //  insert an underscore in the returned string to indicate to later process
        //  that only a sliding piece can exchange - non-sliding has to be first in list
        //  and also not more than 1 square from the origin.
        // Keep moving in the direction until we hit a piece or the board edge
        let mut use_underscore = true;
        while edge_check(current_bit) {
            let next_bit = if shift >= 0 {
                (current_bit as i64 + shift as i64) as u64
            } else {
                current_bit.wrapping_sub(shift.unsigned_abs() as u64)
            };
            // println!("Next bit: {}", next_bit);
            // if next_bit >= 0 && next_bit <= 63 {
                if (occupied & (1u64 << next_bit)) != 0 {
                    if empty_count > 0 && use_underscore {
                        path.push_str("_");
                        empty_count = 0;
                        use_underscore = false;
                    }
                    if let Some(square_string) = &bit_to_square(next_bit) {
                        path.push_str(square_string);
                        use_underscore = false;
                    }
                } else {
                    empty_count += 1;
                }

                current_bit = next_bit;
            // }

            if HALF_WINDS.contains(&direction) { // ie odd numbers indicates half-wind so go no further
                break;
            }
        }
    }
    path
}