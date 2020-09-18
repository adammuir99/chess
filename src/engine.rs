use chess::{Board, MoveGen, Square, ChessMove};
mod negamax;
mod alphabeta;

// Base function that generates the best move
pub fn ai_move(board: &mut Board) -> ChessMove{
    //let m = best_immediate(board);
    //let m = negamax::negamax_root(*board);
    let m = alphabeta::alphabeta_root(*board, -std::i32::MAX, std::i32::MAX);
    let mut result = Board::default();
    board.make_move(m, &mut result);
    *board = result;
    
    return m
}

// This function will return the best immediate move.
// It does not take into account opponents response
// just the highest value capture/quiet move
fn best_immediate(board: &mut Board) -> ChessMove{
    let take = best_capture(board);
    if take == None {
        best_quiet(board)
    } else {
        take.unwrap()   //Return the take
    }
}

// Iterates through all the possible captures and
// returns the capture with the highest value
fn best_capture(board: &mut Board) -> Option<ChessMove>{
    //create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // iterate over opponent's pieces.
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);

    let mut value: Option<f32> = None;
    let mut source: Square = Square::A1;
    let mut dest: Square = Square::A2;
    let mut promotion = None;
    for take in &mut iterable {
        let new_value = Some(capture_value(board, take.get_dest()).abs());
        println!("{} to {}", take.get_source(), take.get_dest());
        if new_value > value || value == None{
            value = new_value;
            source = take.get_source();
            dest = take.get_dest();
            promotion = take.get_promotion();
        }
    };

    if value != None {
        Some(ChessMove::new(source, dest, promotion))
    } else {
        // If there are no captures return Option<None>
        None
    }
}

// Returns the value of the piece on a particular
// square. (For future add heuristics)
fn capture_value(board: &mut Board, square: Square) -> f32 {
    let captured_piece = board.piece_on(square);
    let mut value: f32;
    match captured_piece {
        Some(chess::Piece::Pawn)    => value = 10.0,
        Some(chess::Piece::Knight)  => value = 30.0,
        Some(chess::Piece::Bishop)  => value = 30.0,
        Some(chess::Piece::Rook)    => value = 50.0,
        Some(chess::Piece::Queen)   => value = 90.0,
        Some(chess::Piece::King)    => value = 900.0,
        None                        => value = 0.0
    };

    //Black pieces have negative values by convention
    if board.color_on(square) == Some(chess::Color::Black) {
        value = value * -1.0;
    }

    value   //Return
}

// Currently returns a random quiet move
fn best_quiet(board: &mut Board) -> ChessMove {
    // create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // Get a bitboard of all the unoccupied squares
    iterable.set_iterator_mask(!*board.combined());

    let mut value: Option<f32> = None;
    let mut source: Square = Square::A1;
    let mut dest: Square = Square::A2;
    let mut promotion = None;
    for quiet in &mut iterable {
        let new_value = Some(quiet_value(board, 
                                         quiet.get_source(), 
                                         quiet.get_dest()
                                         ).abs());
                                         
        if new_value > value || value == None{
            value = new_value;
            source = quiet.get_source();
            dest = quiet.get_dest();
            promotion = quiet.get_promotion();
        }
    };

    ChessMove::new(source, dest, promotion)
}

fn quiet_value(board: &mut Board, source: Square, dest: Square) -> f32 {
    1.0
}