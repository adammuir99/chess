use chess::{Board, MoveGen, Square, ChessMove, BoardStatus, EMPTY};
use std::time::Instant;

pub fn eval_speed_test(){
    let board = Board::default();

    
    //Method 1: get the piece on every square (check 64 squares)
    let mut _value = 0;
    let timer1 = Instant::now();
    for _ in 0..100000{
        for rank in 0..8 {
            for file in 0..8 {
                let square = chess::Square::make_square(chess::Rank::from_index(rank), chess::File::from_index(file));
                let piece = board.piece_on(square);
                let mut cost = match piece {
                    Some(chess::Piece::Pawn) => 100,
                    Some(chess::Piece::Bishop) => 330,
                    Some(chess::Piece::Knight) => 320,
                    Some(chess::Piece::Rook) => 500,
                    Some(chess::Piece::Queen) => 900,
                    Some(chess::Piece::King) => 20000,
                    None => 0
                };
                if board.color_on(square) == Some(chess::Color::Black){
                    cost = cost * -1;
                }
                _value += cost;
            }
        }
        _value = 0;
    }
    println!("Method 1, Elapsed time: {:.2?}", timer1.elapsed());

    //Method 2: Get the bitboard for each piece and count the bits
    let timer2 = Instant::now();
    for _ in 0..100000{
        let mut _value2 = 0;
        _value2 += get_piece_balance(board, chess::Piece::Pawn);
        _value2 += get_piece_balance(board, chess::Piece::Bishop);
        _value2 += get_piece_balance(board, chess::Piece::Knight);
        _value2 += get_piece_balance(board, chess::Piece::Rook);
        _value2 += get_piece_balance(board, chess::Piece::Queen);
        _value2 += get_piece_balance(board, chess::Piece::King);
    }

    println!("Method 2, Elapsed time: {:.2?}", timer2.elapsed());
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

    let mut num = n.to_size(0).count_ones() as i32;

    value += num * cost;

    // Check black pieces
    color = board.color_combined(chess::Color::Black);

    //Get the binary representation of all {color} {pieces}
    let n = *pieces & *color;

    num = n.to_size(0).count_ones() as i32;

    value -= num * cost;

    return value
}