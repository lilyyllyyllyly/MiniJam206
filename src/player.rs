use macroquad::prelude::*;

use crate::{Bug, GAME_W, GAME_H};
use crate::sprite::Sprite;
use crate::projectile::Projectile;

use crate::enemy::Enemy;
use crate::enemy::COLLISION_Y_OFFSET as ENEMY_COLLISION_OFFSET;

const MAX_HEALTH: i32 = 5;
const IFRAMES: f64 = 0.8; // measured in seconds, despite literally being called i*frames*
const INVULNERABLE_ALPHA: f32 = 0.5;
const COLLISION_Y_OFFSET: f32 = -4.0;

const SPEED: f32 = 80.0;
const MAX_ACCEL: f32 = 600.0;

const BREAK_ALIGNMENT: f32 = -0.1;
const BREAK_STRENGTH: f32 = 2.5;

const GUN_Y_OFFSET: f32 = -4.0;
const PROJ_SPAWN_DIST: f32 = 4.0;

const MAX_AMMO: u32 = 6;
const RELOAD_TIME: f64 = 0.6;
const SHOOT_DELAY: f64 = 0.15;

pub struct Player {
	pub dead: bool,
	pub health: i32,
	last_hit: f64,

	pub position: Vec2,
	velocity: Vec2,

	speed: f32,
	max_accel: f32,

	pub ammo: u32,
	last_shot: f64,

	sprite: Sprite,
	gun_sprite: Sprite,

	bullet_texture: Texture2D,
}

impl Player {
	pub fn new(texture: Texture2D, gun_texture: Texture2D, bullet_texture: Texture2D) -> Self {
		Self {
			dead: false,
			health: MAX_HEALTH,
			last_hit: 0.0,

			position: vec2(GAME_W/2.0, GAME_H/2.0),
			velocity: vec2(0.0, 0.0),

			speed: SPEED,
			max_accel: MAX_ACCEL,

			ammo: MAX_AMMO,
			last_shot: 0.0,

			sprite: Sprite::new(texture, vec2(6.0, 11.0), WHITE),
			gun_sprite: Sprite::new(gun_texture, vec2(-1.0, -GUN_Y_OFFSET), WHITE),

			bullet_texture,
		}
	}

	pub fn process(&mut self, current_bug: &Bug, delta: f32, time: f64, game_mouse_position: Vec2, projectiles: &mut Vec<Projectile>, enemies: &Vec<Enemy>) {
		// - death -
		if time - self.last_hit <= IFRAMES {
			// invulnerable

			// setting sprite alpha
			self.sprite.tint = Color::new(1.0, 1.0, 1.0, INVULNERABLE_ALPHA);
		} else {
			// vulnerable

			// setting sprite alpha
			self.sprite.tint = WHITE;

			// checking hit
			for e in enemies {
				let player_center: Vec2 = vec2(self.position.x, self.position.y + COLLISION_Y_OFFSET);
				let enemy_center: Vec2 = vec2(e.position.x, e.position.y + ENEMY_COLLISION_OFFSET);

				if (enemy_center - player_center).length() > e.radius {continue;}

				self.health -= 1;
				self.last_hit = time;

				if self.health <= 0 {
					self.dead = true;
					return;
				}

				break;
			}
		}

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
		let mut aim_direction: Vec2 = (game_mouse_position - vec2(self.position.x, self.position.y + GUN_Y_OFFSET)).normalize_or_zero();
		match current_bug {
			Bug::BadAim => aim_direction = vec2(-aim_direction.y, aim_direction.x),
			_ => {},
		}

		// shooting
		if self.ammo == 0 && time - self.last_shot >= RELOAD_TIME {
			self.ammo = 6;
		}

		if is_mouse_button_down(MouseButton::Left) && time - self.last_shot >= SHOOT_DELAY && self.ammo > 0 {
			let proj_position: Vec2 = vec2(self.position.x, self.position.y + GUN_Y_OFFSET) + aim_direction * PROJ_SPAWN_DIST;
			projectiles.push(Projectile::new(proj_position, aim_direction, time, self.bullet_texture.clone()));

			self.ammo -= 1;
			self.last_shot = time;
		}

		// update gun rotation
		self.gun_sprite.rotation = aim_direction.y.atan2(aim_direction.x);
		self.gun_sprite.flip_y = aim_direction.x < 0.0;

		// - sprite -
		self.sprite.process(time);
		self.gun_sprite.process(time);

	}

	pub fn render(&self, current_bug: &Bug) {
		self.sprite.render(current_bug, self.position.x, self.position.y);
		self.gun_sprite.render(current_bug, self.position.x, self.position.y - 4.0);

		//match current_bug {
		//	Bug::Corrupted => draw_circle(self.position.x, self.position.y + COLLISION_Y_OFFSET, 1.0, Color::new(0.1, 0.4, 1.0, 0.75)), // debug collision
		//	_ => {},
		//}
	}
}

