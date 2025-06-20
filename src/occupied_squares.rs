// src/OccupiedSquare.rs
use crate::compass_groups::{Direction, HALF_WINDS};
use regex::Regex; // You might want to move this here if only used here


pub static FILES: &str = "abcdefgh";
pub static RANKS: &str = "12345678";
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
    println!("{origin} {direction:?} ray: {ray}");
}

pub fn generate_ray_path(origin: &str, direction: Direction, occupied: u64) -> Option<String> {
    let mut empty_count: u8 = 0; // Changed from usize to u8
    let mut path = String::new();

    if let Some(start_bit) = square_to_bit(origin) {
        let mut current_bit = start_bit;
        let (shift, edge_check): (i8, Box<dyn Fn(u64) -> bool>) = match direction {
            Direction::N => (8, Box::new(|b| b <= 55)),
            Direction::NNE => (17, Box::new(|b| b <= 46 && (b % 8) != 7)),
            Direction::NE => (9, Box::new(|b| b <= 54 && (b % 8) != 7)),
            Direction::ENE => (10, Box::new(|b| b <= 53 && (b % 8) < 6)),
            Direction::E => (1, Box::new(|b| b <= 62 && (b % 8) != 7)),
            Direction::ESE => (-6, Box::new(|b| b >= 8 && (b % 8) < 6)),
            Direction::SE => (-7, Box::new(|b| b >= 8 && (b % 8) != 7)),
            Direction::SSE => (-15, Box::new(|b| b >= 16 && (b % 8) != 7)),

            // Direction::SSE => (-15, Box::new(|b| b >= 16 && (b % 8) < 8)),
            Direction::S => (-8, Box::new(|b| b >= 8)),
            Direction::SSW => (-17, Box::new(|b| b >= 17 && (b % 8) != 0)),
            Direction::SW => (-9, Box::new(|b| b >= 8 && (b % 8) != 0)),
            Direction::WSW => (-10, Box::new(|b| b >= 10 && (b % 8) > 1)),
            Direction::W => (-1, Box::new(|b| (b % 8) != 0)),
            Direction::WNW => (6, Box::new(|b| b <= 55 && (b % 8) > 1)),
            Direction::NW => (7, Box::new(|b| b <= 55 && (b % 8) != 0)),
            Direction::NNW => (15, Box::new(|b| b <= 47 && (b % 8) != 0)),
        };

        let mut underscore_available = true;
        while edge_check(current_bit) {
            let next_bit = (current_bit as i64 + shift as i64) as u64;
            if (occupied & (1u64 << next_bit)) != 0 {
                if empty_count > 0 && underscore_available {
                    path.push('_');
                    empty_count = 0;
                    underscore_available = false;
                }
                if let Some(square_string) = &bit_to_square(next_bit) {
                    path.push_str(square_string);
                    underscore_available = false;
                }
            } else {
                empty_count += 1;
            }

            current_bit = next_bit;

            if HALF_WINDS.contains(&direction) {
                break;
            }
        }
    } else {
        return None;
    }

    if !path.is_empty() { Some(path) } else { None }
}


