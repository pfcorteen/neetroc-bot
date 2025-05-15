use neetroc_bot::pieces::{ Piece, PieceType, Side };
use neetroc_bot::{compass_groups, board::Board};
use neetroc_bot::occupied_squares::{print_ray_string, generate_ray_path};

fn main() {
    // let mut map = ChessMap::new();


    // println!("Map: {:?}", map);
    
    // Print compass groups
    println!("Cardinal directions: {:?}", *compass_groups::CARDINALS);
    println!("Ordinal directions: {:?}", *compass_groups::ORDINALS);
    println!("Half-Wind directions: {:?}", *compass_groups::HALF_WINDS);
    println!("Horizontal directions: {:?}", *compass_groups::HORIZONTALS);
    println!("Vertical directions: {:?}", *compass_groups::VERTICALS);

    // Create and initialize a standard chess board
    println!("\n--- Standard Chess Board Setup ---");
    let mut board = Board::new();
    board.initialize_standard();
    board.build_all_xchngrs();
    
    // Print the board state
    println!("Chess board initialized with {} pieces", board.len());
    
    // Iterate through all pieces and print their positions
    // for (square, piece) in board.iter_pieces() {
    //     println!("Square {}: {}, sliding: {}", square, piece.get_piece_fen(), piece.is_sliding());
    // }

    // Print the bitboard representation
    println!("\n--- Bitboard Representation ---");
    println!("Occupied bitboard value: {:064b}", board.get_occupied_bitboard());
    board.print_occupied_squares();

    // Test ray-casting in different directions
    println!("\n--- Ray Casting Tests ---");
    
    // Test rays from different positions
    let test_positions = [
        ("e4", 0),  // North from center
        ("e4", 4),  // East from center
        ("e4", 8),  // South from center
        ("e4", 12), // West from center
        ("a1", 2),  // NorthEast from corner
        ("h8", 10), // SouthWest from corner
        ("d5", 14), // NorthWest from middle
        ("f3", 6),  // SouthEast from middle
        ("b1", 1),  // Knight direction to empty square
        ("g8", 11), // knight direction to occupied square
    ];
    
    // SEE board.rs for correct usage of these functions = thanks Gemini
    // for (origin, direction) in test_positions.iter() {
    //     let path = generate_ray_path(origin, *direction, board.get_occupied_bitboard());
    //     print_ray_string(origin, *direction, &path);
    //     println!();
    // }

    // let my_char = 'q';
    // if let Some(piece_type) = PieceType::from_char(my_char) {
    //     let data = piece_type.get_data();
    //     println!("For char '{}' (via enum): Side: {:?}, sliding: {}, direction: {:?}",
    //              my_char, data.side, data.sliding, data.directions);
    // } else {
    //     println!("No variant found for char '{}'", my_char);
    // }
}

