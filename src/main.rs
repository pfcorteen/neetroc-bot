use neetroc_bot::board::Board;
use neetroc_bot::occupied_squares::square_to_bit;

fn main() {
    let mut board = Board::new();
    board.initialise_standard();
    // board.initialise_custom1();
    board.build_all_xchngrs();

    println!("Board relationships: {board:?}")


}
