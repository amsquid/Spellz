use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;

use crate::game;

impl EventHandler for game::Game {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
	
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

		canvas.finish(ctx)
	}
}