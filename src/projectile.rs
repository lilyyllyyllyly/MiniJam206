use macroquad::prelude::*;

use crate::Bug;
use crate::sprite::Sprite;

const LIFETIME: f64 = 0.55;
const SPEED: f32 = 170.0;

pub struct Projectile {
	pub destroy: bool,
	create_time: f64,

	pub position: Vec2,
	direction: Vec2,

	sprite: Sprite,
}

impl Projectile {
	pub fn new(position: Vec2, direction: Vec2, create_time: f64, bullet_texture: Texture2D) -> Self {
		Self {
			destroy: false,
			create_time,

			position,
			direction,

			sprite: Sprite::new(bullet_texture, vec2(1.5, 1.5), WHITE),
		}
	}

	pub fn process(&mut self, delta: f32, time: f64) {
		if time - self.create_time >= LIFETIME {
			self.destroy = true;
			return;
		}

		self.position += self.direction * SPEED * delta;

		// - sprite -
		self.sprite.process(time);
	}

	pub fn render(&self, current_bug: &Bug) {
		self.sprite.render(current_bug, self.position.x, self.position.y);

		//match current_bug {
		//	Bug::Corrupted => draw_circle(self.position.x, self.position.y, 1.0, Color::new(0.1, 0.4, 1.0, 0.75)), // debug collision
		//	_ => {},
		//}
	}
}

