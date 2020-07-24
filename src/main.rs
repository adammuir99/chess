use chess::{Board, MoveGen};

fn main(){
	let board = Board::default();
	let movegen = MoveGen::new_legal(&board);
	assert_eq!(movegen.len(), 20);
}