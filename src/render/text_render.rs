use std::collections::HashMap;

use sdl2::{pixels::Color, rect::{Point, Rect}, render::{Canvas, Texture, TextureCreator}, ttf::Font, video::{Window, WindowContext}};

struct TextCache<'a> 
{
    cache_size : usize,
    textures : HashMap<String,Texture<'a>>
}

impl<'a> TextCache<'a> {
    fn new(cache_size : usize) -> Self {
        TextCache {
            cache_size,
            textures : HashMap::new()
        }
    }

    pub fn texture_from_text(&mut self, text : &str, color : Color, texture_creator : &'a TextureCreator<WindowContext>, font : &Font) -> &Texture<'a> {

        if !self.textures.contains_key(text)
        {
            let surface = font
            .render(text)
            .blended(color)
            .expect("could not create surface");
    
            let new_texture = texture_creator.create_texture_from_surface(surface).expect("failed to create texture.");
            self.textures.insert(text.to_string(), new_texture);
        }

        &self.textures[text]
    }
}

pub struct TextRender<'a> {
    text_cache : TextCache<'a>
}

impl<'a> TextRender<'a> {
    pub fn new() -> Self {
        Self {
            text_cache : TextCache::new(64)
        }   
    }

    pub fn add_to_cache(&mut self, text : &str, color : Color, texture_creator : &'a TextureCreator<WindowContext>, font : &Font) {
        self.text_cache.texture_from_text(text, color, texture_creator, font);
    }

    pub fn render_text(&mut self, text : &str, color : Color, location : Rect, canvas : &mut Canvas<Window>, texture_creator : &'a TextureCreator<WindowContext>, font : &Font) {

        let texture = self.text_cache.texture_from_text(text, color, texture_creator, font);

        canvas.copy(texture, None, location).expect("Failed to render text.");
    }

    pub fn render_text_in_cache(&self, text : &str, location : Rect, canvas : &mut Canvas<Window>) {
        let texture = &self.text_cache.textures[text];

        canvas.copy(&texture, None, location).expect("Failed to render text.");
    }

    pub fn texture_dimensions(&self, text : &str) -> Point {
        let texture = self.text_cache.textures.get(text).expect(&format!("Texture for {} was not created.", text));

        let q = texture.query();
        Point::new(q.width as i32, q.height as i32) 
    }
}

