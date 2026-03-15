use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

mod sprite;
mod player;
mod projectile;
mod enemy;

use player::Player;
use projectile::Projectile;
use enemy::EnemyManager;

const TITLE: &'static str = "wild_west_prototype_v0.1.0";

const GAME_W: f32 = 128.0;
const GAME_H: f32 = 128.0;

const INITIAL_BUG_CHANGE_TIME: f64 = 10.0;
const BUG_TEXT_TIME: f64 = 2.0;

const BUG_CHANGE_TIME_INTERVAL: f64 = 15.0;
const BUG_CHANGE_TIME_DECREMENT: f64 = 1.0;
const BUG_CHANGE_TIME_MIN: f64 = 2.0;

#[derive(Copy, Clone)]
enum Bug {
	BadAim,
	Inverted,
	Paint,
}

static BUGS: [Bug; 3] = [
	Bug::BadAim,
	Bug::Inverted,
	Bug::Paint,
];

impl Bug {
	fn random() -> Bug {
		BUGS.choose().cloned().expect("choose on a [T] should never fail (it always does get on an index from 0 to len open on len)")
	}

	fn name(&self) -> &'static str {
		match self {
			Bug::BadAim => "BAD AIM",
			Bug::Inverted => "INVERTED",
			Bug::Paint => "PAINT",
		}
	}
}

struct GameState {
	render_target: RenderTarget,
	camera: Camera2D,

	render_area_scale: f32,
	render_area_origin: Vec2,

	render_mouse_position: Vec2,
	game_mouse_position: Vec2,

	first_frame: bool,

	delta: f32,
	time: f64,

	player: Player,
	projectiles: Vec<Projectile>,

	enemies: EnemyManager,

	score: u32,

	current_bug: Bug,
	last_bug_change: f64,

	bug_change_time: f64,
	last_bug_time_decrease: f64,

	bg_texture: Texture2D,
}

impl GameState {
	async fn new() -> Self {
		let bg_texture: Texture2D = load_texture("assets/bg.png").await.expect("should be able to load bg texture");
		bg_texture.set_filter(FilterMode::Nearest);

		let player_texture: Texture2D = load_texture("assets/player.png").await.expect("should be able to load player texture");
		player_texture.set_filter(FilterMode::Nearest);

		let gun_texture: Texture2D = load_texture("assets/gun.png").await.expect("should be able to load gun texture");
		gun_texture.set_filter(FilterMode::Nearest);

		let enemy_ball_texture: Texture2D = load_texture("assets/enemy_ball.png").await.expect("should be able to load enemy_ball texture");
		enemy_ball_texture.set_filter(FilterMode::Nearest);

		let mut state: Self = Self {
			render_target: render_target(GAME_W as u32, GAME_H as u32),
			camera: Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_W, GAME_H)),

			render_area_scale: 0.0,
			render_area_origin: vec2(0.0, 0.0),

			render_mouse_position: vec2(0.0, 0.0),
			game_mouse_position: vec2(0.0, 0.0),

			first_frame: true,

			delta: 0.0,
			time: 0.0,

			player: Player::new(player_texture, gun_texture),
			projectiles: Vec::new(),
			enemies: EnemyManager::new(enemy_ball_texture),

			score: 0,

			current_bug: Bug::random(),
			last_bug_change: 0.0,

			bug_change_time: INITIAL_BUG_CHANGE_TIME,
			last_bug_time_decrease: 0.0,

			bg_texture,
		};

		state.render_target.texture.set_filter(FilterMode::Nearest);
		state.camera.render_target = Some(state.render_target.clone());

		state
	}

	fn process(&mut self) -> Result<(), ()> {
		if self.time - self.last_bug_change >= self.bug_change_time {
			self.current_bug = Bug::random();
			self.last_bug_change = self.time;
		}

		if self.time - self.last_bug_time_decrease >= BUG_CHANGE_TIME_INTERVAL {
			self.bug_change_time = f64::max(BUG_CHANGE_TIME_MIN, self.bug_change_time - BUG_CHANGE_TIME_DECREMENT);
			self.last_bug_time_decrease = self.time;
		}

		self.player.process(&self.current_bug, self.delta, self.time, self.game_mouse_position, &mut self.projectiles, &self.enemies.enemies /* ha enemies.enemies */);
		if self.player.dead {return Err(());}

		self.enemies.process(self.delta, self.time, self.player.position, &mut self.projectiles, &mut self.score);

		// - projectiles -
		// processing
		for p in &mut self.projectiles {
			p.process(self.delta, self.time);
		}

		// removing destroyed projectiles
		self.projectiles.retain(|p| !p.destroy);
		// --

		Ok(())
	}

	fn render(&self) {
		// -- RENDERING GAME TO TARGET --

		set_camera(&self.camera);

		// main stuff
		if !self.first_frame {
			match self.current_bug {
				Bug::Paint => {},
				_ => draw_texture(&self.bg_texture, 0.0, 0.0, WHITE),
			}
		} else {
			// gotta draw the background at least once in case Paint falls first
			draw_texture(&self.bg_texture, 0.0, 0.0, WHITE);
		}

		for p in &self.projectiles {
			p.render();
		}

		self.enemies.render();

		self.player.render();

		// ui
		draw_text(format!("HP: {}", self.player.health).as_str(), 0.0,  9.0, 16.0, BLACK);
		draw_text(format!("AMMO: {}", self.player.ammo).as_str(), 0.0, 18.0, 16.0, BLACK);

		draw_text(format!("SCORE: {}", self.score*10).as_str(), 0.0, GAME_H-1.0, 16.0, BLACK);

		let bug_text: String = format!("BUG: {}", self.current_bug.name());
		let bug_text_width: f32 = measure_text(bug_text.as_str(), None, 16, 1.0).width;
		draw_text(bug_text.as_str(), (GAME_W - bug_text_width)/2.0, 27.0, 16.0, Color::new(0.0, 0.0, 0.0, 1.0 - ((self.time - self.last_bug_change)/BUG_TEXT_TIME) as f32));


		// -- RENDERING TARGET TO SCREEN --

		set_default_camera();
		clear_background(BLACK);

		draw_texture_ex(
			&self.render_target.texture,
			self.render_area_origin.x, self.render_area_origin.y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(vec2(GAME_W, GAME_H) * self.render_area_scale),
				flip_y: true,
				..Default::default()
			},
		);
	}
}

fn window_conf() -> Conf {
	Conf {
		window_title: String::from(TITLE),

		platform: miniquad::conf::Platform {
			webgl_version: miniquad::conf::WebGLVersion::WebGL2, // for render target related functions
			..Default::default()
		},

		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	macroquad::rand::srand(macroquad::miniquad::date::now() as _); // random seed

	let mut state: GameState = GameState::new().await;

	loop {
		state.delta = get_frame_time();
		state.time = get_time();

		state.render_area_scale = f32::min(
			screen_width()  / GAME_W,
			screen_height() / GAME_H,
		);

		state.render_area_origin = vec2(
			(screen_width()  - (GAME_W * state.render_area_scale)) / 2.0,
			(screen_height() - (GAME_H * state.render_area_scale)) / 2.0,
		);

		state.render_mouse_position = vec2(mouse_position().0, mouse_position().1);
		state.game_mouse_position = (state.render_mouse_position - state.render_area_origin) / state.render_area_scale;

		if state.process().is_err() {
			state = GameState::new().await;
			continue;
		}

		state.render();

		state.first_frame = false;
		next_frame().await;
	}
}

