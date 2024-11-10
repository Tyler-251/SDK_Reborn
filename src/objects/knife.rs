use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::flex_load::*;

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
    commands.spawn( (
        SpriteBundle {
            texture: loaded.get_typed::<Image>("knife_holder_base").unwrap(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0,64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
