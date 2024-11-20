use bevy::prelude::*;

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct Simple_Room {
    pub macro_position: IVec2,
    pub room_type: RoomType,
    pub dimensions: Vec<IVec2>, // [[0,0], [1, 0]] makes a 2x1 room
    pub doors: Vec<Door>,
}

impl Simple_Room {
    pub fn add_door(&mut self, door: Door) {
        self.doors.push(door);
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum RoomType {
    Spawn,
    Enemy,
    Treasure,
    Shop,
    Boss
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

