
use std::collections::HashMap;

use sdl2::{
    image::LoadTexture, rect::Rect, render::{Canvas, Texture, TextureCreator}, ttf::Font, video::Window, VideoSubsystem
};

pub struct SpriteSheetInfo {
    pub tile_size_pixels: u32,
    pub path: String,
}

struct SpriteSheet {
    tile_size_pixels: u32,
    rows: usize,
    cols: usize,
    sprite_texture: Texture,
}

impl SpriteSheet {
    fn render_tile(&self, canvas: &mut Canvas<Window>, tile_index: usize, dest: Rect) {
        let xth = (tile_index % self.cols) as u32;
        let yth = (tile_index / self.cols) as u32;

        canvas
            .copy(
                &self.sprite_texture,
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

pub struct Render {
    window_width: u32,
    window_height: u32,
    canvas: Canvas<Window>,
    tile_render : TileRender,
    text_render : TextRender,
}

impl Render {
    pub fn start_batch(&mut self) {
        self.canvas.set_draw_color((0, 0, 0));
        self.canvas.clear();
    }

    pub fn end_batch(&mut self) {
        self.canvas.present();
    }
}

pub struct StringTexture {
    text : String,
    texture : Texture
}

pub struct TextRender {
    texture_creator : TextureCreator<Window>,
    font : Font<'a>,
    texture_cache : HashMap<u64, StringTexture>,
    next_index : u64,
}

impl TextRender {

    fn create_string_texture(texture_creator : &mut TextureCreator<Window>, text : &str) {
        texture_creator
    }

    pub fn update_entry(&mut self, index : Option<u64>, text : &str) -> u64 {
        if let Some(index) = index {
            if let Some(cache_entry) = self.texture_cache.get_mut(&index) {
                if cache_entry.text != text {
                    unsafe {
                        cache_entry.texture.destroy();
                    }

                    cache_entry.texture = create_string_texture(text);
                }
            }
        }
    }

    pub fn render_text(&mut self, canvas : &mut Canvas<Window>, index : Option<u64>, text : &str, x : i32, y : i32) -> u64 {
        
        let mut index = self.next_index;


    }
}

pub struct TileRender {
    screen_tile_size: u32,
    graphics: SpriteSheet,
}

impl TileRender {
    pub fn new(
        sprite_info: SpriteSheetInfo,
        video_subsystem: &VideoSubsystem,
        screen_tile_size : u32,
    ) -> Result<Self, String> {
        let window_width = 40 * screen_tile_size;
        let window_height = 20 * screen_tile_size;
        let window = video_subsystem
            .window("dungeon", window_width, window_height)
            .position_centered()
            .build()
            .or_else(|x| Err(x.to_string()))?;

        let canvas = window
            .into_canvas()
            .build()
            .or_else(|e| Err(e.to_string()))?;

        let texture_creator = canvas.texture_creator();
        let sprite_texture = texture_creator.load_texture(&sprite_info.path)?;
        let texture_query = sprite_texture.query();

        let graphics = SpriteSheet {
            cols: (texture_query.width / sprite_info.tile_size_pixels) as usize,
            rows: (texture_query.height / sprite_info.tile_size_pixels) as usize,
            sprite_texture,
            tile_size_pixels: sprite_info.tile_size_pixels,
        };

        Ok(TileRender {
            screen_tile_size,
            graphics,
        })
    }

    pub fn draw_tile_grid(&self, canvas : &mut Canvas<Window>, col: usize, row: usize, tile_index: usize) {
        let x = col as u32 * self.screen_tile_size;
        let y = row as u32 * self.screen_tile_size;

        self.graphics.render_tile(
            canvas,
            tile_index,
            Rect::new(
                x as i32,
                y as i32,
                self.screen_tile_size,
                self.screen_tile_size,
            ),
        );
    }
}
