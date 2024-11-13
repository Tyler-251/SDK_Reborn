use std::time::Instant;
use std::cmp;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player::*;
pub struct BaseMovementPlugin;

impl Plugin for BaseMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_squid, manage_dash));
    }
}


fn control_squid (
    mut player_query: Query<(&mut Velocity, &mut GravityScale, &mut Sprite, &mut Children), With<Player>>,
    mut sprites_query: Query<&mut Sprite, Without<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    if player_query.iter().count() == 0 {return}
    let (mut velocity, mut gravity, mut sprite, player_children) = player_query.single_mut();
    let children = player_children.iter().collect::<Vec<&Entity>>();
    let speed = 170.0;

    let mut movement_vector: Vec2 = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) {
        movement_vector.x -= 1.0;
        sprite.flip_x = true;
        children.iter().for_each(|child| {
            if let Ok(mut child_sprite) = sprites_query.get_mut(**child) {
                child_sprite.flip_x = true;
            }
        });
        
    }
    if input.pressed(KeyCode::KeyD) {
        movement_vector.x += 1.0;
        sprite.flip_x = false;
        children.iter().for_each(|child| {
            if let Ok(mut child_sprite) = sprites_query.get_mut(**child) {
                child_sprite.flip_x = false;
            }
        });
    }

    if movement_vector != Vec2::ZERO {
        let movement = movement_vector.x * speed;
        
        if movement < 0. {
            if velocity.linvel.x > movement {
                velocity.linvel.x = movement;
            } else {
                velocity.linvel.x = velocity.linvel.x * 0.05_f32.powf(time.delta_seconds()); // damping
            }
        }
        if movement > 0. {
            if velocity.linvel.x < movement {
                velocity.linvel.x = movement;
            } else {
                velocity.linvel.x = velocity.linvel.x * 0.05_f32.powf(time.delta_seconds()); // damping
            }
        }
    } else {
        velocity.linvel.x = velocity.linvel.x * 0.05_f32.powf(time.delta_seconds()); // damping
    }

    if input.just_pressed(KeyCode::Space) {
        velocity.linvel.y = 300.0;
    }

    if input.pressed(KeyCode::Space) && velocity.linvel.y > 0.0 {
        gravity.0 = 0.7;
    } else if input.pressed(KeyCode::KeyS) {
        gravity.0 = 2.5;
    } else {
        gravity.0 = 1.3;
    } 

}

fn manage_dash (
    input_stack: ResMut<InputStack>,
    mut player_query: Query<(&mut Velocity, &mut GravityScale), With<Player>>,
) {
    let last_two_inputs = input_stack.into_inner().stack.iter().rev().take(2).collect::<Vec<&(InputDirection, Instant)>>();
    if last_two_inputs.len() != 2 {return}

    let (direction_1, time_1) = last_two_inputs[0].clone();
    let (direction_2, time_2) = last_two_inputs[1].clone();

    if direction_1 == direction_2 {
        match direction_1 {
            InputDirection::Left => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < 0.3 {
                    player_query.single_mut().0.linvel.x = -500.0;
                }
            }
            InputDirection::Right => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < 0.3 {
                    player_query.single_mut().0.linvel.x = 500.0;
                }
            }
            InputDirection::Up => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < 0.3 {
                    player_query.single_mut().0.linvel.y = 500.0;
                }
            }
            InputDirection::Down => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < 0.3 {
                    player_query.single_mut().0.linvel.y = -500.0;
                }
            }
        }
    }


}