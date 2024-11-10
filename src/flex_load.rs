use bevy::prelude::*;
use std::collections::HashMap;
use bevy::asset::{LoadState, UntypedHandle};

pub struct AssetLoadPlugin(LoadedAssets);

impl AssetLoadPlugin {
    pub fn new() -> Self {
        return Self(LoadedAssets::new());
    }
    pub fn add_asset<T: Asset>(&mut self, name: &str, path: &str) {
        self.0.add_asset::<T>(name, path);
    }
}

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone());
        app.init_state::<AssetLoadState>();
        app.add_systems(Startup, load);
        app.add_systems(Update, verify_load.run_if(in_state(AssetLoadState::Loading)));
    }
}

#[derive(Resource, Clone)]
pub struct LoadedAssets {
    pub asset_map: HashMap<String, UntypedHandle>,
    asset_links: HashMap<String, String>,
    load_functions: HashMap<String, AssetLoader>,
    count_loaded: usize,
}

type AssetLoader = fn(&AssetServer, String) -> UntypedHandle;
fn load_function<T: Asset>(asset_server: &AssetServer, path: String) -> UntypedHandle {
    return asset_server.load::<T>(path).into();
}

impl LoadedAssets {
    pub fn new() -> Self {
        Self {
            asset_map: HashMap::default(),
            asset_links: HashMap::default(),
            load_functions: HashMap::default(),
            count_loaded: 0,
        }
    }

    fn add_asset<T: Asset>(&mut self, name: &str, path: &str) {
        self.asset_links.insert(name.to_string(), path.to_string());
        self.load_functions.insert(name.to_string(), load_function::<T>);
    }
    /// Get the number of assets that have been added
    pub fn count(&self) -> usize {
        return self.asset_links.len();
    }
    /// Get the number of assets that have been loaded
    pub fn count_loaded(&self) -> usize {
        return self.count_loaded;
    }
    /// Get the progress of loading assets as a float between 0 and 1
    pub fn load_progress(&self) -> f32 {
        return self.count_loaded as f32 / self.count() as f32;
    }
    /// Get an untyped handle to an asset by name
    pub fn get_untyped(&self, name: &str) -> Option<UntypedHandle> {
        return self.asset_map.get(name).cloned();
    }
    /// Get a typed handle to an asset by name
    /// 
    /// This will panic if the asset was added with Type A and you try to get it as Type B
    pub fn get_typed<T: Asset>(&self, name: &str) -> Option<Handle<T>> {
        return self.get_untyped(name).map(|handle| handle.typed::<T>());
    }

    pub fn get_untyped_clone(&self, name: &str) -> Option<UntypedHandle> {
        return self.get_untyped(name).map(|handle| handle.clone());
    }
    
    pub fn get_typed_clone<T: Asset>(&self, name: &str) -> Option<Handle<T>> {
        return self.get_typed(name).map(|handle| handle.clone());
    }

}

#[derive(States, Hash, Eq, PartialEq, Clone, Debug, Default)]
pub enum AssetLoadState {
    #[default] Loading,
    Ready,
}

fn load (
    asset_server: Res<AssetServer>,
    mut loaded_assets: ResMut<LoadedAssets>,
) {
    for (name, path) in loaded_assets.asset_links.clone().iter() {
        let handle: UntypedHandle = loaded_assets.load_functions[name](&asset_server, path.to_string());
        loaded_assets.asset_map.insert(name.to_string(), handle);
    }
}

fn verify_load (
    mut loaded_assets: ResMut<LoadedAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AssetLoadState>>
) {
    let all_loaded = loaded_assets.asset_map.values().all(|handle| asset_server.get_load_state(handle.id()) == Some(LoadState::Loaded));
    loaded_assets.count_loaded = loaded_assets.asset_map.values().filter(|handle| asset_server.get_load_state(handle.id()) == Some(LoadState::Loaded)).count();
    if all_loaded {
        next_state.set(AssetLoadState::Ready);
    }
}
