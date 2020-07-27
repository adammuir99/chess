use ggez;

use ggez::input;
use ggez::{Context, GameResult};
use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};

use chess::{Board, MoveGen};

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
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		Ok(())
	}
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		Ok(())
	}

	//Handle mouse inputs
	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32){
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
	let movegen = MoveGen::new_legal(&board);
	assert_eq!(movegen.len(), 20);

	let cb = ggez::ContextBuilder::new("input_test", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new();
    event::run(ctx, event_loop, state)
}