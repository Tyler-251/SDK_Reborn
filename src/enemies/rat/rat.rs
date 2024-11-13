use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::flex_load::*;

pub struct RatPlugin;

impl Plugin for RatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_rat);
    }
}

// spawn a rat at -5,0 
//
// you will need to learn how to use a texture atlas (reference squid.rs)
//  - the rat spritesheet is in assets/enemies/rat.png imported as "rat_map"
//
// you will need to learn how to use a physics body 
//  - the rat should have a RigidBody::KinematicVelocityBased, a collider, and velocity (all of these are components in bevy_rapier2d)
//
// (advanced 1) make the rat move back and forth with a timer component
// (advanced 2) make the rat face the direction it is moving
// (advanced 3) make the rat collide with the ground but pass through the player
// (advanced 4) animate rat with the texture atlas and a timer component (do this in the rat_animation.rs file import it here)

fn spawn_rat (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // write your code here
}