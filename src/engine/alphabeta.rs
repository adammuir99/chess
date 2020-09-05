use chess::{Board, MoveGen, ChessMove};
use rand::Rng;

pub fn alphabeta_root(board: Board, mut alpha: i32, beta: i32) -> ChessMove {
   let depth = 3;
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
      } else if value == alpha {
         best_moves.push(chessmove);
      }
   };

   // If there are multiple moves with the same value, take a random one
   let best = best_moves[rand::thread_rng().gen_range(0, best_moves.len())];
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
   let color_modifier = if color {1} else {-1};

    if depth == 0 { return evaluate(board) * color_modifier }

    //create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // iterate over every move
    let targets = board.color_combined(board.side_to_move());
    iterable.set_iterator_mask(!*targets);

    for chessmove in iterable {
        let board_copy = board.make_move_new(chessmove);
        let value = -alphabeta(board_copy, depth - 1, -beta, -alpha, !color);
        if value > alpha { alpha = value; }
        if value >= beta { return value }
    };
    return alpha
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