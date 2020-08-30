use chess::{Board, MoveGen, Square, ChessMove, BoardStatus, EMPTY};

/* int negaMax( int depth ) {
    if ( depth == 0 ) return evaluate();
    int max = -oo;
    for ( all moves)  {
        score = -negaMax( depth - 1 );
        if( score > max )
            max = score;
    }
    return max;
} */
pub fn negamax(board: &mut Board, depth: u32) -> i32{
    if depth == 0 { return evaluate(*board) }

    let mut max = -10000;

    //create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // iterate over every move
    let targets = board.color_combined(board.side_to_move());
    iterable.set_iterator_mask(!*targets);

    for _ in iterable {
        let score = -negamax(board, depth - 1);
        if score > max {
            max = score;
        }
    };
    max
}

fn evaluate(board: Board) -> i32 {
    let mut value2 = 0;
    value2 += get_piece_balance(board, chess::Piece::Pawn);
    value2 += get_piece_balance(board, chess::Piece::Bishop);
    value2 += get_piece_balance(board, chess::Piece::Knight);
    value2 += get_piece_balance(board, chess::Piece::Rook);
    value2 += get_piece_balance(board, chess::Piece::Queen);
    value2 += get_piece_balance(board, chess::Piece::King);

    value2
}

fn get_piece_balance(board: Board, piece: chess::Piece) -> i32 {
    let mut color = board.color_combined(chess::Color::White);
    let pieces = board.pieces(piece);
    let mut value = 0;

    let cost = match piece {
        chess::Piece::Pawn => 100,
        chess::Piece::Bishop => 330,
        chess::Piece::Knight => 320,
        chess::Piece::Rook => 500,
        chess::Piece::Queen => 900,
        chess::Piece::King => 20000,
    };

    //Get the binary representation of all {color} {pieces}
    let n = *pieces & *color;

    let mut num = n.to_size(0).count_ones() as i32; // Get the number of 1's in the binary number (and cast from u32 to i32)

    value += num * cost;

    // Check black pieces
    color = board.color_combined(chess::Color::Black);

    //Get the binary representation of all {color} {pieces}
    let n = *pieces & *color;

    num = n.to_size(0).count_ones() as i32; // Get the number of 1's in the binary number (and cast from u32 to i32)

    value -= num * cost;

    return value
}