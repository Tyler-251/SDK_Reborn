use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use bevy_rapier2d::prelude::*;

pub mod flex_load;
use flex_load::*;

pub mod player;
use player::squid::*;

pub mod objects;
use objects::knife_holder::*;

pub mod scenes;
use scenes::background::*;
use scenes::*;

pub mod enemies;
// use enemies::rat::*;

static BACKGROUND_Z: f32 = -100.0;
static PLATFORM_Z: f32 = -50.0;
static PLAYER_Z: f32 = 10.0;

fn main() {
    let mut asset_plugin = AssetLoadPlugin::new();
    asset_plugin.add_asset::<Image>("squid", "squid/squiddy_flat.png");
    asset_plugin.add_asset::<Image>("squid_map", "squid/squid_map3.png");
    asset_plugin.add_asset::<Image>("rat_map", "rat/rat_map.png");
    asset_plugin.add_asset::<Image>("arrow", "squid/squid_arrow_0.png");
    asset_plugin.add_asset::<Image>("knife", "knife/knife.png");
    asset_plugin.add_asset::<Image>("small_knife", "knife/smallknife.png");

    asset_plugin.add_asset::<Image>("sand", "platforms/sand.png");
    asset_plugin.add_asset::<Image>("walls", "walls/walls.png");
    asset_plugin.add_asset::<Image>("knife_holder_base", "knife/knife_holder/knife_holder_base.png");
    asset_plugin.add_asset::<Image>("knife_holder_mask_0", "knife/knife_holder/knife_holder_mask_0.png");
    asset_plugin.add_asset::<Image>("knife_holder_mask_1", "knife/knife_holder/knife_holder_mask_1.png");


    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()), 
        asset_plugin, 
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(64.0), 
        // RapierDebugRenderPlugin::default(),
        SquidPlugin,
        KnifePlugin,
        BackgroundPlugin,
        // RatPlugin
    ));
    app.add_systems(OnEnter(AssetLoadState::Ready), make_platform);
    app.run();
}

fn make_platform (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: loaded.get_typed::<Image>("sand").unwrap(),
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, PLATFORM_Z)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(128., 32.),
        Platform::SOLID,
    ));
}