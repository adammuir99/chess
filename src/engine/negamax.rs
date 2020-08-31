use chess::{Board, MoveGen, Square, ChessMove, BoardStatus, EMPTY};

//
pub fn negamax_root(board: Board) -> ChessMove {
    let depth = 4;
    let mut best_moves = Vec::new();
    let mut max = -1000000;

    //create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // iterate over every move
    let targets = board.color_combined(board.side_to_move());
    iterable.set_iterator_mask(!*targets);

    for chessmove in iterable {
        let m = ChessMove::new(chessmove.get_source(), chessmove.get_dest(), chessmove.get_promotion());
        let board_copy = board.make_move_new(m);
        let score = -negamax(board_copy, depth - 1, true);

        if score > max {
            max = score;
            best_moves.clear();
            best_moves.push(chessmove)
        }
    };

    let best = best_moves[0];
    return best
    
}


fn negamax(board: Board, depth: u32, color: bool) -> i32{

    let color_modifier = if color {1} else {-1};

    if depth == 0 { return evaluate(board) * color_modifier }

    let mut max = -1000000;

    //create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // iterate over every move
    let targets = board.color_combined(board.side_to_move());
    iterable.set_iterator_mask(!*targets);

    for chessmove in iterable {
        let m = ChessMove::new(chessmove.get_source(), chessmove.get_dest(), chessmove.get_promotion());
        let board_copy = board.make_move_new(m);
        let score = -negamax(board_copy, depth - 1, !color);
        if score > max {
            max = score;
        }
    };
    return max
}

fn evaluate(board: Board) -> i32 {
    let mut value = 0;
    value += get_piece_balance(board, chess::Piece::Pawn);
    value += get_piece_balance(board, chess::Piece::Bishop);
    value += get_piece_balance(board, chess::Piece::Knight);
    value += get_piece_balance(board, chess::Piece::Rook);
    value += get_piece_balance(board, chess::Piece::Queen);
    value += get_piece_balance(board, chess::Piece::King);

    value
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