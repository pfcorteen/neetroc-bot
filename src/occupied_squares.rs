// src/OccupiedSquare.rs
use regex::Regex; // You might want to move this here if only used here

// Add this new struct definition near the top of the file
//#[derive(Debug)]
// pub struct RayResult {
//     pub empty_squares: u8,        // Changed from usize to u8
//     pub occupied_square: Option<String>
// }

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

pub fn print_ray_string(origin: &str, direction: u8, ray: &str) {
    println!("Ray from {} in direction {}: {}", origin, direction, ray);
}

// Modified function to return the new struct
pub fn generate_ray_path(origin: &str, direction: u8, occupied: u64) -> String {
    let mut empty_count: u8 = 0;  // Changed from usize to u8
    let mut path = String::new();
    
    if let Some(start_bit) = square_to_bit(origin) {
        let mut current_bit = start_bit;
        let (shift, edge_check): (i8, Box<dyn Fn(u64) -> bool>) = match direction {
            0 => (8,    Box::new(|b| b <= 55)), // North
            1 => (17,   Box::new(|b| b <= 47 && b % 8 != 0)), // NNE
            2 => (9,    Box::new(|b| b <= 55 && b % 8 != 7)), // NorthEast
            3 => (10,   Box::new(|b| b <= 40 && b % 8 > 1)), // ENE
            4 => (1,    Box::new(|b| b % 8 != 7)), // East
            5 => (-6,   Box::new(|b| b <= 40 && b % 8 < 7)), // ESE
            6 => (-7,   Box::new(|b| b >= 8 && b % 8 != 7)), // SouthEast
            7 => (-15,  Box::new(|b| b <= 54 && b % 8 < 7)), // SSE
            8 => (-8,   Box::new(|b| b >= 8)), // South
            9 => (-17,  Box::new(|b| b >= 9 && b % 8 < 7)), // SSW
            10 => (-9,  Box::new(|b| b >= 8 && b % 8 != 0)), // SouthWest
            11 => (-10, Box::new(|b| b >= 24 && b % 8 > 0)), // WSW
            12 => (-1,  Box::new(|b| b % 8 != 0)), // West
            13 => (6,   Box::new(|b| b >= 24 && b % 8 < 6)), // WNW
            14 => (7,   Box::new(|b| b <= 55 && b % 8 != 0)), // NorthWest
            15 => (15,  Box::new(|b| b >= 16 && b % 8 > 0)), // NNW
            _ => return path, // Invalid direction
        };

        // if the first exchanger encountered is more than one step from the origin
        //  insert an underscore in the returned string to indicate to later process
        //  that only a sliding piece can exchange - non-sliding has to be first in list
        //  and also not more than 1 square from the origin.
        // Keep moving in the direction until we hit a piece or the board edge
        let mut use_underscore = true;
        while edge_check(current_bit) {
            let next_bit = if shift >= 0 {
                                current_bit + shift as u64
                            } else {
                                current_bit.wrapping_sub(shift.unsigned_abs() as u64)
                            };

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

            if (direction % 2) == 1 { // ie odd numbers indicates half-wind so go no further
                break;
            }
        }
    }
    
    path
}

// Modified function to return the new struct
// pub fn ray_to_first_piece(origin: &str, direction: u8, occupied: u64) -> RayResult {
//     let mut empty_count: u8 = 0;  // Changed from usize to u8
//     let mut final_square = None;
    
//     // Get the starting position
//     if let Some(start_bit) = square_to_bit(origin) {
//         let mut current_bit = start_bit;
        
