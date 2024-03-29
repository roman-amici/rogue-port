use bevy_ecs::system::{Res, ResMut};

use crate::{GameResult, MainMenuLayer};

pub fn main_menu_render(mut menu_layer : ResMut<MainMenuLayer>, game_result : Res<GameResult> ){
    
    let title_text =  match *game_result {
        GameResult::Win => "You are a winner! Play again?",
        GameResult::Loss => "Too bad... Play again?",
        GameResult::New => "Get exploring!",
    };

    menu_layer.title = title_text.to_string();
    menu_layer.options.push("<Enter> - Start".to_string());
    menu_layer.options.push("(Q) - Quit".to_string());
}