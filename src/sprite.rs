use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::Bug;

const CORRUPT_TIME: f64 = 1.0;

const PIVOT_CORRUPT_MIN: f32 = -3.0;
const PIVOT_CORRUPT_MAX: f32 =  3.0;

const TINT_CORRUPT_MIN: f32 = 0.0;
const TINT_CORRUPT_MAX: f32 = 1.0;

const SCALE_CORRUPT_MIN: f32 = 0.75;
const SCALE_CORRUPT_MAX: f32 = 1.5;

const SOURCE_CORRUPT_MIN: f32 = 0.75;
const SOURCE_CORRUPT_MAX: f32 = 1.0;

pub struct Sprite {
	// can you tell im incredibly inconsistent with pub
	texture: Texture2D,
	pub tint: Color,

	pivot: Vec2,

	pub rotation: f32,
	pub rotation_offset: Vec2,

	pub flip_x: bool,
	pub flip_y: bool,

	// very bad way of doing the corrupted bug
	last_corrupt: f64,
	corrupt_time_offset: f64,
	pivot_corrupt: Vec2,
	flip_x_corrupt: bool,
	flip_y_corrupt: bool,
	tint_corrupt: Color,
	scale_corrupt: Vec2,
	source_corrupt: Vec2,
}

impl Sprite {
	// since last game i added tint as a parameter of new but then i ended up not adding the other fields as parameters because
	// it makes much more sense to set them later if necessary but then i literally never use anything other than WHITE for tint
	// when calling new so i should have just kept it as it was without tint here but now its too late to change it :sob:
	pub fn new(texture: Texture2D, pivot: Vec2, tint: Color) -> Self {
		Self {
			texture,
			tint,

			pivot,

			rotation: 0.0,
			rotation_offset: vec2(0.0, 0.0),

			flip_x: false,
			flip_y: false,

			// a
			last_corrupt: 0.0,
			corrupt_time_offset: gen_range(0.0, CORRUPT_TIME),
			pivot_corrupt: vec2(0.0, 0.0),
			flip_x_corrupt: false,
			flip_y_corrupt: false,
			tint_corrupt: WHITE,
			scale_corrupt: vec2(1.0, 1.0),
			source_corrupt: vec2(1.0, 1.0),
		}
	}

	pub fn process(&mut self, time: f64) {
		if time + self.corrupt_time_offset - self.last_corrupt < CORRUPT_TIME {return;}

		self.pivot_corrupt = vec2(gen_range(PIVOT_CORRUPT_MIN, PIVOT_CORRUPT_MAX), gen_range(PIVOT_CORRUPT_MIN, PIVOT_CORRUPT_MAX));

		self.flip_x_corrupt = gen_range(0, 2) == 0;
		self.flip_y_corrupt = gen_range(0, 2) == 0;

		self.tint_corrupt = Color::new(
			gen_range(TINT_CORRUPT_MIN, TINT_CORRUPT_MAX),
			gen_range(TINT_CORRUPT_MIN, TINT_CORRUPT_MAX),
			gen_range(TINT_CORRUPT_MIN, TINT_CORRUPT_MAX),
			1.0,
		);

		self.scale_corrupt = vec2(gen_range(SCALE_CORRUPT_MIN, SCALE_CORRUPT_MAX), gen_range(SCALE_CORRUPT_MIN, SCALE_CORRUPT_MAX));
		self.source_corrupt = vec2(gen_range(SOURCE_CORRUPT_MIN, SOURCE_CORRUPT_MAX), gen_range(SOURCE_CORRUPT_MIN, SOURCE_CORRUPT_MAX));

		self.last_corrupt = time + self.corrupt_time_offset;
	}

	pub fn render(&self, current_bugs: &Vec<Bug>, x: f32, y: f32) {
		if current_bugs.contains(&Bug::Corrupted) {
			let source_size = self.texture.size() * self.source_corrupt;
			draw_texture_ex(
				&self.texture,
				x - self.pivot.x + self.pivot_corrupt.x, y - self.pivot.y + self.pivot_corrupt.y,
				self.tint_corrupt,
				DrawTextureParams {
					dest_size: Some(self.texture.size() * self.scale_corrupt),
					source: Some(Rect::new(0.0, 0.0, source_size.x, source_size.y)),
					rotation: self.rotation,
					pivot: Some(vec2(x, y) + self.rotation_offset),
					flip_x: self.flip_x_corrupt,
					flip_y: self.flip_y_corrupt,
					..Default::default()
				}
			);
		} else {
			draw_texture_ex(
				&self.texture,
				x - self.pivot.x, y - self.pivot.y,
				self.tint,
				DrawTextureParams {
					rotation: self.rotation,
					pivot: Some(vec2(x, y) + self.rotation_offset),
					flip_x: self.flip_x,
					flip_y: self.flip_y,
					..Default::default()
				}
			);
		}
	}
}

