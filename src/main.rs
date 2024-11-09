use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;

pub mod flex_load;
use flex_load::*;

fn main() {
    let mut asset_plugin = AssetLoadPlugin::new();
    asset_plugin.add_asset::<Image>("squid", "squid/squiddy-1.png");

    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), asset_plugin));
    app.add_systems(OnEnter(AssetLoadState::Ready), setup);
    app.run();

}

fn setup (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    commands.spawn(
        SpriteBundle {
            texture: loaded.get_typed::<Image>("squid").unwrap(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            
            ..Default::default()
        }
    );
    commands.spawn(Camera2dBundle::default());
}
