use std::cmp::Eq;

use macroquad::prelude::*;
use macroquad::rand::{ChooseRandom, gen_range};

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

const LEVEL_COUNT: usize = 5;
const LEVEL_TIME: f64 = 34.0;

const DOUBLE_BUG_LEVEL: usize = 2;

const BUG_TEXT_TIME: f64 = 2.0;
static BUG_CHANGE_TIMES: [f64; LEVEL_COUNT] = [
	10.0,
	7.0,
	7.0,
	6.0,
	5.0,
];

const BG_CORRUPTED_TINT_MIN: f32 = 0.7;
const BG_CORRUPTED_TINT_MAX: f32 = 1.0;
const BG_CORRUPTED_TINT_TIME: f64 = 2.5;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Bug {
	None,
	BadAim,
	Inverted,
	Paint,
	Corrupted,
}

static BUGS: [Bug; 4] = [
	Bug::BadAim,
	Bug::Inverted,
	Bug::Paint,
	Bug::Corrupted,
];

impl Bug {
	fn random() -> Bug {
		BUGS.choose().cloned().expect("choose on a [T] should never fail (it always does get on an index from 0 to len open on len)")
	}

	fn name(&self) -> &'static str {
		match self {
			Bug::None => "NONE (um actual bug what)",
			Bug::BadAim => "BAD AIM",
			Bug::Inverted => "INVERTED",
			Bug::Paint => "PAINT",
			Bug::Corrupted => "CORRUPTED",
		}
	}
}

struct GameState {
	font: Font,

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

	level: usize,
	last_level_change: f64,

	current_bugs: Vec<Bug>,
	last_bug_change: f64,

	last_bg_corrupt: f64,
	bg_corrupt_tint: Color,

	bg_texture: Texture2D,
}

impl GameState {
	async fn new(start_time: f64) -> Self {
		let bg_texture: Texture2D = load_texture("assets/bg.png").await.expect("should be able to load bg texture");
		bg_texture.set_filter(FilterMode::Nearest);

		let player_texture: Texture2D = load_texture("assets/player.png").await.expect("should be able to load player texture");
		player_texture.set_filter(FilterMode::Nearest);

		let gun_texture: Texture2D = load_texture("assets/gun.png").await.expect("should be able to load gun texture");
		gun_texture.set_filter(FilterMode::Nearest);

		let bullet_texture: Texture2D = load_texture("assets/bullet.png").await.expect("should be able to load bullet texture");
		bullet_texture.set_filter(FilterMode::Nearest);

		let enemy_ball_texture: Texture2D = load_texture("assets/enemy_ball.png").await.expect("should be able to load enemy_ball texture");
		enemy_ball_texture.set_filter(FilterMode::Nearest);

		let mut state: Self = Self {
			font: load_ttf_font("mousetrap2.ttf").await.expect("should be able to load mousetrap2 font"),

			render_target: render_target(GAME_W as u32, GAME_H as u32),
			camera: Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_W, GAME_H)),

			render_area_scale: 0.0,
			render_area_origin: vec2(0.0, 0.0),

			render_mouse_position: vec2(0.0, 0.0),
			game_mouse_position: vec2(0.0, 0.0),

			first_frame: true,

			delta: 0.0,
			time: start_time,

			player: Player::new(player_texture, gun_texture, bullet_texture),
			projectiles: Vec::new(),
			enemies: EnemyManager::new(enemy_ball_texture, start_time),

			score: 0,

			level: 0,
			last_level_change: start_time,

			current_bugs: vec!(Bug::None),
			last_bug_change: start_time,

			last_bg_corrupt: 0.0,
			bg_corrupt_tint: WHITE,

