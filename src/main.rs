use ggez;
use nalgebra;
use std::env;
use std::path;
use std::time::Instant;

use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Color, MeshBuilder, DrawParam};

use chess::{Board, Square, ChessMove, BoardStatus, EMPTY};

//Import external modules
mod engine;
mod tests;

use nalgebra as na;
type Point2 = na::Point2<f32>;

//Sprites for each piece
struct Assets {
	black_pawn: graphics::Image,
	black_rook: graphics::Image,
	black_bishop: graphics::Image,
	black_knight: graphics::Image,
	black_queen: graphics::Image,
	black_king: graphics::Image,

	white_pawn: graphics::Image,
	white_rook: graphics::Image,
	white_bishop: graphics::Image,
	white_knight: graphics::Image,
	white_queen: graphics::Image,
	white_king: graphics::Image,
}

impl Assets {
	fn new(ctx: &mut Context) -> GameResult<Assets> {
		let black_pawn = graphics::Image::new(ctx, "/black_pawn.png")?;
		let black_rook = graphics::Image::new(ctx, "/black_rook.png")?;
		let black_bishop = graphics::Image::new(ctx, "/black_bishop.png")?;
		let black_knight = graphics::Image::new(ctx, "/black_knight.png")?;
		let black_queen = graphics::Image::new(ctx, "/black_queen.png")?;
		let black_king = graphics::Image::new(ctx, "/black_king.png")?;

		let white_pawn = graphics::Image::new(ctx, "/white_pawn.png")?;
		let white_rook = graphics::Image::new(ctx, "/white_rook.png")?;
		let white_bishop = graphics::Image::new(ctx, "/white_bishop.png")?;
		let white_knight = graphics::Image::new(ctx, "/white_knight.png")?;
		let white_queen = graphics::Image::new(ctx, "/white_queen.png")?;
		let white_king = graphics::Image::new(ctx, "/white_king.png")?;

		Ok(Assets {
            black_pawn,
			black_rook,
			black_bishop,
			black_knight,
			black_queen,
			black_king,
			white_pawn,
			white_rook,
			white_bishop,
			white_knight,
			white_queen,
			white_king,
        })
	}
}

struct Remember{
	curr_pressed_square: Square,
	last_pressed_square: Square,
	released_square: Square,
	display_last_move: bool		// only false at the start of the game
}

impl Remember {
	fn initialize() -> Remember{
		Remember {
			curr_pressed_square: Square::A2,
			last_pressed_square: Square::A3,
			released_square: Square::A1,
			display_last_move: false,
		}
	}
}

struct MainState {
	pos_x: f32,
	pos_y: f32,
	mouse_down: bool,
	assets: Assets,
	board: Board,
	remember: Remember,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
		let assets = Assets::new(ctx)?;
		let remember = Remember::initialize();
		let board = Board::default();

