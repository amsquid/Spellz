use ggez::Context;

pub struct Game {
	
}

impl Game {
	pub fn new(_ctx: &mut Context) -> Game {
		// Setup Window
		_ctx.gfx.set_window_title("Spellz");
		let _ = _ctx.gfx.set_drawable_size(1600.0, 900.0);

		// Loading Resources
		Game {
			// ...
		}
	}
}