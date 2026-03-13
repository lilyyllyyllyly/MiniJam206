use macroquad::prelude::*;

const TITLE: &'static str = "Mini Jam 206";

const GAME_W: f32 = 128.0;
const GAME_H: f32 = 128.0;

struct GameState {
	render_target: RenderTarget,
	camera: Camera2D,

	delta: f32,

	text_pos: Vec2,
}

impl GameState {
	fn new() -> Self {
		let mut state: Self = Self {
			render_target: render_target(GAME_W as u32, GAME_H as u32),
			camera: Camera2D::from_display_rect(Rect::new(0.0, 0.0, GAME_W, GAME_H)),

			delta: 0.0,

			text_pos: vec2(0.0, 0.0),
		};

		state.render_target.texture.set_filter(FilterMode::Nearest);
		state.camera.render_target = Some(state.render_target.clone());

		state
	}

	fn process(&mut self) {
		self.text_pos += vec2(10.0, 10.0) * self.delta;
	}

	fn render(&self) {
		// -- RENDERING GAME TO TARGET --

		set_camera(&self.camera);
		clear_background(WHITE);

		draw_text(":3", self.text_pos.x, self.text_pos.y, 16.0, BLACK);


		// -- RENDERING TARGET TO SCREEN --

		set_default_camera();
		clear_background(BLACK);

		let screen_area_scale: f32 = f32::min(
			screen_width()  / GAME_W,
			screen_height() / GAME_H,
		);

		draw_texture_ex(
			&self.render_target.texture,
			(screen_width()  - (GAME_W * screen_area_scale)) / 2.0,
			(screen_height() - (GAME_H * screen_area_scale)) / 2.0,
			WHITE,
			DrawTextureParams {
				dest_size: Some(vec2(GAME_W * screen_area_scale, GAME_H * screen_area_scale)),
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
	let mut state: GameState = GameState::new();

	loop {
		state.delta = get_frame_time();

		state.process();
		state.render();

		next_frame().await;
	}
}

