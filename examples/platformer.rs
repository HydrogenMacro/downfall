use macroquad::prelude::*;
use downfall::world::World;

#[macroquad::main("Downfall Platformer Example")]
async fn main() {
	let world = World::new();
	loop {
		clear_background(SKYBLUE);
		draw_rectangle(0., 0., 10., 10., RED);
		next_frame().await;
	}
}