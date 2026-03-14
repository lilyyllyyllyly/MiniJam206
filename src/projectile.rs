use macroquad::prelude::*;

const LIFETIME: f64 = 0.55;
const SPEED: f32 = 170.0;

pub struct Projectile {
	pub destroy: bool,
	create_time: f64,

	position: Vec2,
	direction: Vec2,
}

impl Projectile {
	pub fn new(position: Vec2, direction: Vec2, create_time: f64) -> Self {
		Self {
			destroy: false,
			create_time,

			position,
			direction,
		}
	}

	pub fn process(&mut self, delta: f32, time: f64) {
		if time - self.create_time >= LIFETIME {
			self.destroy = true;
			return;
		}

		self.position += self.direction * SPEED * delta;
	}

	pub fn render(&self) {
		draw_circle(self.position.x, self.position.y, 1.5, BLACK);
	}
}

