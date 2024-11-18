use bevy::prelude::*;
use crate::flex_load::*;
use super::room::*;
use rand::*;

pub struct LayoutPlugin;

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Layout::new());
        app.add_systems(OnEnter(AssetLoadState::Ready), generate_rooms);
    }
}

#[derive(Resource)]
pub struct Layout {
    pub rooms: Vec<Room>,
    pub populated_dimensions: Vec<UVec2>
}

impl Layout {
    pub fn new() -> Self {
        Layout {
            rooms: vec![],
            populated_dimensions: vec![]
        }
    }
}

fn generate_rooms (
    mut layout: ResMut<Layout>
) {
    layout.rooms.insert(
        0,
        Room {
            macro_position: UVec2::new(0, 0),
            dimensions: vec![UVec2::new(1, 1)],
            doors: vec![]
        }
    );
    for i in 1..=5 {
        let mut rng = rand::thread_rng();
        let starting_room = layout.rooms.get(rng.gen_range(0..layout.rooms.len())).unwrap();
        let mut valid_placement = false; 
        while !valid_placement {
            let direction = rng.gen_range(0..4);
            match direction {
                0 => { // up
                    if !layout.populated_dimensions.contains(starting_room.macro_position + UVec2::new(0, 1)) {
                        continue;
                    }
                },
                1 => { // down

                },
                2 => { // left

                },
                3 => { // right

                },
                _ => {}
            }
        }
    }
}