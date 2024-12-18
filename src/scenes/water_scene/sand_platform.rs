use bevy::prelude::*;
use crate::scenes::platform::*;
use bevy_rapier2d::prelude::*;
use crate::flex_load::*;

pub fn spawn_sand_platform (
    commands: &mut Commands,
    loaded_assets: &Res<LoadedAssets>,
    position: Vec3,
) {

    commands.spawn((
        Sprite {
            image: loaded_assets.get_typed::<Image>("sand").unwrap(),
            custom_size: Some(Vec2::new(256.0, 64.0)),
            ..default()
        },
        Transform::from_translation(position),
        RigidBody::Fixed,
        Collider::cuboid(128., 32.),
        Platform::SOLID,
));
}