pub fn get_next_sqid(osqid: &str, drctn: Direction) -> Option<String> {
    if osqid.len() != 2 {
        return None;
    }

    let sqid_file = osqid.chars().next().expect("Expected a file character");
    let sqid_rank = osqid.chars().nth(1).expect("Expected a rank character");

    let calc_sqid =
        |rlmt_opt: Option<i8>, flmt_opt: Option<i8>, roffset: i8, foffset: i8| -> Option<String> {
            let mut rank = sqid_rank; // default rank
            let mut file = sqid_file; // default file
            if let Some(rlmt) = rlmt_opt {
                let ridx_opt: Option<usize> = RANKS.find(sqid_rank);
                let ridx: i8 = ridx_opt.unwrap() as i8;
                if ridx == rlmt {
                    return None; // too close to edge of board
                }
                rank = RANKS.chars().nth((ridx + roffset) as usize).unwrap();
            } // None of rlmt -> use default rank

            if let Some(flmt) = flmt_opt {
                let fidx_opt: Option<usize> = FILES.find(sqid_file);
                let fidx: i8 = fidx_opt.unwrap() as i8;
                if fidx == flmt {
                    return None; // too close to edge of board
                }
                file = FILES.chars().nth((fidx + foffset) as usize).unwrap();
            } // None of flmt -> use default file

            let sqid = [file, rank];
            Some(sqid.iter().collect())
        };

    match drctn {
        Direction::N => calc_sqid(Some(7), None, 1, 0),
        Direction::NNE => calc_sqid(Some(6), Some(7), 2, 1),
        Direction::NE => calc_sqid(Some(7), Some(7), 1, 1),
        Direction::ENE => calc_sqid(Some(7), Some(6), 1, 2),
        Direction::E => calc_sqid(None, Some(7), 0, 1),
        Direction::ESE => calc_sqid(Some(0), Some(6), -1, 2),
        Direction::SE => calc_sqid(Some(0), Some(7), -1, 1),
        Direction::SSE => calc_sqid(Some(1), Some(7), -2, 1),
        Direction::S => calc_sqid(Some(0), None, -1, 0),
        Direction::SSW => calc_sqid(Some(1), Some(0), -2, -1),
        Direction::SW => calc_sqid(Some(0), Some(0), -1, -1),
        Direction::WSW => calc_sqid(Some(0), Some(1), -1, -2),
        Direction::W => calc_sqid(None, Some(0), 0, -1),
        Direction::WNW => calc_sqid(Some(7), Some(1), 1, -2),
        Direction::NW => calc_sqid(Some(7), Some(0), 1, -1),
        Direction::NNW => calc_sqid(Some(6), Some(0), 2, -1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compass_groups::Direction;

    #[test]
    fn test_piece_moves() {
        let tsq = "a1";
        let sq_opt = get_next_sqid(tsq, Direction::N);
        if let Some(sq) = sq_opt {
            assert!(sq == "a2");
        } else {
            println!("{sq_opt:?} was None");
        }

        let tsq = "d4";
        let answers = ["e6", "f5", "f3", "e2", "c2", "b3", "b5", "c6"];
        for (cnt, drctn) in HALF_WINDS.iter().enumerate() {
            let sq_opt = get_next_sqid(tsq, *drctn);
            let answr = answers[cnt];
            if let Some(sq) = sq_opt {
                println!("{drctn:?} from {tsq} = {sq}, {answr:?}");
                assert!(sq == answr);
            } else {
                println!("{drctn:?} from {tsq} = {sq_opt:?}, {answr:?}");
            }
        }
    }

    #[test]
    fn test_knight_movements() {
        println!("Entered test_knight_movements");

        let mut bit_board = 0u64;
        let squares = [
            "a1", "b3", "c2", "a8", "b6", "c7", "h1", "f2", "g3", "h8", "f7", "g6", "a4", "a5",
            "h4", "h5", "d1", "e1", "d8", "e8",
        ];
        let mut path_count = 0;
        let mut none_count = 0;

        for sq in squares {
            if let Some(bit) = square_to_bit(sq) {
                bit_board |= 1u64 << bit;
            }
        }
        for sq in squares {
            for drctn in HALF_WINDS.iter() {
                let path_opt = generate_ray_path(sq, *drctn, bit_board);
                match path_opt {
                    None => {
                        none_count += 1;
                        println!("No ray path for {sq} {drctn:?}, count: {none_count}");
                    }
                    Some(path) => {
                        path_count += 1;
                        println!("Testing {sq} {drctn:?} got path {path}, count: {path_count}");
                    }
                }
            }
        }

        assert!(path_count == 32);
        assert!(none_count == 128);
    }

    fn test_knight_move(
        origin: &str,
        direction: Direction,
        expected_target: &str,
        should_succeed: bool,
    ) {
        let mut occupied = 0u64;
        // Set both origin and target as occupied
        if let Some(origin_bit) = square_to_bit(origin) {
            occupied |= 1u64 << origin_bit;
        }
        if let Some(target_bit) = square_to_bit(expected_target) {
            occupied |= 1u64 << target_bit;
        }

        let path_opt = generate_ray_path(origin, direction, occupied);
        match path_opt {
            None => {
                println!("No ray path for {origin} {direction:?} to {expected_target}");
            }
            Some(path) => {
                println!("Testing {origin} {direction:?} to {expected_target}: got path '{path}'");
                if should_succeed {
                    assert!(
                        !path.is_empty(),
                        "Path should not be empty for valid knight move"
                    );
                    assert!(
                        path.contains(expected_target),
                        "Path should contain target square"
                    );
                } else {
                    assert!(
                        path.is_empty(),
                        "Path should be empty for invalid knight move"
                    );
                }
            }
        }
    }

    #[test]
    fn test_all_knight_moves() {
        // NNE moves (2 up, 1 right)
        test_knight_move("b1", Direction::ENE, "c3", true);
        test_knight_move("a1", Direction::NNE, "b3", true); // Should fail (can't start from a-file)

        // ENE moves (2 right, 1 up)
        test_knight_move("a1", Direction::ENE, "c2", true);
        test_knight_move("g1", Direction::NNE, "h3", true); // Should fail (not enough space to right)

        // ESE moves (2 right, 1 down)
        test_knight_move("a8", Direction::ESE, "c7", true);
        test_knight_move("g8", Direction::ESE, "h6", false); // Should fail (not enough space to right)

        // SSE moves (2 down, 1 right)
        test_knight_move("a8", Direction::SSE, "b6", true);
        test_knight_move("h8", Direction::SSE, "g6", false); // Should fail (can't start from h-file)

        // SSW moves (2 down, 1 left)
        test_knight_move("h8", Direction::SSW, "g6", true);
        test_knight_move("a8", Direction::SSW, "b6", false); // Should fail (can't start from a-file)

        // WSW moves (2 left, 1 down)
        test_knight_move("h8", Direction::WSW, "f7", true);
        test_knight_move("a8", Direction::WSW, "c7", false); // Should fail (not enough space to left)

        // WNW moves (2 left, 1 up)
        test_knight_move("h1", Direction::WNW, "f2", true);
        test_knight_move("a1", Direction::WNW, "c2", false); // Should fail (not enough space to left)

        // NNW moves (2 up, 1 left)
        test_knight_move("h1", Direction::NNW, "g3", true);
        test_knight_move("a1", Direction::NNW, "b3", false); // Should fail (can't start from a-file)
    }
}
