use bevy::prelude::*;
use crate::flex_load::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_background);
    }
}

fn spawn_background (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let wall_atlas_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 3, None, None)
    );

    for x in -10..10 {
        for y in -10..10 {
            commands.spawn((
                SpriteBundle {
                    texture: loaded.get_typed::<Image>("walls").unwrap(),
                    transform: Transform::from_translation(Vec3::new(x as f32 * 64.0, y as f32 * 64.0, -10.0)),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                TextureAtlas {
                    layout: wall_atlas_layout.clone(),
                    index: 5,
                }
            ));
        }
    }
}