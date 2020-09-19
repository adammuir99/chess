use chess::{Board, MoveGen, ChessMove};
use std::cmp;
mod pst;

pub fn alphabeta_root(board: Board, mut alpha: i32, beta: i32) -> ChessMove {
   let depth = 5;
   let mut best_moves = Vec::new();

   //create an iterable
   let mut iterable = MoveGen::new_legal(&board);

   // iterate over every move
   let targets = board.color_combined(board.side_to_move());
   iterable.set_iterator_mask(!*targets);

   for chessmove in iterable {
      let board_copy = board.make_move_new(chessmove);
      let value = -alphabeta(board_copy, depth - 1, -beta, -alpha, true);
      if value > alpha { 
         alpha = value;
         best_moves.clear();
         best_moves.push(chessmove);
      }
   };

   // Get the best move
   let best = best_moves[0];
   return best
   
}

/* int alphaBeta( int alpha, int beta, int depthleft ) {
    if( depthleft == 0 ) return quiesce( alpha, beta );
    for ( all moves)  {
       score = -alphaBeta( -beta, -alpha, depthleft - 1 );
       if( score >= beta )
          return beta;   //  fail hard beta-cutoff
       if( score > alpha )
          alpha = score; // alpha acts like max in MiniMax
    }
    return alpha;
 } */
fn alphabeta(board: Board, depth: u32, mut alpha: i32, beta: i32, color: bool) -> i32{
   //If the board status is checkmate, the current player has lost -> return large negative number
   if board.status() == chess::BoardStatus::Checkmate { return -100000 }

   let color_modifier = if color {1} else {-1};

    if depth == 0 { return evaluate(board) * color_modifier }

    let mut value = -std::i32::MAX;

    //create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // iterate over every move
    let targets = board.color_combined(board.side_to_move());
    iterable.set_iterator_mask(!*targets);

    for chessmove in iterable {
        let board_copy = board.make_move_new(chessmove);
        value = cmp::max(-alphabeta(board_copy, depth - 1, -beta, -alpha, !color), value);
        alpha = cmp::max(alpha, value);
        if alpha >= beta {break}
    };
    return value
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
   let mut position_value = 0;

   let cost = match piece {
       chess::Piece::Pawn => 100,
       chess::Piece::Bishop => 330,
       chess::Piece::Knight => 320,
       chess::Piece::Rook => 500,
       chess::Piece::Queen => 900,
       chess::Piece::King => 20000,
   };

   let mut pst_cost = match piece {
      chess::Piece::Pawn => pst::PST_WHITE_PAWN,
      chess::Piece::Bishop => pst::PST_WHITE_BISHOP,
      chess::Piece::Knight => pst::PST_WHITE_KNIGHT,
      chess::Piece::Rook => pst::PST_WHITE_ROOK,
      chess::Piece::Queen => pst::PST_WHITE_QUEEN,
      chess::Piece::King => pst::PST_WHITE_KING,
  };

   //Get the binary representation of all {color} {pieces}
   let n = (*pieces & *color).to_size(0);

   let mut num = n.count_ones() as i32; // Get the number of 1's in the binary number (and cast from u32 to i32)

   value += num * cost;

   let mut n2 = n;

   while n2 != 0 {
      let square = n2.trailing_zeros();   //The number of trailing 0s will give the index of the first 1 (LSB = A1)
      n2 &= n2 - 1;  // ANDing with (itself - 1) will remove the least significant 1
      position_value = position_value + pst_cost[square as usize];
   }

   value = value + position_value;

   // Check black pieces
   position_value = 0;

   let mut pst_cost = match piece {
      chess::Piece::Pawn => pst::PST_BLACK_PAWN,
      chess::Piece::Bishop => pst::PST_BLACK_BISHOP,
      chess::Piece::Knight => pst::PST_BLACK_KNIGHT,
      chess::Piece::Rook => pst::PST_BLACK_ROOK,
      chess::Piece::Queen => pst::PST_BLACK_QUEEN,
      chess::Piece::King => pst::PST_BLACK_KING,
  };

   color = board.color_combined(chess::Color::Black);

   //Get the binary representation of all {color} {pieces}
   let n = (*pieces & *color).to_size(0);

   num = n.count_ones() as i32; // Get the number of 1's in the binary number (and cast from u32 to i32)

   value -= num * cost;

   let mut n2 = n;

   while n2 != 0 {
      let square = n2.trailing_zeros();   //The number of trailing 0s will give the index of the first 1 (LSB = A1)
      n2 &= n2 - 1;  // ANDing with (itself - 1) will remove the least significant 1
      position_value = position_value + pst_cost[square as usize];
   }

   value = value - position_value;

   return value
}