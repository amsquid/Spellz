use ggez::ContextBuilder;
use ggez::event;

pub mod game;
pub mod events;

fn main() {
	let (mut ctx, event_loop) = ContextBuilder::new("spellz", "Joe")
		.build()
		.expect("Context failed to create");

	let game = game::Game::new(&mut ctx);

	event::run(ctx, event_loop, game);
}