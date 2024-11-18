use bevy::prelude::*;

#[derive(Component)]
pub struct Room {
    pub macro_position: UVec2,
    pub dimensions: Vec<UVec2>, // [[0,0], [1, 0]] makes a 2x1 room
    pub doors: Vec<Door>,
}

pub struct Door {
    pub position: UVec2,
    pub direction: DoorDirection
}

pub enum DoorDirection {
    Up,
    Down,
    Left,
    Right
}