#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    width_tiles: usize,
    height_tiles: usize,

    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width_tiles: usize, height_tiles: usize) -> Self {
        Self {
            width_tiles,
            height_tiles,
            tiles: vec![TileType::Floor; width_tiles * height_tiles],
        }
    }

    pub fn map_index(&self, x: usize, y: usize) -> usize {
        self.width_tiles * y + x
    }

    pub fn map_index_to_coords(&self, index: usize) -> (usize, usize) {
        let x = index % self.width_tiles;
        let y = index / self.width_tiles;

        (x, y)
    }
}
