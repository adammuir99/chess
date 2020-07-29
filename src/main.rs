use ggez;
use nalgebra;
use std::env;
use std::path;

use ggez::input;
use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, DrawMode, Color, MeshBuilder, DrawParam};

use chess::{Board, MoveGen};

use nalgebra as na;
type Point2 = na::Point2<f32>;

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

struct MainState {
	pos_x: f32,
	pos_y: f32,
	mouse_down:bool,
	assets: Assets,
	board: Board
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
		let assets = Assets::new(ctx)?;

		let board = Board::default();
		let combined = board.color_combined(chess::Color::White);
		println!("{:?}", combined);
		let movegen = MoveGen::new_legal(&board);
		assert_eq!(movegen.len(), 20);

        Ok (MainState {
            pos_x: 100.0,
            pos_y: 100.0,
			mouse_down: false,
			assets: assets,
			board: board,
		})
	}

	fn draw_board(&self, ctx: &mut Context) -> GameResult<()>{
		let (tile_width, tile_height) = self.tile_size(ctx);
		let mut mb = MeshBuilder::new();
		for row in 0..8 {
			for col in 0..8 {
				let color = if row % 2 == 0 && col % 2 == 0 {
					graphics::Color::from_rgb(255, 255, 226)	//Light
				} else if row % 2 != 0 && col % 2 != 0{
					graphics::Color::from_rgb(255, 255, 226)	//Light
				} else {
					graphics::Color::from_rgb(209, 147, 65)		//Dark
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

	fn draw_pieces(&self, ctx: &mut Context, board: &Board) -> GameResult<()> {
		let  mut bitboard = *board.pieces(chess::Piece::Pawn);
		//Iterate over each square in the bitboard
		for square in bitboard {
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

	fn square_to_coordinate(&self, ctx: &mut Context, square: chess::Square) -> (f32, f32){
		let (tile_width, tile_height) = self.tile_size(ctx);
		let (x, y) = match square {
			chess::Square::A1 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			chess::Square::A2 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			chess::Square::A3 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			chess::Square::A4 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			chess::Square::A5 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			chess::Square::A6 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			chess::Square::A7 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			chess::Square::A8 => (tile_width * 0.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			chess::Square::B1 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			chess::Square::B2 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			chess::Square::B3 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			chess::Square::B4 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			chess::Square::B5 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			chess::Square::B6 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			chess::Square::B7 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			chess::Square::B8 => (tile_width * 1.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			chess::Square::C1 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			chess::Square::C2 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			chess::Square::C3 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			chess::Square::C4 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			chess::Square::C5 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			chess::Square::C6 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			chess::Square::C7 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			chess::Square::C8 => (tile_width * 2.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			chess::Square::D1 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			chess::Square::D2 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			chess::Square::D3 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			chess::Square::D4 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			chess::Square::D5 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			chess::Square::D6 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			chess::Square::D7 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			chess::Square::D8 => (tile_width * 3.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			chess::Square::E1 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			chess::Square::E2 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			chess::Square::E3 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			chess::Square::E4 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			chess::Square::E5 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			chess::Square::E6 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			chess::Square::E7 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			chess::Square::E8 => (tile_width * 4.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			chess::Square::F1 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			chess::Square::F2 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			chess::Square::F3 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			chess::Square::F4 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			chess::Square::F5 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			chess::Square::F6 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			chess::Square::F7 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			chess::Square::F8 => (tile_width * 5.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			chess::Square::G1 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			chess::Square::G2 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			chess::Square::G3 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			chess::Square::G4 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			chess::Square::G5 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			chess::Square::G6 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			chess::Square::G7 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			chess::Square::G8 => (tile_width * 6.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			chess::Square::H1 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 7.0 + (tile_height - 93.0) / 2.0),
			chess::Square::H2 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 6.0 + (tile_height - 93.0) / 2.0),
			chess::Square::H3 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 5.0 + (tile_height - 93.0) / 2.0),
			chess::Square::H4 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 4.0 + (tile_height - 93.0) / 2.0),
			chess::Square::H5 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 3.0 + (tile_height - 93.0) / 2.0),
			chess::Square::H6 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 2.0 + (tile_height - 93.0) / 2.0),
			chess::Square::H7 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 1.0 + (tile_height - 93.0) / 2.0),
			chess::Square::H8 => (tile_width * 7.0 + (tile_width - 93.0) / 2.0, tile_height * 0.0 + (tile_height - 93.0) / 2.0),

			_ => (-1.0, -1.0),
		};
		(x, y)
	}
	
	//Calculate the size of each tile
	fn tile_size(&self, ctx: &mut Context) -> (f32, f32) {
		let (width, height) = graphics::drawable_size(ctx);
		((width / 8.0), (height / 8.0))
	}
}

impl ggez::event::EventHandler for MainState {
	//Called upon each logic update to the game. This should be where the game's logic takes place.
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		Ok(())
	}
	//Called to do the drawing of your game.
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, [0.36, 0.20, 0.09, 1.0].into());
		self.draw_board(ctx)?;
		self.draw_pieces(ctx, &self.board)?;
        let circle = graphics::Mesh::new_circle(
            ctx,
			DrawMode::fill(),
			Point2::new(self.pos_x, self.pos_y),
			10.0,
			1.0,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &circle, DrawParam::default())?;
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
	fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32){
		self.mouse_down = true;
		println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
	}
	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32){
		self.mouse_down = false;
		println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
	}
}

fn main() -> GameResult{	

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
	.window_setup(conf::WindowSetup::default().title("Adam's shitty chess engine!"))
	.window_mode(conf::WindowMode::default().dimensions(800.0, 800.0))
	.add_resource_path(sprite_dir);
	
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}