use macroquad::prelude::*;

pub struct Sprite {
	// can you tell im incredibly inconsistent with pub
	texture: Texture2D,
	pub tint: Color,

	pivot: Vec2,

	pub rotation: f32,
	pub rotation_offset: Vec2,

	pub flip_x: bool,
	pub flip_y: bool,
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
		}
	}

	pub fn render(&self, x: f32, y: f32) {
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

