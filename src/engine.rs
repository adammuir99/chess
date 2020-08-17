use chess::{Board, MoveGen, Square, ChessMove, BoardStatus, EMPTY};

pub fn ai_move(board: &mut Board){
    //create an iterable
    let mut iterable = MoveGen::new_legal(&board);

    // lets iterate over targets.
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);

    // count the number of targets
    let mut count = 0;
    for test in &mut iterable {
        //println!("{} to {}", test.get_source(), test.get_dest());
        count += 1;
        let m = ChessMove::new(test.get_source(), test.get_dest(), None);
        let mut result = Board::default();
        board.make_move(m, &mut result);
        *board = result;
        break;
        // This move captures one of my opponents pieces (with the exception of en passant)
    }

    // now, iterate over the rest of the moves
    if count == 0{
        iterable.set_iterator_mask(!EMPTY);
        for test in &mut iterable {
            //println!("{} to {}", test.get_source(), test.get_dest());
            let m = ChessMove::new(test.get_source(), test.get_dest(), None);
            let mut result = Board::default();
            board.make_move(m, &mut result);
            *board = result;
            break;
            // This move does not capture anything
        }
    }
}