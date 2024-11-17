use bevy::prelude::*;

use super::{sand_platform::*, water_background::WaterSceneBackgroundPlugin};
use crate::{flex_load::*, PLATFORM_Z, scenes::*};

pub struct WaterScenePlugin;

impl Plugin for WaterScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WaterSceneBackgroundPlugin, ParallaxPlugin));
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_four_platforms);
    }
}

fn spawn_four_platforms (
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
) {
    spawn_sand_platform(&mut commands, &loaded_assets, Vec3::new(0.0, -64.0, PLATFORM_Z));
    spawn_sand_platform(&mut commands, &loaded_assets, Vec3::new(-500.0, 228.0, PLATFORM_Z));
    spawn_sand_platform(&mut commands, &loaded_assets, Vec3::new(500.0, 228.0, PLATFORM_Z));
    spawn_sand_platform(&mut commands, &loaded_assets, Vec3::new(0.0, 556.0, PLATFORM_Z));
}