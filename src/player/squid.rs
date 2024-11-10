use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier};
use crate::flex_load::*;

pub struct SquidPlugin;

impl Plugin for SquidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_squid);
        app.add_systems(Update, control_squid.run_if(in_state(AssetLoadState::Ready)));
    }
}

#[derive(Component)] 
pub struct Player;

fn spawn_squid (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: loaded.get_typed::<Image>("squid").unwrap(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
            
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(25.0, 26.0),
        Velocity::default(),
        GravityScale(1.0),
        Friction {
            coefficient: 0.5,
            ..default()
        },
        LockedAxes::ROTATION_LOCKED,
        Player
    ));
    commands.spawn(Camera2dBundle::default());
}

fn control_squid (
    mut player_query: Query<(&mut Velocity, &mut GravityScale, &mut Sprite), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.iter().count() == 0 {return}
    let (mut velocity, mut gravity, mut sprite) = player_query.single_mut();
    let speed = 170.0;

    let mut movement_vector: Vec2 = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) {
        movement_vector.x -= 1.0;
        sprite.flip_x = true;
    }
    if input.pressed(KeyCode::KeyD) {
        movement_vector.x += 1.0;
        sprite.flip_x = false;
    }

    if movement_vector != Vec2::ZERO {
        velocity.linvel = Vec2::new(movement_vector.x * speed, velocity.linvel.y)
    } else {
        velocity.linvel = Vec2::new(velocity.linvel.x.clamp(-100., 100.), velocity.linvel.y)
    }

    if input.just_pressed(KeyCode::Space) {
        velocity.linvel.y = 300.0;
    }

    if input.pressed(KeyCode::Space) && velocity.linvel.y > 0.0 {
        gravity.0 = 0.7;
    } else if input.pressed(KeyCode::KeyS) {
        gravity.0 = 2.0;
    } else {
        gravity.0 = 1.5;
    } 

}