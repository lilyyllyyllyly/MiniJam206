use macroquad::prelude::*;

mod sprite;
mod player;
mod projectile;

use player::Player;
use projectile::Projectile;

const TITLE: &'static str = "Mini Jam 206";

const GAME_W: f32 = 128.0;
const GAME_H: f32 = 128.0;

struct GameState {
	render_target: RenderTarget,
	camera: Camera2D,

	render_area_scale: f32,
	render_area_origin: Vec2,

	render_mouse_position: Vec2,
	game_mouse_position: Vec2,

	delta: f32,
	time: f64,

	player: Player,
	projectiles: Vec<Projectile>,

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

		let mut state: Self = Self {
			render_target: render_target(GAME_W as u32, GAME_H as u32),
			camera: Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_W, GAME_H)),

			render_area_scale: 0.0,
			render_area_origin: vec2(0.0, 0.0),

			render_mouse_position: vec2(0.0, 0.0),
			game_mouse_position: vec2(0.0, 0.0),

			delta: 0.0,
			time: 0.0,

			bg_texture,

			player: Player::new(player_texture, gun_texture),

			projectiles: Vec::new(),
		};

		state.render_target.texture.set_filter(FilterMode::Nearest);
		state.camera.render_target = Some(state.render_target.clone());

		state
	}

	fn process(&mut self) {
		self.player.process(self.delta, self.time, self.game_mouse_position, &mut self.projectiles);

		for p in &mut self.projectiles {
			p.process(self.delta, self.time);
		}

		self.projectiles.retain(|p| !p.destroy);
	}

	fn render(&self) {
		// -- RENDERING GAME TO TARGET --

		set_camera(&self.camera);

		draw_texture(&self.bg_texture, 0.0, 0.0, WHITE);

		self.player.render();

		for p in &self.projectiles {
			p.render();
		}


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

		state.process();
		state.render();

		next_frame().await;
	}
}

