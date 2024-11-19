use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use bevy_rapier2d::prelude::*;

pub mod flex_load;
pub mod asset_registry;
use asset_registry::*;

pub mod player_character;
use player_character::player::*;

pub mod objects;
use objects::knife_holder::*;

pub mod scenes;
use scenes::background::*;
use scenes::*;
use scenes::water_scene::*;
use layout::LayoutPlugin;


pub mod enemies;
// use enemies::rat::*;

static BACKGROUND_Z: f32 = -100.0;
static PLATFORM_Z: f32 = -50.0;
static PLAYER_Z: f32 = 10.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()), 
        LoadedAssetsPlugin, // see asset_registry.rs
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(64.0), 
        // RapierDebugRenderPlugin::default(), // physics colliders debug rendering
        LayoutPlugin,
        SquidPlugin,
        WaterScenePlugin,
    ));
    app.run();
}