//         // Define the bit shift based on direction
//         // 0 = North (+8), 2 = NorthEast (+9), 4 = East (+1), 6 = SouthEast (-7),
//         // 8 = South (-8), 10 = SouthWest (-9), 12 = West (-1), 14 = NorthWest (+7)
//         let (shift, edge_check): (i32, Box<dyn Fn(u64) -> bool>) = match direction {
//             0 => (8, Box::new(|b| b <= 55)), // North
//             2 => (9, Box::new(|b| b <= 55 && b % 8 != 7)), // NorthEast
//             4 => (1, Box::new(|b| b % 8 != 7)), // East
//             6 => (-7, Box::new(|b| b >= 8 && b % 8 != 7)), // SouthEast
//             8 => (-8, Box::new(|b| b >= 8)), // South
//             10 => (-9, Box::new(|b| b >= 8 && b % 8 != 0)), // SouthWest
//             12 => (-1, Box::new(|b| b % 8 != 0)), // West
//             14 => (7, Box::new(|b| b <= 55 && b % 8 != 0)), // NorthWest
//             _ => return RayResult { empty_squares: 0, occupied_square: None }, // Invalid direction
//         };

//         // Keep moving in the direction until we hit a piece or the board edge
//         while edge_check(current_bit) {
//             let next_bit = if shift >= 0 {
//                 current_bit + shift as u64
//             } else {
//                 current_bit.wrapping_sub(shift.unsigned_abs() as u64)
//             };
            
//             // Check if the square is occupied
//             let is_occupied = (occupied & (1u64 << next_bit)) != 0;
            
//             if is_occupied {
//                 final_square = bit_to_square(next_bit);
//                 break;
//             }
            
//             empty_count += 1;
//             current_bit = next_bit;
//         }
//     }
    
//     RayResult {
//         empty_squares: empty_count,
//         occupied_square: final_square
//     }
// }

// Modified helper function to print the ray results
// pub fn print_ray_results(origin: &str, direction: u8, ray: &RayResult) {
//     println!("Ray from {} in direction {}:", origin, direction);
//     println!("  Empty squares before piece: {}", ray.empty_squares);
//     match &ray.occupied_square {
//         Some(square) => println!("  Occupied square found: {}", square),
//         None => println!("  No piece found (reached board edge)")
//     }
// }

// fn knight_moves_bitmask(from_square: &str) -> Option<u64> {
//     let from_bit = square_to_bit(from_square)?; // Use ? for concise error handling

//     // Knight move offsets (relative to the bit representation)
//     let offsets: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];

//     let mut bitmask: u64 = 0;

//     for offset in offsets.iter() {
//         // Calculate target square bit position
//         if let Some(target_bit) = from_bit.checked_add_signed(*offset as i64) {
//             if target_bit < 64 {
//                 if let Some(target_square) = bit_to_square(target_bit) {
//                     // Basic validity check: Ensure target square is within the board
//                     if target_square.len() == 2 {
//                         bitmask |= 1 << target_bit;
//                     }
//                 }
//             }
//         }
//     }
// }
// fn generate_sliding_path( origin: &str, direction: u8, occupied: u64, max_distance: u8, // Optional: limit the search distance
// ) -> String {
//     let mut path = String::new();
//     let mut current_square = origin.to_string();
//     let mut distance_travelled: u8 = 8;
//     let mut hit_square: &str;
//     let mut empty_squares: u8;

//     while distance_travelled < max_distance {
//         if let ray_result = ray_to_first_piece(&current_square, direction, occupied) {  // origin: &str, direction: u8, occupied: u64
//             hit_square = ray_result.occupied_square;
//             empty_squares = ray_result.empty_squares;
//             distance_travelled += empty_squares + if hit_square.is_some() { 1 } else { 0 };

//             if empty_squares > 0 {
//                 path.push_str(&empty_squares.to_string());
//             }

//             if let Some(hit_square) = hit_square {
//                 path.push_str(&hit_square);
//                 break; // Stop if we hit a piece
//             }

//             if let Some(next_square) = bit_to_square(square_to_bit(&origin).unwrap() + direction as u64) {
//                 current_square = hit_square;
//             } else {
//                  break;
//             }

//         } else {
//             break; // Stop if search_along_direction returns None (invalid direction or other error)
//         }
//     }

//     path
// }