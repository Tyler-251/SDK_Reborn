use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, MaterialMesh2dBundle}};
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
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: loaded_assets.get_typed::<Image>("background").unwrap(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1280.0, 720.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
            ..default()
        },
        ParallaxLayer {
            speed_x: 0.0,
            speed_y: 0.0,
            offset: Vec2::new(0.0, 0.0),
        },
    ));

    for i in -3..=3 {
        commands.spawn((
            SpriteBundle {
                texture: loaded_assets.get_typed::<Image>("reef_far").unwrap(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1300.0, 400.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z + 1.0 + (i as f32 * 0.01))),
                ..default()
            },
            ParallaxLayer {
                speed_x: -10.0,
                speed_y: -2.0,
                offset: Vec2::new(1200.0 * (i as f32), -200.0),
            },
        ));

        commands.spawn((
            SpriteBundle {
                texture: loaded_assets.get_typed::<Image>("light_beams").unwrap(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(300.0, 1000.0)),
                    color: Color::srgba(1.0, 1.0, 0.9, 0.05),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z + 2.0)),
                ..default()
            },
            ParallaxLayer {
                speed_x: -15.0,
                speed_y: -2.0,
                offset: Vec2::new((1000.0 * (i as f32)) + 300.0, 0.0),
            },
        ));

        commands.spawn((
            SpriteBundle {
                texture: loaded_assets.get_typed::<Image>("reef").unwrap(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1000.0, 300.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z + 3.0)),
                ..default()
            },
            ParallaxLayer {
                speed_x: -20.0,
                speed_y: -3.0,
                offset: Vec2::new(1000.0 * (i as f32), -220.0),
            },
        ));
        commands.spawn((
            SpriteBundle {
                texture: loaded_assets.get_typed::<Image>("watertop").unwrap(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(375.0, 150.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z + 4.0)),
                ..default()
            },
            ParallaxLayer {
                speed_x: -10.0,
                speed_y: 0.0,
                offset: Vec2::new(375.0 * (i as f32), 300.0),
            },
        ));
    }
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct WaterMaterial {
    #[uniform(0)]
    color: LinearRgba,

    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

impl Material2d for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "waterscene/watertopshader.wgsl".into()
    }
}