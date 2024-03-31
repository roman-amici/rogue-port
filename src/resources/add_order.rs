use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct AddOrder {
    next : i32
}

impl AddOrder {
    pub fn next(&mut self) -> i32 {
        self.next += 1;
        self.next
    }
    pub fn get(&self) -> i32 {
        self.next
    }
    
    pub fn new() -> Self {
        Self {
            next : 0
        }
    }
}