#[allow(unused, unused_imports, unused_mut, unused_variables)]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::flex_load::*;


pub struct RatPlugin;

impl Plugin for RatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_rat);
        app.add_systems(Update, rat_movement.run_if(in_state(AssetLoadState::Ready)));
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

#[derive(Component)]
struct Rat;

#[derive(Component)]
struct DirectionTimer{
    right_face: bool,
    timer: Timer,
}
impl DirectionTimer {
    pub fn new (duration: f32) -> Self {
        Self {
            right_face: true,
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }
}

fn spawn_rat (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let rat_map_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::new(27, 20), 2, 2, None, None)
    );
    // write your code here
    commands.spawn((
        Rat,
        Sprite {
            image: loaded.get_typed_clone::<Image>("rat_map").unwrap(),
            custom_size: Some(Vec2::new(54.0, 40.0)),
            texture_atlas: 
                Some( TextureAtlas {
                    layout: rat_map_layout.clone(),
                    index: 0,
                }),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule_y(3., 20.),
        Velocity::default(),
        Transform::from_translation(Vec3::new(-5.,0., 0.)),
        GravityScale(1.0),
        DirectionTimer::new(2.0),
        LockedAxes::ROTATION_LOCKED

    ));
    }
fn rat_movement(
    time: Res<Time>,
    mut rat_query: Query<(&mut Sprite, &mut Velocity, &mut DirectionTimer), With<Rat>>,
){

    for (mut rat_sprite, mut rat_velocity, mut timer) in rat_query.iter_mut(){
        
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            timer.right_face = !timer.right_face;

        }

        if(timer.right_face){
            rat_velocity.linvel.x = 50.0;
            rat_sprite.flip_x = true;
        } else{
            rat_velocity.linvel.x = -50.0;
            rat_sprite.flip_x = false;
        }    
    }

    



}

