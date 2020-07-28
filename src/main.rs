use ggez;
use nalgebra;

use ggez::input;
use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, DrawMode, Color, MeshBuilder, DrawParam};

use chess::{Board, MoveGen};

use nalgebra as na;
type Point2 = na::Point2<f32>;

struct MainState {
	pos_x: f32,
	pos_y: f32,
	mouse_down:bool,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            pos_x: 100.0,
            pos_y: 100.0,
            mouse_down: false,
        }
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
					graphics::Color::from_rgb(92, 59, 0)		//Dark
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
	let board = Board::default();
	println!("{:?}", chess::get_rank(chess::Rank::Second));
	let movegen = MoveGen::new_legal(&board);
	assert_eq!(movegen.len(), 20);

	let cb = ggez::ContextBuilder::new("Chess", "Adam Muir")
	.window_setup(conf::WindowSetup::default().title("Adam's shitty chess engine!"))
	.window_mode(conf::WindowMode::default().dimensions(960.0, 960.0));

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new();
    event::run(ctx, event_loop, state)
}