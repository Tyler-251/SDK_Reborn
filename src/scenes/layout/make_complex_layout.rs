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
            // println!("this should not happen often");
            continue;
        }

        let new_chunk = *valid_placements[rng.gen_range(0..valid_placements.len())];
        let new_room = generate_room(new_chunk, rng.gen_range(1..=3), &layout);
        println!("new room: {:?}", new_room.get_permutation());
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