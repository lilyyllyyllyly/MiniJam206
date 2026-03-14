use macroquad::prelude::*;

use crate::{GAME_W, GAME_H};
use crate::sprite::Sprite;
use crate::projectile::Projectile;

const SPEED: f32 = 80.0;
const MAX_ACCEL: f32 = 400.0;

const BREAK_ALIGNMENT: f32 = -0.1;
const BREAK_STRENGTH: f32 = 2.0;

const PROJ_SPAWN_Y_OFFSET: f32 = -4.0;
const PROJ_SPAWN_DIST: f32 = 4.0;

pub struct Player {
	position: Vec2,
	velocity: Vec2,

	speed: f32,
	max_accel: f32,

	sprite: Sprite,
	gun_sprite: Sprite,
}

impl Player {
	pub fn new(texture: Texture2D, gun_texture: Texture2D) -> Self {
		Self {
			position: vec2(GAME_W/2.0, GAME_H/2.0),
			velocity: vec2(0.0, 0.0),

			speed: SPEED,
			max_accel: MAX_ACCEL,

			sprite: Sprite::new(texture, vec2(6.0, 11.0), WHITE),
			gun_sprite: Sprite::new(gun_texture, vec2(-1.0, 4.0), WHITE),
		}
	}

	pub fn process(&mut self, delta: f32, time: f64, game_mouse_position: Vec2, projectiles: &mut Vec<Projectile>) {
		// - move -
		let direction: Vec2 = vec2(
			(if is_key_down(KeyCode::D) {1.0} else {0.0}) - (if is_key_down(KeyCode::A) {1.0} else {0.0}),
			(if is_key_down(KeyCode::S) {1.0} else {0.0}) - (if is_key_down(KeyCode::W) {1.0} else {0.0}),
		).normalize_or_zero();

		let goal_vel: Vec2 = direction * self.speed;

		let max_accel = if self.velocity.normalize_or_zero().dot(direction) < BREAK_ALIGNMENT {
			self.max_accel * BREAK_STRENGTH
		} else {
			self.max_accel
		};

		self.velocity += (goal_vel - self.velocity).clamp_length_max(max_accel * delta);

		self.position = (self.position + self.velocity * delta).clamp(
			vec2(0.0, 4.0), // shaving 4 pixels off the top just so you dont disappear
			vec2(GAME_W, GAME_H),
		);

		// - gun -
		let aim_direction: Vec2 = (game_mouse_position - self.position).normalize_or_zero();

		// shooting
		if is_mouse_button_pressed(MouseButton::Left) {
			let proj_position: Vec2 = vec2(self.position.x, self.position.y + PROJ_SPAWN_Y_OFFSET) + aim_direction * PROJ_SPAWN_DIST;
			projectiles.push(Projectile::new(proj_position, aim_direction, time));
		}

		// update gun rotation
		self.gun_sprite.rotation = aim_direction.y.atan2(aim_direction.x);
		self.gun_sprite.flip_y = aim_direction.x < 0.0;

	}

	pub fn render(&self) {
		self.sprite.render(self.position.x, self.position.y);
		self.gun_sprite.render(self.position.x, self.position.y - 4.0);
	}
}

