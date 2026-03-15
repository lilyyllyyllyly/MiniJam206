use macroquad::prelude::*;
use macroquad::rand::{gen_range};

use crate::{GAME_W, GAME_H};
use crate::sprite::Sprite;

// enemy consts
const RADIUS: f32 = 4.0;
const COLLISION_Y_OFFSET: f32 = -5.0;

const DASH_TIME: f64 = 2.0;
const DASH_STRENGTH: f32 = 60.0;
const DASH_FALLOFF: f32 = 60.0;

// manager consts
const SPAWN_TIME_MIN: f64 = 3.0;
const SPAWN_TIME_MAX: f64 = 4.0;
const INITIAL_SPAWN_TIME: f64 = 1.0;

const DOUBLE_SPAWN_CHANCE: f32 = 0.3;

// shouldve made separate files, oh well

pub struct Enemy {
	position: Vec2,
	velocity: Vec2,

	last_dash: f64,

	radius: f32,

	sprite: Sprite,
}

impl Enemy {
	pub fn new(position: Vec2, texture: Texture2D) -> Self {
		Self {
			position,
			velocity: vec2(0.0, 0.0),

			last_dash: 0.0,

			radius: RADIUS,

			sprite: Sprite::new(texture, vec2(10.0, 18.0), WHITE),
		}
	}

	pub fn process(&mut self, delta: f32, time: f64, player_position: Vec2) {
		let direction: Vec2 = (player_position - self.position).normalize_or_zero();

		if time - self.last_dash >= DASH_TIME {
			self.velocity = direction * DASH_STRENGTH;
			self.last_dash = time;
		}

		self.position += self.velocity * delta;

		self.velocity = self.velocity.clamp_length_max(f32::max(self.velocity.length() - DASH_FALLOFF * delta, 0.0));
	}

	pub fn render(&self) {
		self.sprite.render(self.position.x, self.position.y);
		draw_circle_lines(self.position.x, self.position.y + COLLISION_Y_OFFSET, self.radius, 1.0, Color::new(0.1, 0.4, 1.0, 0.75)); // debug collision
	}
}

pub struct EnemyManager {
	pub enemies: Vec<Enemy>,

	last_spawn: f64,
	next_spawn_time: f64,

	enemy_ball_texture: Texture2D,
}

impl EnemyManager {
	pub fn new(enemy_ball_texture: Texture2D) -> Self {
		Self {
			enemies: Vec::new(),

			last_spawn: 0.0,
			next_spawn_time: INITIAL_SPAWN_TIME,

			enemy_ball_texture,
		}
	}

	fn spawn(&mut self) {
		let enter_vertically: bool = gen_range(0, 1) == 0;
		let position: Vec2 = if enter_vertically {
			let top: bool = gen_range(0, 1) == 0;
			vec2(
				gen_range(0.0, GAME_W),
				if top {0.0} else {GAME_H},
			)
		} else {
			let left: bool = gen_range(0, 1) == 0;
			vec2(
				if left {0.0} else {GAME_W},
				gen_range(0.0, GAME_H),
			)
		};

		self.enemies.push(Enemy::new(
			position,
			self.enemy_ball_texture.clone(),
		));
	}

	pub fn process(&mut self, delta: f32, time: f64, player_position: Vec2) {
		// spawning
		if time - self.last_spawn >= self.next_spawn_time {
			if gen_range(0.0, 1.0) < DOUBLE_SPAWN_CHANCE {
				for _ in 0..2 {
					self.spawn();
				}
			} else {
				self.spawn();
			}

			self.last_spawn = time;
			self.next_spawn_time = gen_range(SPAWN_TIME_MIN, SPAWN_TIME_MAX);
		}

		// processing enemies
		for e in &mut self.enemies {
			e.process(delta, time, player_position);
		}
	}

	pub fn render(&self) {
		for e in &self.enemies {
			e.render();
		}
	}
}

