use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use bevy_rapier2d::prelude::*;

pub mod flex_load;
use flex_load::*;

pub mod player {
    pub mod squid;
    pub mod ui;
}
use player::squid::*;

pub mod objects {
    pub mod knife;
    pub mod background;
}
use objects::knife::*;
use objects::background::*;

static BACKGROUND_Z: f32 = -100.0;
static PLATFORM_Z: f32 = -50.0;
static PLAYER_Z: f32 = 10.0;

fn main() {
    let mut asset_plugin = AssetLoadPlugin::new();
    asset_plugin.add_asset::<Image>("squid", "squid/squiddy_flat.png");
    asset_plugin.add_asset::<Image>("arrow", "squid/squid_arrow_0.png");
    asset_plugin.add_asset::<Image>("knife", "knife/knife.png");
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
        BackgroundPlugin,
    ));
    app.run();
}