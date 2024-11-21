use bevy::prelude::*;
use rand::*;
use super::complex_layout::*;
use crate::flex_load::AssetLoadState;

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
    display_query: Query<(&FauxDisplay, Entity)>,
) {
    if display_query.iter().count() > 0 {
        return;
    }
    for room in layout.rooms.iter() {
        let mut rng = rand::thread_rng();
        let room_color: Color;
        match room.room_type {
            ComplexRoomType::Enemy => {
                room_color = Color::srgb(0.0, 1.0, 0.0);
            },
            ComplexRoomType::Boss => {
                room_color = Color::srgb(1.0, 0.0, 1.0);
            },
            ComplexRoomType::Shop => {
                room_color = Color::srgb(1.0, 1.0, 1.0);
            },
            ComplexRoomType::Treasure => {
                room_color = Color::srgb(1.0, 1.0, 0.0);
            },
            ComplexRoomType::Spawn => {
                room_color = Color::srgb(0.0, 0.0, 1.0);
            },
        }
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
            if room.chunks.contains(&IVec2::new(chunk.x + 1, chunk.y)) { //right
                commands.spawn((
                    FauxDisplay,
                    SpriteBundle {
                        sprite: Sprite {
                            color: room_color,
                            custom_size: Some(Vec2::new(10.0, 90.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(((chunk.x) as f32 * 100.0) + 50.0, chunk.y as f32 * 100.0, 0.0)),
                        ..default()
                    }
                ));
            }
            if room.chunks.contains(&IVec2::new(chunk.x - 1, chunk.y)) { //left
                commands.spawn((
                    FauxDisplay,
                    SpriteBundle {
                        sprite: Sprite {
                            color: room_color,
                            custom_size: Some(Vec2::new(10.0, 90.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(((chunk.x) as f32 * 100.0) - 50.0, chunk.y as f32 * 100.0, 0.0)),
                        ..default()
                    }
                ));
            }
            if room.chunks.contains(&IVec2::new(chunk.x, chunk.y + 1)) { //up
                commands.spawn((
                    FauxDisplay,
                    SpriteBundle {
                        sprite: Sprite {
                            color: room_color,
                            custom_size: Some(Vec2::new(90.0, 10.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(chunk.x as f32 * 100.0, ((chunk.y) as f32 * 100.0) + 50.0, 0.0)),
                        ..default()
                    }
                ));
            }
            if room.chunks.contains(&IVec2::new(chunk.x, chunk.y - 1)) { //down
                commands.spawn((
                    FauxDisplay,
                    SpriteBundle {
                        sprite: Sprite {
                            color: room_color,
                            custom_size: Some(Vec2::new(90.0, 10.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(chunk.x as f32 * 100.0, ((chunk.y) as f32 * 100.0) - 50.0, 0.0)),
                        ..default()
                    }
                ));
            }
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
                        color: Color::srgb(1.0, 0.0, 0.0),
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
    fn generate_specific_room (
        starting_chunk: IVec2,
        room_dimensions: UVec2,
        layout: &ComplexLayout,
        room_type: ComplexRoomType
    ) -> ComplexRoom {
        let mut rng = rand::thread_rng();
        let mut possible_positions: Vec<Vec<IVec2>> = vec![];
        for x in 0..room_dimensions.x {
            for y in 0..room_dimensions.y {
                let mut possible_position: Vec<IVec2> = vec![];
                for sub_x in 0..room_dimensions.x {
                    for sub_y in 0..room_dimensions.y {
                        possible_position.push(IVec2::new(x as i32 - sub_x as i32, y as i32 - sub_y as i32));
                    }
                }
                possible_positions.push(possible_position);
            }
        }

        let mut valid_positions: Vec<Vec<IVec2>> = vec![];
        for possible_position in possible_positions.iter() {
            let mut valid = true;
            for chunk in possible_position.iter() {
                if layout.get_populated_chunks().contains(&IVec2::new(chunk.x + starting_chunk.x, chunk.y + starting_chunk.y)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                valid_positions.push(possible_position.iter().map(|chunk| {
                    IVec2::new(chunk.x + starting_chunk.x, chunk.y + starting_chunk.y)
                }).collect::<Vec<_>>());
            }
        }
        if valid_positions.len() == 0 {
            println!("No valid positions found for room");
            return ComplexRoom::new(vec![]);
        } else {
            let chosen_position = &valid_positions[rng.gen_range(0..valid_positions.len())];
            return ComplexRoom {
                chunks: chosen_position.clone(),
                doors: vec![],
                room_type,
            };
        }
    }
    let mut rng = rand::thread_rng();
    let shop_room_count = 1;
    let treasure_room_count = 3;
    let max_rooms = 15 + shop_room_count + treasure_room_count;
    let spawn = ComplexRoom {
        chunks: vec![IVec2::new(0, 0)],
        doors: vec![],
        room_type: ComplexRoomType::Spawn,
    };
    let mut active_room = spawn.clone();
    layout.rooms.push(spawn);
    while layout.rooms.len() < max_rooms {
        let neighbors = active_room.all_neighboring_chunks();

        let valid_placements = neighbors.iter().filter(|&neighbor| {
            layout.chunk_to_room(*neighbor).is_none()
        }).collect::<Vec<_>>();

        if valid_placements.len() == 0 { // if room is blocked in, pick a new room and start over
            active_room = layout.rooms[rng.gen_range(0..layout.rooms.len())].clone();
            // println!("this should not happen often");
            continue;
        }

        let new_chunk = *valid_placements[rng.gen_range(0..valid_placements.len())];
        let new_room: ComplexRoom;
        if layout.rooms.len() == max_rooms - 1 { //last room is boss room
            new_room = generate_specific_room(new_chunk, UVec2::new(3, 2), &layout, ComplexRoomType::Boss);
            if new_room.chunks.len() == 0 {
                active_room = layout.rooms[rng.gen_range(0..layout.rooms.len())].clone();
                continue;
            }
        } else if layout.rooms.len() % (max_rooms / (shop_room_count + 1)) == 0 { //shop room
            new_room = generate_specific_room(new_chunk, UVec2::new(2, 1), &layout, ComplexRoomType::Shop);
            if new_room.chunks.len() == 0 {
                active_room = layout.rooms[rng.gen_range(0..layout.rooms.len())].clone();
                continue;
            }
        } else if layout.rooms.len() > max_rooms - (treasure_room_count + 2) { //add treasure rooms (todo: must be randomly spread across map)
            new_room = generate_specific_room(new_chunk, UVec2::new(1, 1), &layout, ComplexRoomType::Treasure);
            if new_room.chunks.len() == 0 {
                active_room = layout.rooms[rng.gen_range(0..layout.rooms.len())].clone();
                continue;
            }
        } 
        else {
            new_room = generate_room(new_chunk, rng.gen_range(1..=3), &layout);
        }
        layout.rooms.push(new_room.clone());
        layout.insert_door(active_room.clone(), new_room);
    }
    // add more doors
    for room in layout.rooms.clone().iter() {
        for other_room in layout.rooms.clone().iter() {
            if room.is_adjacent(other_room) && rng.gen_bool(0.5) {
                layout.insert_door(room.clone(), other_room.clone());
            }
        }
    }
}