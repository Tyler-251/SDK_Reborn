use bevy::prelude::*;
use crate::{flex_load::*, ParallaxLayer, BACKGROUND_Z};

pub struct WaterSceneBackgroundPlugin;

impl Plugin for WaterSceneBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_background_layers);
    }
}



fn spawn_background_layers (
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
) {
    commands.spawn((
        Sprite {
            image: loaded_assets.get_typed::<Image>("background").unwrap(),
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
        ParallaxLayer {
            speed_x: 0.0,
            speed_y: 0.0,
            offset: Vec2::new(0.0, 0.0),
        },
    ));

    for i in -3..=3 {
        commands.spawn((
            Sprite {
                image: loaded_assets.get_typed::<Image>("background").unwrap(),
                custom_size: Some(Vec2::new(1300.0, 400.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z + 1.0 + (i as f32 * 0.01))),
            ParallaxLayer {
                speed_x: -10.0,
                speed_y: -2.0,
                offset: Vec2::new(1200.0 * (i as f32), -200.0),
            }
        ));

        commands.spawn((
            Sprite {
                image: loaded_assets.get_typed::<Image>("light_beams").unwrap(),
                custom_size: Some(Vec2::new(300.0, 1000.0)),
                color: Color::srgba(1.0, 1.0, 0.9, 0.05),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z + 2.0)),
            ParallaxLayer {
                speed_x: -15.0,
                speed_y: -2.0,
                offset: Vec2::new((1000.0 * (i as f32)) + 300.0, 0.0),
            },
        ));

        commands.spawn((
            Sprite {
                image: loaded_assets.get_typed::<Image>("reef").unwrap(),
                custom_size: Some(Vec2::new(1000.0, 300.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z + 3.0)),
            ParallaxLayer {
                speed_x: -20.0,
                speed_y: -3.0,
                offset: Vec2::new(1000.0 * (i as f32), -220.0),
            },
        ));
        commands.spawn((
            Sprite {
                image: loaded_assets.get_typed::<Image>("watertop").unwrap(),
                custom_size: Some(Vec2::new(375.0, 150.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z + 4.0)),
            ParallaxLayer {
                speed_x: -10.0,
                speed_y: 0.0,
                offset: Vec2::new(375.0 * (i as f32), 300.0),
            },
        ));
    }
}