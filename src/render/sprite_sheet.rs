use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use super::sprite_sheet_info::SpriteSheetInfo;

pub struct SpriteSheet<'a> {
    tile_size_pixels: u32,
    rows: usize,
    cols: usize,
    texture: Texture<'a>,
}

impl<'a> SpriteSheet<'a> {
    pub fn new(
        sprite_info: &SpriteSheetInfo,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        let texture = texture_creator
            .load_texture(&sprite_info.path)
            .expect("Could not find texture");

        let texture_query = texture.query();

        Self {
            tile_size_pixels: sprite_info.tile_size_pixels,
            cols: (texture_query.width / sprite_info.tile_size_pixels) as usize,
            rows: (texture_query.height / sprite_info.tile_size_pixels) as usize,
            texture,
        }
    }

    pub fn render_index(&self, canvas: &mut Canvas<Window>, tile_index: usize, dest: Rect) {
        let xth = (tile_index % self.cols) as u32;
        let yth = (tile_index / self.cols) as u32;

        canvas
            .copy(
                &self.texture,
                Some(Rect::new(
                    (xth * self.tile_size_pixels) as i32,
                    (yth * self.tile_size_pixels) as i32,
                    self.tile_size_pixels,
                    self.tile_size_pixels,
                )),
                Some(dest),
            )
            .expect("Failed to render tile.");
    }
}
