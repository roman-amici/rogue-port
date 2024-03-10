use std::time::Duration;

use sdl2::image::{InitFlag, LoadTexture};
use tile_render::{SpriteSheetInfo, TileRender};

mod map;
mod tile_render;

mod prelude {
    pub use crate::map::*;
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;

    let info = SpriteSheetInfo {
        path: "dungeonfont.png".to_string(),
        tile_size_pixels: 32,
    };
    let mut renderer = TileRender::new(info, &video_subsystem)?;

    renderer.fill(35);
    loop {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
