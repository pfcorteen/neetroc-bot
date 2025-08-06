use neetroc_bot::board::Board;
use neetroc_bot::board::Square::*;

fn main() {
    let mut board = Board::new();
    // board.init_standard();
    board.init_custom_from();
    board.build_all_xchngrs();
    println!("Main board has {} pieces", board.len());
    println!("Main board:");
    println!("{}", board); // NNB too heavy for debug runs

    if let Some(mut next_board) = board.full_process_move(e5, f6) {
        next_board.build_all_xchngrs();
        println!("Next board has {} pieces", next_board.len());
        println!("Next board:");
        println!("{}", next_board); // NNB too heavy for debug runs

        if let Some(prpsd_board) = board.pre_processed_move(e5, f6) {
            let next_str = next_board.to_ordered_string();
            let prpsd_str = prpsd_board.to_ordered_string();
            if next_str == prpsd_str {
                println!("Test passed!");
            } else {
                println!("Test failed! Diff:");
                let next_lines: Vec<_> = next_str.lines().collect();
                let prpsd_lines: Vec<_> = prpsd_str.lines().collect();
                let max_len = next_lines.len().max(prpsd_lines.len());
                for i in 0..max_len {
                    let n = next_lines.get(i).unwrap_or(&"");
                    let p = prpsd_lines.get(i).unwrap_or(&"");
                    if n != p {
                        println!("- Next:   {}", n);
                        println!("+ Prpsd:  {}", p);
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
