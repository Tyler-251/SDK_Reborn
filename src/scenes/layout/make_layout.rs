use bevy::{prelude::*, transform::commands};
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

#[derive(Resource)]
pub struct Layout {
    pub rooms: Vec<Room>,
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
}

fn generate_rooms (
    mut layout: ResMut<Layout>
) {
    layout.rooms.insert(
        0,
        Room {
            macro_position: IVec2::new(0, 0),
            dimensions: vec![IVec2::new(0, 0)],
            doors: vec![]
        }
    );

    let mut rng = rand::thread_rng();
    let starting_room = layout.rooms.get(rng.gen_range(0..layout.rooms.len())).unwrap();
    let mut active_room = starting_room.clone();
    for i in 0..10 {
        println!("Placing room {}", i);
        let mut valid_placement = false; 
        let mut counter = 0;
        while !valid_placement { //todo: better way to iterate through directions than random every loop
            valid_placement = true;
            let new_position: IVec2;

            if counter > 10 {panic!("Could not place room after 10 attempts");}

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

                let new_room = Room {
                    macro_position: new_position,
                    dimensions: vec![IVec2::new(0, 0)],
                    doors: vec![Door {
                        position: IVec2::new(0, 0),
                        direction: door_direction
                    }]
                };
                active_room = new_room.clone();
    
                layout.rooms.insert(
                    i+1,
                    new_room.clone()
                );
            }
            counter += 1;              
        }

    }

}



fn spawn_placeholders (
    mut commands: Commands,
    layout: Res<Layout>,
) {
    for room in &layout.rooms {
        println!("room at {:?}", room.macro_position);
        commands.spawn(
            SpriteBundle {
                // texture: loaded_assets.get_typed::<Image>("placeholder").unwrap(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                    ..default()
                },
                transform: Transform::from_translation(room.macro_position.extend(0).as_vec3() * 105.0),
                ..default()
            }
        );

        if room.doors.len() == 0 {continue;}
        let door_position: Vec2;
        match room.doors[0].direction {
            DoorDirection::Up => door_position = Vec2::new(0.0, 50.0),
            DoorDirection::Down => door_position = Vec2::new(0.0, -50.0),
            DoorDirection::Left => door_position = Vec2::new(-50.0, 0.0),
            DoorDirection::Right => door_position = Vec2::new(50.0, 0.0),
        }

        commands.spawn(
            SpriteBundle {
                // texture: loaded_assets.get_typed::<Image>("placeholder").unwrap(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    color: Color::srgba(1.0, 0.0, 0.0, 0.5),
                    ..default()
                },
                transform: Transform::from_translation((room.macro_position.extend(0).as_vec3() * 105.0) + door_position.extend(0.0)),
                ..default()
            }
        );
    }
}