        Ok (MainState {
            pos_x: 100.0,
            pos_y: 100.0,
			mouse_down: false,
			assets: assets,
			board: board,
			remember: remember,
		})
	}

	//Draw the 8x8 board
	fn draw_board(&self, ctx: &mut Context) -> GameResult<()>{
		let (tile_width, tile_height) = self.tile_size(ctx);
		let mut mb = MeshBuilder::new();
		for row in 0..8 {
			for col in 0..8 {
				let color = if row % 2 == 0 && col % 2 == 0 {
					Color::from_rgb(255, 255, 226)	//Light
				} else if row % 2 != 0 && col % 2 != 0{
					Color::from_rgb(255, 255, 226)	//Light
				} else {
					Color::from_rgb(209, 147, 65)	//Dark
				};
				let (row, col) = (row as f32, col as f32);
				mb.rectangle(
					graphics::DrawMode::fill(),
					graphics::Rect::new(row * tile_width, col * tile_height, tile_width, tile_height),
					color
				);
			}
		}
		let mut mesh = mb.build(ctx)?;
		graphics::draw(ctx, &mut mesh, DrawParam::default())
	}

	//Draw the highlighted square when the user clicks
	fn draw_highlights(&self, ctx: &mut Context) -> GameResult<()>{
		let (tile_width, tile_height) = self.tile_size(ctx);
		let mut mb = MeshBuilder::new();

		//Highlight pressed square
		let (mut x, mut y) = self.square_to_coordinate(ctx, self.remember.curr_pressed_square);
		x = x - (tile_width - 93.0) / 2.0;
		y = y - (tile_height - 93.0) / 2.0;

		let mut color = Color::from_rgb(245, 247, 109);	//Highlight color

		mb.rectangle(
			graphics::DrawMode::fill(),
			graphics::Rect::new(x, y, tile_width, tile_height),
			color
		);
		
		//Highlight released square
		let (mut x2, mut y2) = self.square_to_coordinate(ctx, self.remember.released_square);
		x2 = x2 - (tile_width - 93.0) / 2.0;
		y2 = y2 - (tile_height - 93.0) / 2.0;

		mb.rectangle(
			graphics::DrawMode::fill(),
			graphics::Rect::new(x2, y2, tile_width, tile_height),
			color
		);
		
		//Highlight previous move
		let (mut x3, mut y3) = self.square_to_coordinate(ctx, self.remember.last_pressed_square);
		x3 = x3 - (tile_width - 93.0) / 2.0;
		y3 = y3 - (tile_height - 93.0) / 2.0;

		mb.rectangle(
			graphics::DrawMode::fill(),
			graphics::Rect::new(x3, y3, tile_width, tile_height),
			color
		);

		//Highlight checked piece
		if *self.board.checkers() != EMPTY{	//if the bitboard of pieces putting me in check is NOT empty
			let (mut x4, mut y4) = self.square_to_coordinate(ctx, self.board.king_square(self.board.side_to_move()));	//Get the square of the moving player's king
			x4 = x4 - (tile_width - 93.0) / 2.0;
			y4 = y4 - (tile_height - 93.0) / 2.0;

			color = Color::from_rgb(255, 50, 50);	//Color red

			mb.rectangle(
				graphics::DrawMode::fill(),
				graphics::Rect::new(x4, y4, tile_width, tile_height),
				color
			);
		}

		let mut mesh = mb.build(ctx)?;
		graphics::draw(ctx, &mut mesh, DrawParam::default())
	}
	//Reads all the 'pieces' bitboards, checks which color is on the square and draws the sprite
	fn draw_pieces(&self, ctx: &mut Context, board: &Board) -> GameResult<()> {
		let  mut bitboard = *board.pieces(chess::Piece::Pawn);
		//Iterate over each square in the bitboard
		for square in bitboard {
			//IF you are holding a piece draw it in-hand (not on-board)
			if square == self.remember.curr_pressed_square && self.mouse_down {
				if board.color_on(square) == Some(chess::Color::Black){
					graphics::draw(ctx, &self.assets.black_pawn, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				} else {
					graphics::draw(ctx, &self.assets.white_pawn, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				}
				//Skip this iteration
				continue;
			}
			//Now draw the pieces to the board
			if board.color_on(square) == Some(chess::Color::Black){
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.black_pawn, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			} else {
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.white_pawn, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			}
		}

		bitboard = *board.pieces(chess::Piece::Rook);
		for square in bitboard {
			//IF you are holding a piece draw it in-hand (not on-board)
			if square == self.remember.curr_pressed_square && self.mouse_down{
				if board.color_on(square) == Some(chess::Color::Black){
					graphics::draw(ctx, &self.assets.black_rook, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				} else {
					graphics::draw(ctx, &self.assets.white_rook, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				}
				//Skip this iteration
				continue;
			}
			//Now draw the pieces to the board
			if board.color_on(square) == Some(chess::Color::Black){
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.black_rook, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			} else {
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.white_rook, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			}
		}

		bitboard = *board.pieces(chess::Piece::Bishop);
		for square in bitboard {
			//IF you are holding a piece draw it in-hand (not on-board)
			if square == self.remember.curr_pressed_square && self.mouse_down{
				if board.color_on(square) == Some(chess::Color::Black){
					graphics::draw(ctx, &self.assets.black_bishop, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				} else {
					graphics::draw(ctx, &self.assets.white_bishop, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				}
				//Skip this iteration
				continue;
			}
			//Now draw the pieces to the board
			if board.color_on(square) == Some(chess::Color::Black){
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.black_bishop, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			} else {
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.white_bishop, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			}
		}

		bitboard = *board.pieces(chess::Piece::Knight);
		for square in bitboard {
			//IF you are holding a piece draw it in-hand (not on-board)
			if square == self.remember.curr_pressed_square && self.mouse_down{
				if board.color_on(square) == Some(chess::Color::Black){
					graphics::draw(ctx, &self.assets.black_knight, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				} else {
					graphics::draw(ctx, &self.assets.white_knight, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				}
				//Skip this iteration
				continue;
			}
			//Now draw the pieces to the board
			if board.color_on(square) == Some(chess::Color::Black){
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.black_knight, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			} else {
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.white_knight, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			}
		}

		bitboard = *board.pieces(chess::Piece::Queen);
		for square in bitboard {
			//IF you are holding a piece draw it in-hand (not on-board)
			if square == self.remember.curr_pressed_square && self.mouse_down{
				if board.color_on(square) == Some(chess::Color::Black){
					graphics::draw(ctx, &self.assets.black_queen, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				} else {
					graphics::draw(ctx, &self.assets.white_queen, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				}
				//Skip this iteration
				continue;
			}
			//Now draw the pieces to the board
			if board.color_on(square) == Some(chess::Color::Black){
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.black_queen, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			} else {
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.white_queen, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			}
		}

		bitboard = *board.pieces(chess::Piece::King);
		for square in bitboard {
			//IF you are holding a piece draw it in-hand (not on-board)
			if square == self.remember.curr_pressed_square && self.mouse_down{
				if board.color_on(square) == Some(chess::Color::Black){
					graphics::draw(ctx, &self.assets.black_king, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				} else {
					graphics::draw(ctx, &self.assets.white_king, graphics::DrawParam::new().dest(Point2::new(self.pos_x - 46.5, self.pos_y - 46.5)))?;
				}
				//Skip this iteration
				continue;
			}
			//Now draw the pieces to the board
			if board.color_on(square) == Some(chess::Color::Black){
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.black_king, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			} else {
				let (x, y) = self.square_to_coordinate(ctx, square);
				graphics::draw(ctx, &self.assets.white_king, graphics::DrawParam::new().dest(Point2::new(x, y)))?;
			}
		}

		Ok(())
	}

	fn draw_gamestate(&self, ctx: &mut Context, gamestate: String) {
		let text = ggez::graphics::Text::new(ggez::graphics::TextFragment {
            // `TextFragment` stores a string, and optional parameters which will override those
            // of `Text` itself. This allows inlining differently formatted lines, words,
            // or even individual letters, into the same block of text.
            text: gamestate.to_string(),
            color: Some(Color::new(1.0, 0.0, 0.0, 1.0)),
            // `Font` is a handle to a loaded TTF, stored inside the `Context`.
            // `Font::default()` always exists and maps to DejaVuSerif.
            font: Some(graphics::Font::default()),
            scale: Some(ggez::graphics::Scale::uniform(150.0)),
            // This doesn't do anything at this point; can be used to omit fields in declarations.
            ..Default::default()
		});
		
		graphics::draw(ctx, &text, DrawParam::default().dest(nalgebra::Point2::new(25.0, 320.0)));
	}

	fn square_to_coordinate(&self, ctx: &mut Context, square: Square) -> (f32, f32){
		let (tile_width, tile_height) = self.tile_size(ctx);
		let (x, y) = match square {
			Square::A1 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			Square::A2 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			Square::A3 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			Square::A4 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			Square::A5 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			Square::A6 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			Square::A7 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			Square::A8 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			Square::B1 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			Square::B2 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			Square::B3 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			Square::B4 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			Square::B5 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			Square::B6 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			Square::B7 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			Square::B8 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			Square::C1 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			Square::C2 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			Square::C3 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			Square::C4 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			Square::C5 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			Square::C6 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			Square::C7 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			Square::C8 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			Square::D1 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			Square::D2 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			Square::D3 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			Square::D4 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			Square::D5 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			Square::D6 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			Square::D7 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			Square::D8 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			Square::E1 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			Square::E2 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			Square::E3 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			Square::E4 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			Square::E5 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			Square::E6 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			Square::E7 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			Square::E8 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			Square::F1 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			Square::F2 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			Square::F3 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			Square::F4 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			Square::F5 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			Square::F6 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			Square::F7 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			Square::F8 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			Square::G1 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			Square::G2 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			Square::G3 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			Square::G4 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			Square::G5 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			Square::G6 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			Square::G7 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			Square::G8 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			Square::H1 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			Square::H2 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			Square::H3 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			Square::H4 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			Square::H5 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			Square::H6 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			Square::H7 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			Square::H8 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			_ => (-1.0, -1.0),
		};
		(x, y)
	}

	fn coordinate_to_square(&self, ctx: &mut Context, (x, y): (f32, f32)) -> Square {
		let (tile_width, tile_height) = self.tile_size(ctx);
		let file = if x >= 0.0 && x < tile_width {
			chess::File::A
		} else if x >= tile_width && x < tile_width*2.0 {
			chess::File::B
		} else if x >= tile_width*2.0 && x < tile_width*3.0 {
			chess::File::C
		} else if x >= tile_width*3.0 && x < tile_width*4.0 {
			chess::File::D
		} else if x >= tile_width*4.0 && x < tile_width*5.0 {
			chess::File::E
		} else if x >= tile_width*5.0 && x < tile_width*6.0 {
			chess::File::F
		} else if x >= tile_width*6.0 && x < tile_width*7.0 {
			chess::File::G
		} else if x >= tile_width*7.0 && x < tile_width*8.0 {
			chess::File::H
		} else {
			chess::File::A	//Should never reach this case
		};

		let rank = if y >= 0.0 && y < tile_height {
			chess::Rank::Eighth
		} else if y >= tile_height && y < tile_height*2.0 {
			chess::Rank::Seventh
		} else if y >= tile_height*2.0 && y < tile_height*3.0 {
			chess::Rank::Sixth
		} else if y >= tile_height*3.0 && y < tile_height*4.0 {
			chess::Rank::Fifth
		} else if y >= tile_height*4.0 && y < tile_height*5.0 {
			chess::Rank::Fourth
		} else if y >= tile_height*5.0 && y < tile_height*6.0 {
			chess::Rank::Third
		} else if y >= tile_height*6.0 && y < tile_height*7.0 {
			chess::Rank::Second
		} else if y >= tile_height*7.0 && y < tile_height*8.0 {
			chess::Rank::First
		} else {
			chess::Rank::First	//Should never reach this case
		};

		Square::make_square(rank, file)
	}
	
	//Calculate the size of each tile
	fn tile_size(&self, ctx: &mut Context) -> (f32, f32) {
		let (width, height) = graphics::drawable_size(ctx);
		((width / 8.0), (height / 8.0))
	}
}

impl ggez::event::EventHandler for MainState {
	//Called upon each logic update to the game. This should be where the game's logic takes place.
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		//assert_eq!(self.board.status(), BoardStatus::Ongoing);
		if self.board.side_to_move() == chess::Color::Black {
			let timer = Instant::now();
			let m = engine::ai_move(&mut self.board);
			println!("Time to calculate move: {:.2?}", timer.elapsed());
			self.remember.released_square = m.get_source();
			self.remember.last_pressed_square = m.get_dest();
		}
		Ok(())
	}
	//Called to do the drawing of your game.
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		//graphics::clear(ctx, [0.36, 0.20, 0.09, 1.0].into());
		self.draw_board(ctx)?;
		if self.remember.display_last_move{
			self.draw_highlights(ctx)?;
		}
		self.draw_pieces(ctx, &self.board)?;

		if self.board.status() == BoardStatus::Checkmate{
			self.draw_gamestate(ctx, "Checkmate".to_string());
		} else if self.board.status() == BoardStatus::Stalemate {
			self.draw_gamestate(ctx, " Stalemate".to_string());
		}
		
		graphics::present(ctx)?;
		
		Ok(())
	}

	//Handle mouse inputs
	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32){
		if self.mouse_down {
			self.pos_x = x;
			self.pos_y = y;
		}
	}
	fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32){
		self.mouse_down = true;
		self.remember.curr_pressed_square = self.coordinate_to_square(ctx, (x, y));
	}
	fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32){
		self.mouse_down = false;
		let released;
		//Check for release outside the window
		let (tile_width, tile_height) = self.tile_size(ctx);
		if x < 0.0 || x > (tile_width * 8.0) || y < 0.0 || y > (tile_height * 8.0){
			//Do nothing
		} else {	//User released inside the window
			released = self.coordinate_to_square(ctx, (x, y));
			
			if released != self.remember.curr_pressed_square{
				//Determine whether the piece needs to be promoted
				let promotion = if self.board.piece_on(released) == Some(chess::Piece::Pawn)
								&& (released.get_rank() == chess::Rank::First
								|| released.get_rank() == chess::Rank::Eighth) {
					Some(chess::Piece::Queen)
				} else {
					None
				};
				let m = ChessMove::new(self.remember.curr_pressed_square,
					released,
					promotion);
				//If the suggested move is legal, make the move and update the board
				if self.board.legal(m) {
					self.remember.curr_pressed_square = released;	//For highlighting
					self.remember.display_last_move = true;			//Allow highlights
					let mut result = Board::default();
					self.board.make_move(m, &mut result);
					self.board = result;
				}
			}
		}
	}
}

fn main() -> GameResult{
	//tests
	tests::eval_speed_test();

	//Add path of sprite folder
	let sprite_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("sprites");
        path
    } else {
        path::PathBuf::from("./sprites")
	};
	
	println!("Adding path {:?}", sprite_dir);

	let cb = ggez::ContextBuilder::new("Chess", "Adam Muir")
	.window_setup(conf::WindowSetup::default().title("Adam's chess engine!"))
	.window_mode(conf::WindowMode::default().dimensions(800.0, 800.0))
	.add_resource_path(sprite_dir);
	
	let (ctx, event_loop) = &mut cb.build()?;
	
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}