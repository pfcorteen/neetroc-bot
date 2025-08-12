use neetroc_bot::board::Board;
use neetroc_bot::board::Square::*;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let mut board = Board::new();
    // board.init_standard();
    board.init_double_discovered_check();
    board.build_all_xchngrs();
    println!("Main board has {} pieces", board.len());
    println!("Main board:");
    println!("{}", board); // NNB too heavy for debug runs

    if let Some(mut next_board) = board.full_process_move(g2, f1) {
        next_board.build_all_xchngrs();
        println!("Next board has {} pieces", next_board.len());
        println!("Next board:");
        println!("{}", next_board); // NNB too heavy for debug runs

        if let Some(prpsd_board) = board.pre_processed_move(g2, f1) {
            let next_str = next_board.to_ordered_string();
            let prpsd_str = prpsd_board.to_ordered_string();
            if next_str == prpsd_str {
                println!("Test passed!");
            } else {
                println!("Test failed! Diff:");
                // Build key-aligned, normalized maps using the first 3-character pid as key.
                fn extract_pid3(line: &str) -> Option<String> {
                    let pre_pipe = line.split('|').next().unwrap_or("");
                    let pid: String = pre_pipe
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .take(3)
                        .collect();
                    if pid.len() == 3 { Some(pid) } else { None }
                }

                fn normalize_line(line: &str) -> String {
                    let mut parts = line.splitn(2, '|');
                    let left = parts.next().unwrap_or("").trim();
                    let right = parts.next().unwrap_or("");
                    let mut tokens: Vec<String> = right
                        .split(',')
                        .map(|t| t.trim())
                        .filter(|t| !t.is_empty())
                        .map(|t| t.to_string())
                        .collect();
                    tokens.sort();
                    if tokens.is_empty() {
                        left.to_string()
                    } else {
                        format!("{} | {}", left, tokens.join(", "))
                    }
                }

                let mut next_map: BTreeMap<String, (String, String)> = BTreeMap::new();
                for line in next_str.lines().filter(|l| !l.trim().is_empty()) {
                    if let Some(pid3) = extract_pid3(line) {
                        next_map.insert(pid3, (line.to_string(), normalize_line(line)));
                    }
                }

                let mut prpsd_map: BTreeMap<String, (String, String)> = BTreeMap::new();
                for line in prpsd_str.lines().filter(|l| !l.trim().is_empty()) {
                    if let Some(pid3) = extract_pid3(line) {
                        prpsd_map.insert(pid3, (line.to_string(), normalize_line(line)));
                    }
                }

                let all_keys: BTreeSet<String> = next_map
                    .keys()
                    .cloned()
                    .chain(prpsd_map.keys().cloned())
                    .collect();

                for key in all_keys {
                    let n = next_map.get(&key);
                    let p = prpsd_map.get(&key);
                    let n_norm = n.map(|(_, norm)| norm.as_str());
                    let p_norm = p.map(|(_, norm)| norm.as_str());
                    if n_norm != p_norm {
                        println!("Key: {}", key);
                        let n_orig = n.map(|(orig, _)| orig.as_str()).unwrap_or("(missing)");
                        let p_orig = p.map(|(orig, _)| orig.as_str()).unwrap_or("(missing)");
                        println!("- Next:   {}", n_orig);
                        println!("+ Prpsd:  {}", p_orig);
                    }
                }
            }
        }
    }

    if let Some(prpsd_board) = board.pre_processed_move(e5, f6) {
        println!("pre_processed_move board has {} pieces", prpsd_board.len());
        println!("{}", prpsd_board); // NNB too heavy for debug runs
    }

    // test_piece_moves();
    // fn test_piece_moves() {
    //     let tsq = "a1";
    //     let sq_opt = get_next_sqid(tsq, Direction::N);
    //     if let Some(sq) = sq_opt {
    //         assert!(sq == "a2");
    //     } else {
    //         println!("{sq_opt:?} was None");
    //     }

    //     let tsq = "d4";
    //     let answers = ["e6", "f5", "f3", "e2", "c2", "b3", "b5", "c6"];
    //     for (cnt, drctn) in HALF_WINDS.iter().enumerate() {
    //         let sq_opt = get_next_sqid(tsq, *drctn);
    //         let answr = answers[cnt];
    //         if let Some(sq) = sq_opt {
    //             println!("{drctn:?} from {tsq} = {sq}, {answr:?}");
    //             assert!(sq == answr);
    //         } else {
    //             println!("{drctn:?} from {tsq} = {sq_opt:?}, {answr:?}");
    //         }
    //     }
    // }
}
