use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use bevy_rapier2d::prelude::*;

pub mod flex_load;
use flex_load::*;

fn main() {
    let mut asset_plugin = AssetLoadPlugin::new();
    asset_plugin.add_asset::<Image>("squid", "squid/squiddy-1.png");
    asset_plugin.add_asset::<Image>("knife", "knife/knife.png");
    asset_plugin.add_asset::<Image>("sand", "platforms/sand.png");

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()), 
        asset_plugin, 
        RapierPhysicsPlugin::<NoUserData>::default(), 
        RapierDebugRenderPlugin::default(),
    ));
    app.add_systems(OnEnter(AssetLoadState::Ready), setup);
    app.run();

}

fn setup (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: loaded.get_typed::<Image>("squid").unwrap(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
            
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(25.0, 26.0),
        Velocity::default(),
        GravityScale(1.0),
    ));
    commands.spawn(
        SpriteBundle {
            texture: loaded.get_typed::<Image>("knife").unwrap(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(62.0,32.0)),
                ..Default::default()
            },
            ..Default::default()
        }
    );
    commands.spawn((
        SpriteBundle {
            texture: loaded.get_typed::<Image>("sand").unwrap(),
            transform: Transform::from_translation(Vec3::new(0.0, -250.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(128.0, 32.0),
    ));
    commands.spawn(Camera2dBundle::default());
}

