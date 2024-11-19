use bevy::prelude::*;

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct Room {
    pub macro_position: IVec2,
    pub dimensions: Vec<IVec2>, // [[0,0], [1, 0]] makes a 2x1 room
    pub doors: Vec<Door>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Door {
    pub position: IVec2,
    pub direction: DoorDirection
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DoorDirection {
    Up,
    Down,
    Left,
    Right
}