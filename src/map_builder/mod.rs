mod cellular_automata_architect;
mod empty_architect;
mod map_builder;
mod random_walk_architect;
mod rooms_architect;

use rand::{Rng, RngCore};

pub use map_builder::MapBuilder;

use self::{
    cellular_automata_architect::CellularAutomataArchitect, empty_architect::EmptyArchitect,
    random_walk_architect::RandomWalkArchitect, rooms_architect::RoomsArchitect,
};

pub trait MapArchitect {
    fn new(&mut self, width: usize, height: usize, rng: &mut dyn RngCore) -> MapBuilder;
}

pub fn random_architect(rng: &mut dyn RngCore) -> Box<dyn MapArchitect> {
    match rng.gen_range(0..50) {
        _ => Box::new(RandomWalkArchitect {}),
    }
}
