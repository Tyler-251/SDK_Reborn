use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::flex_load::*;

pub struct KnifePlugin;

impl Plugin for KnifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_knife_holder);
    }
}

fn spawn_knife_holder (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    //make knife here
    commands.spawn( (
        SpriteBundle {
            texture: loaded.get_typed::<Image>("knife_holder_base").unwrap(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0,64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    spawn_knife(&mut commands, &loaded, Vec3::new(-18.0, 10.0, 0.1));
    spawn_knife(&mut commands, &loaded, Vec3::new(-18.0, -10.0, 0.1));

    commands.spawn( (
        SpriteBundle {
            texture: loaded.get_typed::<Image>("knife_holder_mask_0").unwrap(),
            transform: Transform::from_translation(Vec3::new(0.0,0.0,0.2)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )); 

    spawn_knife(&mut commands, &loaded, Vec3::new(-10.0, 13.0, 0.3));
    spawn_knife(&mut commands, &loaded, Vec3::new(-10.0, -1.0, 0.3));
    spawn_knife(&mut commands, &loaded, Vec3::new(-10.0, -13.0, 0.3));

    commands.spawn( (
        SpriteBundle {
            texture: loaded.get_typed::<Image>("knife_holder_mask_1").unwrap(),
            transform: Transform::from_translation(Vec3::new(0.0,0.0,0.4)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )); 
}

#[derive(Component)]
pub struct Knife;

#[derive(Component)]
pub struct KnifeGroup {
    pub knives: Vec<Entity>,
}

fn spawn_knife (
    mut commands: &mut Commands,
    loaded: &Res<LoadedAssets>,
    starting_pos: Vec3
) {
    commands.spawn( (
        Knife,
        SpriteBundle {
            texture: loaded.get_typed::<Image>("knife").unwrap(),
            transform: Transform::from_translation(starting_pos),
            sprite: Sprite {
                custom_size: Some(Vec2::new(62.0,32.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Collider::cuboid(31., 10.),
        Sensor,
    ));
}