			bg_texture,
		};

		state.render_target.texture.set_filter(FilterMode::Nearest);
		state.camera.render_target = Some(state.render_target.clone());

		state
	}

	fn process(&mut self) -> Result<(), ()> {
		// - updating level -
		if self.level < LEVEL_COUNT-1 && self.time - self.last_level_change >= LEVEL_TIME {
			self.level += 1;
			if self.level == DOUBLE_BUG_LEVEL {self.current_bugs.push(Bug::None)}
			self.last_level_change = self.time;
		}

		// - updating bug -
		if self.time - self.last_bug_change >= BUG_CHANGE_TIMES[self.level] {
			for i in 0..self.current_bugs.len() {
				self.current_bugs[i] = Bug::random();
			}

			self.last_bug_change = self.time;
		}

		// - bg corrupt
		if self.time - self.last_bg_corrupt >= BG_CORRUPTED_TINT_TIME {
			self.bg_corrupt_tint = Color::new(
				gen_range(BG_CORRUPTED_TINT_MIN, BG_CORRUPTED_TINT_MAX),
				gen_range(BG_CORRUPTED_TINT_MIN, BG_CORRUPTED_TINT_MAX),
				gen_range(BG_CORRUPTED_TINT_MIN, BG_CORRUPTED_TINT_MAX),
				1.0,
			);
			self.last_bg_corrupt = self.time;
		}

		// - main stuff -
		self.player.process(&self.current_bugs, self.delta, self.time, self.game_mouse_position, &mut self.projectiles, &self.enemies.enemies /* ha enemies.enemies */);
		if self.player.dead {return Err(());}

		self.enemies.process(self.level, self.delta, self.time, self.player.position, &mut self.projectiles, &mut self.score);

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

		// - main stuff -
		if self.first_frame || !self.current_bugs.contains(&Bug::Paint) {
			// gotta draw the background at least once in case Paint falls first
			let tint: Color = if self.current_bugs.contains(&Bug::Corrupted) {self.bg_corrupt_tint} else {WHITE};
			draw_texture(&self.bg_texture, 0.0, 0.0, tint);
		}

		for p in &self.projectiles {
			p.render(&self.current_bugs);
		}

		self.enemies.render(&self.current_bugs);

		self.player.render(&self.current_bugs);

		// - ui -
		draw_text_ex(
			format!("HP: {}", self.player.health).as_str(),
			1.0, 6.0,
			TextParams {
				font: Some(&self.font),
				font_size: 10,
				color: BLACK,
				..Default::default()
			}
		);

		draw_text_ex(
			format!("AMMO: {}", self.player.ammo).as_str(),
			1.0, 12.0,
			TextParams {
				font: Some(&self.font),
				font_size: 10,
				color: BLACK,
				..Default::default()
			}
		);

		draw_text_ex(
			format!("SCORE: {}", self.score).as_str(),
			1.0, GAME_H - 1.0,
			TextParams {
				font: Some(&self.font),
				font_size: 10,
				color: BLACK,
				..Default::default()
			}
		);

		let level_text: String = format!("LEVEL: {}", self.level+1);
		let level_text_width: f32 = measure_text(level_text.as_str(), Some(&self.font), 10, 1.0).width;
		draw_text_ex(
			level_text.as_str(),
			((GAME_W-level_text_width)/2.0).floor(), 6.0,
			TextParams {
				font: Some(&self.font),
				font_size: 10,
				color: BLACK,
				..Default::default()
			}
		);

		for (i, &b) in self.current_bugs.iter().enumerate() {
			if b == Bug::None {continue;}

			let bug_text: String = format!("BUG: {}", b.name());
			let bug_text_width: f32 = measure_text(bug_text.as_str(), Some(&self.font), 10, 1.0).width;
			draw_text_ex(
				bug_text.as_str(),
				((GAME_W - bug_text_width)/2.0).floor(), 18.0 + 6.0 * i as f32,
				TextParams {
					font: Some(&self.font),
					font_size: 10,
					color: Color::new(0.0, 0.0, 0.0, 1.0 - ((self.time - self.last_bug_change)/BUG_TEXT_TIME) as f32),
					..Default::default()
				}
			);
		}


		// -- RENDERING TARGET TO SCREEN --

		set_default_camera();
		clear_background(BLACK);

		// inverted screen bug
		let (flip_x, flip_y): (bool, bool) = if self.current_bugs.contains(&Bug::Inverted) {(true, false)} else {(false, true)};

		draw_texture_ex(
			&self.render_target.texture,
			self.render_area_origin.x, self.render_area_origin.y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(vec2(GAME_W, GAME_H) * self.render_area_scale),
				flip_x, flip_y,
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

	let mut menu: bool = true;
	let mut last_menu: bool = true;

	let mut state: GameState = GameState::new(0.0).await;

	let mut highscore: u32 = 0;
	let mut lastscore: u32 = 0;
	let mut highlevel: usize = 0;
	let mut lastlevel: usize = 0;

	loop {
		if last_menu && !menu {
			state = GameState::new(state.time).await;
		}
		last_menu = menu;

		// setting state variables
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


		if menu {
			if is_key_pressed(KeyCode::Space) {
				menu = false;
				continue;
			}

			set_camera(&state.camera);
			clear_background(BLACK);

			let title_text_width: f32 = measure_text(TITLE, Some(&state.font), 10, 1.0).width;
			draw_text_ex(
				TITLE,
				((GAME_W-title_text_width)/2.0).floor(), 18.0,
				TextParams {
					font: Some(&state.font),
					font_size: 10,
					..Default::default()
				},
			);

			let play_text_width: f32 = measure_text("PLAY", Some(&state.font), 10, 1.0).width;
			draw_text_ex(
				"PLAY",
				((GAME_W-play_text_width)/2.0).floor(), GAME_H/2.0,
				TextParams {
					font: Some(&state.font),
					font_size: 10,
					..Default::default()
				},
			);

			draw_text_ex(
				format!("LAST SCORE: {} (LEVEL {})", lastscore, lastlevel + 1).as_str(),
				1.0, GAME_H-7.0,
				TextParams {
					font: Some(&state.font),
					font_size: 10,
					..Default::default()
				},
			);

			draw_text_ex(
				format!("HIGHSCORE: {} (LEVEL {})", highscore, highlevel + 1).as_str(),
				1.0, GAME_H-1.0,
				TextParams {
					font: Some(&state.font),
					font_size: 10,
					..Default::default()
				},
			);

			// -- RENDERING TARGET TO SCREEN --

			set_default_camera();
			clear_background(BLACK);

			draw_texture_ex(
				&state.render_target.texture,
				state.render_area_origin.x, state.render_area_origin.y,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(GAME_W, GAME_H) * state.render_area_scale),
					flip_y: true,
					..Default::default()
				},
			);

		} else {
			// fix mouse position on inverted bug (should only affect movement)
			if state.current_bugs.contains(&Bug::Inverted) {
				state.game_mouse_position = vec2(GAME_W, GAME_H) - state.game_mouse_position;
			}

			// process and render
			if state.process().is_err() {
				lastscore = state.score;
				lastlevel = state.level;
				if lastscore > highscore {
					highscore = lastscore;
					highlevel = lastlevel;
				}

				menu = true;
				continue;
			}

			state.render();
			// -

			state.first_frame = false;
		}

		next_frame().await;
	}
}

