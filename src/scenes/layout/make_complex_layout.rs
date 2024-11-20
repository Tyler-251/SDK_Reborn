use core::panic;

use bevy::{prelude::*, transform::commands};
use rand::*;

use crate::{flex_load::AssetLoadState, layout::room};

pub struct ComplexLayoutPlugin;

impl Plugin for ComplexLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ComplexLayout::new());
        app.add_systems(OnEnter(AssetLoadState::Ready), generate);
        app.add_systems(Update, (manually_gen, render_ghost_rooms).chain());
    }
}

pub fn manually_gen (
    mut commands: Commands,
    mut layout: ResMut<ComplexLayout>,
    inputs: Res<ButtonInput<KeyCode>>,
    mut display_query: Query<(&FauxDisplay, Entity)>,
) {
    if inputs.just_pressed(KeyCode::KeyF) {
        layout.rooms = vec![];
        for (_display, entity) in display_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
        generate(layout);
    }
}



#[derive(Component)]
pub struct FauxDisplay;

pub fn render_ghost_rooms (
    mut commands: Commands,
    layout: ResMut<ComplexLayout>,
    mut display_query: Query<(&FauxDisplay, Entity)>,
) {
    if display_query.iter().count() > 0 {
        return;
    }
    for room in layout.rooms.iter() {
        let mut rng = rand::thread_rng();
        let room_color = Color::srgb(
            rng.gen_range(0.0..1.0), 
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0)
        );
        for chunk in room.chunks.iter() {
            commands.spawn((
                FauxDisplay,
                SpriteBundle {
                    sprite: Sprite {
                        color: room_color,
                        custom_size: Some(Vec2::new(90.0, 90.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(chunk.x as f32 * 100.0, chunk.y as f32 * 100.0, 0.0)),
                    ..default()
                }
            ));
        }
        for door in room.doors.iter() {
            let from = door.from.as_vec2();
            let to = door.to.as_vec2();
            let midpoint = from.lerp(to, 0.5);
            let direction = to - from;
            let angle = direction.y.atan2(direction.x);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        color: Color::srgb(0.0, 1.0, 0.0),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(midpoint.x as f32 * 100.0, midpoint.y as f32 * 100.0, 0.0),
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    ..default()
                },
                FauxDisplay
            ));
        }
    }
}

