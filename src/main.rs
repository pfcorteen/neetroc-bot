use neetroc_bot::board::move_framework;
// use neetroc_bot::board::Board;
use neetroc_bot::board::Square::*;
fn main() {
    // let mut board = Board::new();
    // // board.init_standard();
    // board.init_double_discovered_check();
    // board.build_all_xchngrs();
    // println!("Main board:");
    // println!("{:?}", board); // NNB too heavy for debug runs

    move_framework(
        vec!["e1K", "a6k", "a5P", "b7p", "a1R", "g2B", "g1B", "d8N"],
        vec![(g2, f1), (b7, b5), (a5, b6)]
    );
}
