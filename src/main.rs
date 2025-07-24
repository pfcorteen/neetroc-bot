use neetroc_bot::board::Board;
use neetroc_bot::board::Square::*;

fn main() {
    let mut board = Board::new();
    // board.init_standard();
    board.init_custom_from();
    board.build_all_xchngrs();
    println!("Main board has {} pieces", board.len());
    println!("Main board: {board:?}"); // NNB too heavy for debug runs

    if let Some(mut next_board) = board.full_process_move(e5, f6) {
        next_board.build_all_xchngrs();
        println!("Next board has {} pieces", next_board.len());
        println!("Next board: {next_board:?}"); // NNB too heavy for debug runs
    }

    if let Some(prpsd_board) = board.process_move(e5, f6) {
        // prpsd_board.build_new_xchngrs();
        println!("Proposed board has {} pieces", prpsd_board.len());
        // println!("Proposed board: {prpsed_board:?}"); NNB too heavy for debug runs
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
