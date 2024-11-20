use bevy::{color, prelude::*, transform::commands};
use crate::flex_load::*;
use super::room::*;
use rand::*;

pub struct LayoutPlugin;

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Layout::new());
        app.add_systems(OnEnter(AssetLoadState::Ready), (generate_rooms, spawn_placeholders).chain());
    }
}

#[derive(Resource, Clone)]
pub struct Layout {
    pub rooms: Vec<Simple_Room>,
}

impl Layout {
    pub fn new() -> Self {
        Layout {
            rooms: vec![],
        }
    }
    pub fn get_populated_dimensions(&self) -> Vec<IVec2> {
        let mut dimensions = vec![];
        for room in &self.rooms {
            dimensions.push(room.macro_position);
        }
        return dimensions;
    }
    pub fn get_adjacent_rooms(&self, room: &Simple_Room) -> Vec<Simple_Room> {
        let mut adjacent_rooms = vec![];
        for other_room in &self.rooms {
            if room.macro_position == other_room.macro_position {continue;}
            if room.macro_position == other_room.macro_position + IVec2::new(0, 1) {adjacent_rooms.push(other_room.clone());}
            if room.macro_position == other_room.macro_position + IVec2::new(1, 0) {adjacent_rooms.push(other_room.clone());}
            if room.macro_position == other_room.macro_position + IVec2::new(0, -1) {adjacent_rooms.push(other_room.clone());}
            if room.macro_position == other_room.macro_position + IVec2::new(-1, 0) {adjacent_rooms.push(other_room.clone());}
        }
        return adjacent_rooms;
    }
}

fn generate_rooms (
    mut layout: ResMut<Layout>
) {
    layout.rooms.insert(
        0,
        Simple_Room {
            room_type: RoomType::Spawn,
            macro_position: IVec2::new(0, 0),
            dimensions: vec![IVec2::new(0, 0)],
            doors: vec![]
        }
    );
    let room_count = 10;
    let mut rng = rand::thread_rng();
    let starting_room = layout.rooms.get(rng.gen_range(0..layout.rooms.len())).unwrap();
    let mut active_room = starting_room.clone();
    for i in 0..room_count {
        // println!("Placing room {}", i);
        let mut valid_placement = false; 
        let mut counter = 0;
        while !valid_placement { //todo: better way to iterate through directions than random every loop
            valid_placement = true;
            let new_position: IVec2;

            if counter > 10 {
                active_room = layout.rooms.get(rng.gen_range(0..layout.rooms.len())).unwrap().clone(); //switch to diff room
                println!("Switching to room at {:?}", active_room.macro_position);
            }

            let direction_index = rng.gen_range(0..4);
            let direction: IVec2;
            if direction_index == 0 {direction =  IVec2::new(0, 1);}  //up
            else if direction_index == 1 {direction =  IVec2::new(1, 0);}  //right
            else if direction_index == 2 {direction =  IVec2::new(0, -1);}  //down
            else {direction =  IVec2::new(-1, 0);}  //left

            new_position = active_room.macro_position + direction;
            
            for position in layout.get_populated_dimensions() {  //check for overlaps
                if position == new_position {
                    valid_placement = false;
                }
            }

            if valid_placement {
                let door_direction: DoorDirection;
                match direction_index {
                    2 => door_direction = DoorDirection::Up,
                    3 => door_direction = DoorDirection::Right,
                    0 => door_direction = DoorDirection::Down,
                    1 => door_direction = DoorDirection::Left,
                    _ => panic!("Invalid door direction")
                }

                let mut new_room = Simple_Room {
                    room_type: RoomType::Enemy,
                    macro_position: new_position,
                    dimensions: vec![IVec2::new(0, 0)],
                    doors: vec![Door {
                        position: IVec2::new(0, 0),
                        direction: door_direction
                    }]
                };
                active_room = new_room.clone();

                if i == room_count - 1 {
                    new_room.room_type = RoomType::Boss;
                }
    
                layout.rooms.push(
                    new_room.clone()
                );
            }
            counter += 1;              
        }
        if i == room_count / 3 {
            build_branch(&mut layout, &active_room, 3);
        }
        if i == (room_count * 2) / 3 {
            build_branch(&mut layout, &active_room, 5);
        }

    }
    add_extra_doors(&mut layout);
}

