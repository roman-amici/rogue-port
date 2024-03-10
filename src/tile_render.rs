use sdl2::{
    image::{InitFlag, LoadTexture},
    rect::Rect,
    render::{Canvas, Texture, TextureQuery},
    video::Window,
    Sdl, VideoSubsystem,
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

pub struct TileRender {
    window_width: u32,
    window_height: u32,
    screen_tile_size: u32,
    canvas: Canvas<Window>,
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
            window_width,
            window_height,
            screen_tile_size,
            canvas,
            graphics,
        })
    }

    pub fn start_batch(&mut self) {
        self.canvas.set_draw_color((0, 0, 0));
        self.canvas.clear();
    }

    pub fn end_batch(&mut self) {
        self.canvas.present();
    }

    pub fn draw_tile_grid(&mut self, col: usize, row: usize, tile_index: usize) {
        let x = col as u32 * self.screen_tile_size;
        let y = row as u32 * self.screen_tile_size;

        self.graphics.render_tile(
            &mut self.canvas,
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
