use ggez::Context;
use ggez::graphics::{DrawMode, Rect, rectangle, set_color};
use ggez::graphics::Color;
use nalgebra::{Isometry2, Vector2};
use ncollide::world::{CollisionObjectHandle, CollisionWorld2};

pub struct Player {
	position: Vector2<f32>,
	pub velocity: Vector2<f32>,
	size: Vector2<f32>,
	color: Color,
	col_handle: Option<CollisionObjectHandle>,
}

impl Player {
	pub fn new() -> Self {
		Player {
			position: Vector2::new(0., 0.),
			velocity: Vector2::new(0., 0.),
			size: Vector2::new(32., 32.),
			color: Color::new(1., 1., 1., 1.),
			col_handle: None,
		}
	}

	pub fn set_col_handle(&mut self, col_handle: CollisionObjectHandle) {
		self.col_handle = Some(col_handle);
	}

	pub fn update(&mut self, world: &mut CollisionWorld2<f32, ()>) {
		self.position.x += self.velocity.x;
		self.position.y += self.velocity.y;

		world.set_position(self.col_handle.unwrap(), Isometry2::new(self.position.clone(), 0.));
	}

	pub fn set_color(&mut self, new_color: Color) {
		self.color = new_color;
	}

	pub fn draw(&self, ctx: &mut Context) {
		set_color(ctx, self.color);
		rectangle(ctx, DrawMode::Fill, Rect::new(self.position.x, self.position.y, self.size.x, self.size.y));
	}
}
