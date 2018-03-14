extern crate ggez;
extern crate nalgebra;
extern crate ncollide;

use ggez::*;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{Color, DrawMode, Rect};
use nalgebra::{Isometry2, Vector2};
use ncollide::shape::{Cuboid2, ShapeHandle2};
use ncollide::world::{CollisionGroups, CollisionObjectHandle, CollisionWorld2, GeometricQueryType};
use player::Player;

pub mod player;

pub struct CollisionData {}

struct MainState {
	player: Player,
	world: CollisionWorld2<f32, ()>,
}

impl MainState {
	fn new(_ctx: &mut Context) -> Self {
		MainState {
			player: Player::new(),
			world: CollisionWorld2::new(0.02),
		}
	}

	pub fn add_collision_entity(&mut self, isometry: Isometry2<f32>, shape_handle:
	ShapeHandle2<f32>,
		groups: CollisionGroups, query: GeometricQueryType<f32>) -> CollisionObjectHandle {
		self.world.add(isometry, shape_handle, groups, query, ())
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		self.player.update(&mut self.world);
		self.world.update();

		if self.world.contacts().count() > 0 {
			self.player.set_color(Color::new(1., 0., 0., 1.));
		} else {
			self.player.set_color(Color::new(1., 1., 1., 1.));
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);
		graphics::set_color(ctx, Color::new(0., 1., 0., 1.));
		graphics::rectangle(ctx, DrawMode::Fill, Rect::new(50., 50., 32., 32.));
		self.player.draw(ctx);
		graphics::present(ctx);
		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, key: Keycode, _keymod: Mod, _repeat: bool) {
		if _repeat { return; }
		match key {
			Keycode::Z => self.player.velocity.y -= 1.,
			Keycode::S => self.player.velocity.y += 1.,
			Keycode::Q => self.player.velocity.x -= 1.,
			Keycode::D => self.player.velocity.x += 1.,
			_ => (),
		}
	}

	fn key_up_event(&mut self, ctx: &mut Context, key: Keycode, _keymod: Mod, _repeat: bool) {
		if _repeat { return; }
		match key {
			Keycode::Z => self.player.velocity.y += 1.,
			Keycode::S => self.player.velocity.y -= 1.,
			Keycode::Q => self.player.velocity.x += 1.,
			Keycode::D => self.player.velocity.x -= 1.,
			_ => (),
		}
	}
}

pub fn main() {
	let c = conf::Conf::new();
	let ctx = &mut Context::load_from_conf("ggez_ncollide_example", "opinon", c).unwrap();

	let mut state = MainState::new(ctx);
	let shape = ShapeHandle2::new(Cuboid2::new(Vector2::new(16., 16.)));
	let groups = CollisionGroups::new();
	let query = GeometricQueryType::Contacts(0., 0.);
	state.add_collision_entity(Isometry2::new(Vector2::new(50., 50.), 0.), shape.clone(), groups,
							   query);
	let player_collision_handle = state.add_collision_entity(Isometry2::new(Vector2::new(0., 0.), 0.), shape.clone(), groups, query);
	state.player.set_col_handle(player_collision_handle);

	event::run(ctx, &mut state).unwrap();
}