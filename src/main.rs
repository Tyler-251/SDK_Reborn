use bevy::{asset, prelude::*};
use bevy::render::texture::ImagePlugin;
use bevy_rapier2d::prelude::*;

pub mod flex_load;
use flex_load::*;

pub mod player_character;
use player_character::player::*;

pub mod objects;
use objects::knife_holder::*;

pub mod scenes;
use scenes::background::*;
use scenes::*;
use scenes::water_scene::*;

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

    asset_plugin.add_asset::<Image>("background", "waterscene/background/background.png");
    asset_plugin.add_asset::<Image>("reef", "waterscene/background/reef.png");
    asset_plugin.add_asset::<Image>("watertop", "waterscene/background/watertop.png");
    asset_plugin.add_asset::<Image>("reef_far", "waterscene/background/far_coral.png");
    asset_plugin.add_asset::<Image>("light_beams", "waterscene/background/light_beams.png");

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
        WaterScenePlugin,
        // RatPlugin
    ));
    app.run();
}