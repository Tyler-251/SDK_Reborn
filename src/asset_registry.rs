use bevy::prelude::*;
use crate::flex_load::*;

pub struct LoadedAssetsPlugin;

impl Plugin for LoadedAssetsPlugin {
    fn build(&self, app: &mut App) {
        let mut asset_plugin = AssetLoadPlugin::new();
        asset_plugin.add_asset::<Image>("squid", "squid/squiddy_flat.png");
        asset_plugin.add_asset::<Image>("squid_map", "squid/squid_map3.png");
        asset_plugin.add_asset::<Image>("rat_map", "rat/rat_map.png");
        asset_plugin.add_asset::<Image>("arrow", "squid/squid_arrow_0.png");
        asset_plugin.add_asset::<Image>("knife", "knife/knife.png");
        asset_plugin.add_asset::<Image>("small_knife", "knife/smallknife.png");
    
        asset_plugin.add_asset::<Image>("background", "waterscene/background/background.png");
        asset_plugin.add_asset::<Image>("reef", "waterscene/background/preef.png");
        asset_plugin.add_asset::<Image>("watertop", "waterscene/background/watertop.png");
        asset_plugin.add_asset::<Image>("reef_far", "waterscene/background/far_coral.png");
        asset_plugin.add_asset::<Image>("light_beams", "waterscene/background/light_beams.png");
    
        asset_plugin.add_asset::<Image>("sand", "platforms/sand.png");
        asset_plugin.add_asset::<Image>("walls", "walls/walls.png");
        asset_plugin.add_asset::<Image>("knife_holder_base", "knife/knife_holder/knife_holder_base.png");
        asset_plugin.add_asset::<Image>("knife_holder_mask_0", "knife/knife_holder/knife_holder_mask_0.png");
        asset_plugin.add_asset::<Image>("knife_holder_mask_1", "knife/knife_holder/knife_holder_mask_1.png");
        
        app.add_plugins(asset_plugin);
    }
}