pub fn generate (
    mut layout: ResMut<ComplexLayout>
) {
    fn generate_room (
        starting_chunk: IVec2,
        room_size: usize,
        layout: &ComplexLayout
    ) -> ComplexRoom {
        let mut rng = rand::thread_rng();
        let mut room_chunks = vec![starting_chunk];
        let mut active_chunk = starting_chunk;
        while room_chunks.len() < room_size {
            let all_surrounding_chunks = get_surrounding_chunks(active_chunk);
            let valid_chunks = all_surrounding_chunks.iter().filter(|&chunk| {
                !room_chunks.contains(chunk) && !layout.get_populated_chunks().contains(chunk)
            }).collect::<Vec<_>>();
            if valid_chunks.len() == 0 {
                break;
            }
            let new_chunk = *valid_chunks[rng.gen_range(0..valid_chunks.len())];
            room_chunks.push(new_chunk);
            active_chunk = new_chunk;
        }
        return ComplexRoom::new(room_chunks);
    }
    let mut rng = rand::thread_rng();
    let spawn = ComplexRoom::new(vec![IVec2::new(0, 0)]);
    let mut active_room = spawn.clone();
    layout.rooms.push(spawn);
    while layout.rooms.len() < 15 {
        let neighbors = active_room.all_neighboring_chunks();

        let valid_placements = neighbors.iter().filter(|&neighbor| {
            layout.chunk_to_room(*neighbor).is_none()
        }).collect::<Vec<_>>();

        if valid_placements.len() == 0 { // if room is blocked in, pick a new room and start over
            active_room = layout.rooms[rng.gen_range(0..layout.rooms.len())].clone();
            println!("this should not happen often");
            continue;
        }

        let new_chunk = *valid_placements[rng.gen_range(0..valid_placements.len())];
        let new_room = generate_room(new_chunk, rng.gen_range(1..=3), &layout);
        layout.rooms.push(new_room.clone());
        layout.insert_door(active_room.clone(), new_room);
    }
    // add more doors
    for room in layout.rooms.clone().iter() {
        for other_room in layout.rooms.clone().iter() {
            if room.is_adjacent(other_room) && rng.gen_bool(0.3) {
                layout.insert_door(room.clone(), other_room.clone());
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComplexRoom {
    pub chunks: Vec<IVec2>,
    pub doors: Vec<ComplexRoomDoor>,
}

impl ComplexRoom {
    pub fn new(chunks: Vec<IVec2>) -> Self {
        Self {
            chunks,
            doors: vec![],
        }
    }
    pub fn add_door(&mut self, door: ComplexRoomDoor) {
        if self.doors.contains(&door) {
            return;
        }
        self.doors.push(door);
    }
    pub fn all_neighboring_chunks(&self) -> Vec<IVec2> {
        let mut chunk_list: Vec<IVec2> = vec![];

        for chunk in self.chunks.iter() {
            let north = IVec2::new(chunk.x, chunk.y + 1);
            let south = IVec2::new(chunk.x, chunk.y - 1);
            let east = IVec2::new(chunk.x + 1, chunk.y);
            let west = IVec2::new(chunk.x - 1, chunk.y);

            if !self.chunks.contains(&north) && !chunk_list.contains(&north) {
                chunk_list.push(north);
            }
            if !self.chunks.contains(&south) && !chunk_list.contains(&south) {
                chunk_list.push(south);
            }
            if !self.chunks.contains(&east) && !chunk_list.contains(&east) {
                chunk_list.push(east);
            }
            if !self.chunks.contains(&west) && !chunk_list.contains(&west) {
                chunk_list.push(west);
            }
        }

        return chunk_list;
    }
    fn is_adjacent(&self, other: &ComplexRoom) -> bool {
        for chunk in self.chunks.iter() {
            for other_chunk in other.chunks.iter() {
                if chunk.as_vec2().distance(other_chunk.as_vec2()) == 1.0 {
                    return true;
                }
            }
        }
        return false;
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComplexRoomDoor {
    pub from: IVec2,
    pub to: IVec2,
}

#[derive(Resource, Clone)]
pub struct ComplexLayout {
    pub rooms: Vec<ComplexRoom>,
}

impl ComplexLayout {
    pub fn new() -> Self {
        Self {
            rooms: vec![],
        }
    }
    pub fn get_populated_chunks(&self) -> Vec<IVec2> {
        self.rooms.iter().flat_map(|room| room.chunks.clone()).collect()
    }
    pub fn chunk_to_room(&self, chunk: IVec2) -> Option<&ComplexRoom> {
        self.rooms.iter().find(|room| room.chunks.contains(&chunk))
    }
    pub fn insert_door_by_chunks(&mut self, chunk_a: IVec2, chunk_b: IVec2) {
        let room_a = self.chunk_to_room(chunk_a).unwrap().clone(); 
        let room_b = self.chunk_to_room(chunk_b).unwrap().clone(); 
        self.rooms.iter_mut().for_each(|room| {
            if room.chunks == room_a.chunks {
                room.add_door(ComplexRoomDoor {
                    from: chunk_a,
                    to: chunk_b,
                });
            }
            if room.chunks == room_b.chunks {
                room.add_door(ComplexRoomDoor {
                    from: chunk_b,
                    to: chunk_a,
                });
            }
        });
    }
    pub fn insert_door(&mut self, room_a: ComplexRoom, room_b: ComplexRoom) {
        // find list of chunks that are directly next to each other in the two rooms and pick one to doorify
        let mut rng = rand::thread_rng();
        let mut possible_doors = vec![];
        for chunk_a in room_a.chunks.iter() {
            for chunk_b in room_b.chunks.iter() {
                if chunk_a.as_vec2().distance(chunk_b.as_vec2()) == 1.0 {
                    possible_doors.push((*chunk_a, *chunk_b));
                }
            }
        }
        if possible_doors.len() == 0 {
            panic!("No possible doors found between rooms");
        }
        let choice = possible_doors[rng.gen_range(0..possible_doors.len())];
        self.insert_door_by_chunks(choice.0, choice.1);
    }
    
}

fn random_unit_vector() -> IVec2 {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..4);
    match roll {
        0 => IVec2::new(0, 1),
        1 => IVec2::new(0, -1),
        2 => IVec2::new(1, 0),
        3 => IVec2::new(-1, 0),
        _ => panic!("Invalid roll, this should never happen"),
    }
}

fn get_surrounding_chunks(chunk: IVec2) -> Vec<IVec2> {
    vec![
        IVec2::new(chunk.x, chunk.y + 1),
        IVec2::new(chunk.x, chunk.y - 1),
        IVec2::new(chunk.x + 1, chunk.y),
        IVec2::new(chunk.x - 1, chunk.y),
    ]
}

