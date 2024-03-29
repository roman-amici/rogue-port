use sdl2::{rect::{Point, Rect}, render::Canvas, video::Window};

use crate::{MainMenuLayer, Viewport};

use super::TextRender;

pub struct MainMenuRender {
    height : i32,
    width : i32,
    height_title : i32,
    height_option : i32,
}

impl MainMenuRender {
    pub fn new(width : i32,height : i32) -> Self {
        Self {
            height,
            width,
            height_title : 64,
            height_option : 48
        }
    }

    pub fn render_menu(&self, canvas : &mut Canvas<Window>, layer :  &MainMenuLayer,  text_render : &TextRender ){
        let title_size = text_render.texture_dimensions(&layer.title);
        let center = Point::new(self.width / 2, self.height /2);

        let width_title = (title_size.x as f32 / title_size.y as f32) * self.height_title as f32;
        let top_left = center - (Point::new(width_title as i32, self.height_title) / 2);        
        let title_box = Rect::new(top_left.x, top_left.y, width_title as u32, self.height_title as u32);

        text_render.render_text_in_cache(&layer.title, title_box, canvas);

        let mut start_y = title_box.y + title_box.height() as i32 + 5;
        for option in layer.options.iter() {
            let option_size = text_render.texture_dimensions(option);
            let width_option = (option_size.x as f32 / option_size.y as f32) * self.height_option as f32;
            let x = center.x - (width_option / 2.0) as i32;
            let option_box = Rect::new(x, start_y, width_option as u32, self.height_option as u32);
            
            text_render.render_text_in_cache(option, option_box, canvas);
            start_y = start_y + self.height_option + 5;
        }
    }
}