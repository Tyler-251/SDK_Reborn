use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct KnifePlugin;

impl Plugin for KnifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_knife);
    }
}

fn spawn_knife (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    //make knife here
}
