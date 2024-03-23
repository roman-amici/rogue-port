use sdl2::{rect::{Point, Rect}, render::Canvas, video::Window};

use crate::{HudElement, HudLayer, Viewport};

use super::{TextRender, TileRender};

pub struct HudRender {
    pub tooltip_height : i32,
    pub health_bar_height : i32,
    pub tile_screen_size : u32,
    pub viewport : Viewport
}

impl HudRender {

    pub fn new(tile_screen_size : u32, viewport : Viewport) -> Self {
        Self {
            tooltip_height : (tile_screen_size / 2) as i32,
            health_bar_height : tile_screen_size as i32,
            tile_screen_size,
            viewport
        }
    }

    pub fn render_hud_layer(
        &self,
        hud_layer : &HudLayer,
        canvas : &mut Canvas<Window>,
        text_render : &mut TextRender,
    ) {
        let mut used_text = vec![];
        for element in hud_layer.hud_elements.iter() {
    
            match element {
                HudElement::Tooltip { tile_space_index, text , ..} => {
                    used_text.push(text.clone());

                    self.render_tooltip(tile_space_index, text, canvas, text_render);
                },
                HudElement::HealthBar { fraction, text } => {
                    used_text.push(text.clone());

                    self.render_healthbar(*fraction, text, canvas, text_render);
                } 
            }

        }
    }

    fn text_dimensions(height : i32, text : &str, text_render : & TextRender ) -> Point {
        let dimensions = text_render.texture_dimensions(text);
        let width_height_ratio: f32 = (dimensions.x as f32) / (dimensions.y as f32);
        let width = (width_height_ratio * (height as f32)) as i32;

        Point::new(width, height)
    }

    fn center_x(text_dimensions : &Point, center : Point) -> Rect {
        let x = center.x - (text_dimensions.x / 2);
        let y = center.y;

        Rect::new(x,y, text_dimensions.x as u32, text_dimensions.y as u32)
    }

    fn render_tooltip(&self, tile_point : &Point, text : &str,  canvas : &mut Canvas<Window>, text_render : &TextRender){

        let tile_start = TileRender::tile_to_screen_space(self.tile_screen_size, tile_point.y, tile_point.x);
        let center_x = tile_start.x + (self.tile_screen_size / 2) as i32;

        let text_dimensions = Self::text_dimensions(self.tooltip_height, text, text_render);
        let text_box = Self::center_x(&text_dimensions, Point::new(center_x, tile_start.y));

        canvas.set_draw_color((255, 255, 0 ));
        canvas.fill_rect(text_box).expect("failed to render rect.");

        text_render.render_text_in_cache(text, text_box, canvas);
    }

    fn render_healthbar(&self, fraction : f32, text : &str,  canvas : &mut Canvas<Window>, text_render : & TextRender ) {
        let center_x = (self.viewport.width_tiles * self.tile_screen_size) / 2;

        let text_dimensions = Self::text_dimensions(self.health_bar_height, text, text_render);
        let text_box = Self::center_x(&text_dimensions, Point::new(center_x as i32, 0));

        // Draw back color
        let screen_width = self.viewport.width_tiles * self.tile_screen_size;
        canvas.set_draw_color((128, 0, 32));
        canvas.fill_rect(Rect::new(0,0, screen_width, self.health_bar_height as u32)).expect("Failed to draw healthbar");

        let colored_width = (screen_width as f32 * fraction) as u32;
        canvas.set_draw_color((255,0,0));
        canvas.fill_rect(Rect::new(0, 0, colored_width, text_box.height())).expect("Failed to draw healthbar");

        text_render.render_text_in_cache(text, text_box, canvas);
    }
}