fn build_branch (
    layout: &mut ResMut<Layout>,
    starting_room: &Simple_Room,
    branch_length: i32
) {
    let mut rng = rand::thread_rng();
    let mut active_room = starting_room.clone();
    for i in 0..branch_length {
        // println!("Placing room {}", i);
        let mut valid_placement = false; 
        let mut counter = 0;
        while !valid_placement { //todo: better way to iterate through directions than random every loop
            valid_placement = true;
            let new_position: IVec2;

            if counter > 10 {
                active_room = layout.rooms.get(rng.gen_range(0..layout.rooms.len())).unwrap().clone(); //switch to diff room
                println!("Switching to room at {:?}", active_room.macro_position);
            }

            let direction_index = rng.gen_range(0..4);
            let direction: IVec2;
            if direction_index == 0 {direction =  IVec2::new(0, 1);}  //up
            else if direction_index == 1 {direction =  IVec2::new(1, 0);}  //right
            else if direction_index == 2 {direction =  IVec2::new(0, -1);}  //down
            else {direction =  IVec2::new(-1, 0);}  //left

            new_position = active_room.macro_position + direction;
            
            for position in layout.get_populated_dimensions() {  //check for overlaps
                if position == new_position {
                    valid_placement = false;
                }
            }

            if valid_placement {
                let door_direction: DoorDirection;
                match direction_index {
                    2 => door_direction = DoorDirection::Up,
                    3 => door_direction = DoorDirection::Right,
                    0 => door_direction = DoorDirection::Down,
                    1 => door_direction = DoorDirection::Left,
                    _ => panic!("Invalid door direction")
                }

                let new_room = Simple_Room {
                    room_type: RoomType::Enemy,
                    macro_position: new_position,
                    dimensions: vec![IVec2::new(0, 0)],
                    doors: vec![Door {
                        position: IVec2::new(0, 0),
                        direction: door_direction
                    }]
                };
                active_room = new_room.clone();
                
                layout.rooms.push(
                    new_room.clone()
                );
            }
            counter += 1;              
        }
    }
}

fn add_extra_doors (
    layout: &mut ResMut<Layout>
) {
    let adjacent_rooms_list: Vec<Vec<Simple_Room>> = layout.rooms.iter().map(|room| layout.get_adjacent_rooms(room)).collect();
    for (room, adjacent_rooms) in layout.rooms.iter_mut().zip(adjacent_rooms_list) {
        if adjacent_rooms.len() == 0 {continue;}
        for adjacent_room in adjacent_rooms {
            let mut door_direction: DoorDirection = DoorDirection::Up;
            if adjacent_room.macro_position == room.macro_position + IVec2::new(0, 1) {door_direction = DoorDirection::Up;}
            if adjacent_room.macro_position == room.macro_position + IVec2::new(1, 0) {door_direction = DoorDirection::Right;}
            if adjacent_room.macro_position == room.macro_position + IVec2::new(0, -1) {door_direction = DoorDirection::Down;}
            if adjacent_room.macro_position == room.macro_position + IVec2::new(-1, 0) {door_direction = DoorDirection::Left;}
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..4) == 0 {
                println!("Adding extra door");
                room.doors.push(Door {
                    position: IVec2::new(0, 0),
                    direction: door_direction
                });
            }
        }
    }
}

fn spawn_placeholders (
    mut commands: Commands,
    layout: Res<Layout>,
) {
    for room in &layout.rooms {
        let color: Color;
        match room.room_type {
            RoomType::Spawn => color = Color::srgba(0.0, 1.0, 0.0, 0.5),
            RoomType::Enemy => color = Color::srgba(1.0, 0.0, 0.0, 0.5),
            RoomType::Treasure => color = Color::srgba(1.0, 1.0, 0.0, 0.5),
            RoomType::Shop => color = Color::srgba(0.0, 1.0, 1.0, 0.5),
            RoomType::Boss => color = Color::srgba(1.0, 0.0, 1.0, 0.5),
        }
        // println!("room at {:?}", room.macro_position);
        commands.spawn(
            SpriteBundle {
                // texture: loaded_assets.get_typed::<Image>("placeholder").unwrap(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    color,
                    ..default()
                },
                transform: Transform::from_translation(room.macro_position.extend(0).as_vec3() * 105.0),
                ..default()
            }
        );

        if room.doors.len() == 0 {continue;}
        for door in room.doors.clone() {
            let door_position: Vec2;
            match door.direction {
                DoorDirection::Up => door_position = Vec2::new(0.0, 50.0),
                DoorDirection::Down => door_position = Vec2::new(0.0, -50.0),
                DoorDirection::Left => door_position = Vec2::new(-50.0, 0.0),
                DoorDirection::Right => door_position = Vec2::new(50.0, 0.0),
            }
    
            commands.spawn(
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        color: Color::srgba(1.0, 1.0, 1.0, 0.5),
                        ..default()
                    },
                    transform: Transform::from_translation((room.macro_position.extend(0).as_vec3() * 105.0) + door_position.extend(0.0)),
                    ..default()
                }
            );
        }
    }
}