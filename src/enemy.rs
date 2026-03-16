use macroquad::prelude::*;
use macroquad::rand::{gen_range};

use crate::{Bug, LEVEL_COUNT, GAME_W, GAME_H};
use crate::sprite::Sprite;
use crate::projectile::Projectile;

// enemy consts
const MAX_HEALTH: i32 = 3;

const RADIUS: f32 = 5.0;
pub const COLLISION_Y_OFFSET: f32 = -5.0;

const DASH_TIME: f64 = 2.0;
const DASH_STRENGTH: f32 = 60.0;
const DASH_FALLOFF: f32 = 60.0;

// manager consts
const INITIAL_SPAWN_TIME: f64 = 1.0;

static SPAWN_TIME_MIN: [f64; LEVEL_COUNT] = [
	5.0,
	3.5,
	3.5,
	3.0,
	2.0,
];

static SPAWN_TIME_MAX: [f64; LEVEL_COUNT] = [
	5.0,
	4.0,
	4.0,
	4.0,
	3.0,
];

static TRIPLE_SPAWN_CHANCE: [f32; LEVEL_COUNT] = [
	0.0,
	0.1,
	0.1,
	0.2,
	0.2,
];

static DOUBLE_SPAWN_CHANCE: [f32; LEVEL_COUNT] = [
	0.3,
	0.4,
	0.4,
	0.5,
	0.5,
];

// shouldve made separate files, oh well

pub struct Enemy {
	pub destroy: bool,
	health: i32,

	pub position: Vec2,
	velocity: Vec2,

	last_dash: f64,

	pub radius: f32,

	sprite: Sprite,
}

impl Enemy {
	pub fn new(position: Vec2, texture: Texture2D) -> Self {
		Self {
			destroy: false,
			health: MAX_HEALTH,

			position,
			velocity: vec2(0.0, 0.0),

			last_dash: 0.0,

			radius: RADIUS,

			sprite: Sprite::new(texture, vec2(10.0, 18.0), WHITE),
		}
	}

	pub fn process(&mut self, delta: f32, time: f64, player_position: Vec2, projectiles: &mut Vec<Projectile>, score: &mut u32) {
		// - damage and health -
		let collider_center: Vec2 = vec2(self.position.x, self.position.y + COLLISION_Y_OFFSET);

		for p in projectiles {
			if (p.position - collider_center).length() > self.radius {continue;}

			self.health -= 1;
			p.destroy = true;
		}

		if self.health <= 0 {
			self.destroy = true;
			*score += 10;
			return;
		}

		// - move -
		let direction: Vec2 = (player_position - self.position).normalize_or_zero();

		if time - self.last_dash >= DASH_TIME {
			self.velocity = direction * DASH_STRENGTH;
			self.last_dash = time;
		}

		self.position += self.velocity * delta;

		self.velocity = self.velocity.clamp_length_max(f32::max(self.velocity.length() - DASH_FALLOFF * delta, 0.0));

		// - sprite -
		self.sprite.process(time);
	}

	pub fn render(&self, current_bugs: &Vec<Bug>) {
		self.sprite.render(current_bugs, self.position.x, self.position.y);
	}
}

pub struct EnemyManager {
	pub enemies: Vec<Enemy>,

	last_spawn: f64,
	next_spawn_time: f64,

	enemy_ball_texture: Texture2D,
}

impl EnemyManager {
	pub fn new(enemy_ball_texture: Texture2D, time: f64) -> Self {
		Self {
			enemies: Vec::new(),

			last_spawn: time,
			next_spawn_time: INITIAL_SPAWN_TIME,

			enemy_ball_texture,
		}
	}

	fn spawn(&mut self) {
		let enter_vertically: bool = gen_range(0, 2) == 0;

		let position: Vec2 = if enter_vertically {
			let top: bool = gen_range(0, 2) == 0;
			vec2(
				gen_range(0.0, GAME_W),
				if top {0.0} else {GAME_H+18.0},
			)
		} else {
			let left: bool = gen_range(0, 2) == 0;
			vec2(
				if left {0.0} else {GAME_W},
				gen_range(0.0, GAME_H+18.0),
			)
		};

		self.enemies.push(Enemy::new(
			position,
			self.enemy_ball_texture.clone(),
		));
	}

	pub fn process(&mut self, level: usize, delta: f32, time: f64, player_position: Vec2, projectiles: &mut Vec<Projectile>, score: &mut u32) {
		// - spawning -
		if time - self.last_spawn >= self.next_spawn_time {
			let r: f32 = gen_range(0.0, 1.0);
			let count: u32 = if r < TRIPLE_SPAWN_CHANCE[level] {
				3
			} else if r < DOUBLE_SPAWN_CHANCE[level] {
				2
			} else {
				1
			};

			for _ in 0..count {
				self.spawn();
			}

			self.last_spawn = time;
			self.next_spawn_time = gen_range(SPAWN_TIME_MIN[level], SPAWN_TIME_MAX[level]);
		}

		// - processing enemies -
		for e in &mut self.enemies {
			e.process(delta, time, player_position, projectiles, score);
		}

		self.enemies.retain(|e| !e.destroy);
	}

	pub fn render(&self, current_bugs: &Vec<Bug>) {
		for e in &self.enemies {
			e.render(current_bugs);
		}
	}
}

