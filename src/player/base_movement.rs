use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::{Duration, Instant};
use crate::player::*;
pub struct BaseMovementPlugin;

impl Plugin for BaseMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DashTimer::new(0.5));
        app.add_systems(Update, ((control_squid, manage_dash).chain(), tick_dash_timer));
    }
}

fn control_squid (
    mut player_query: Query<(&mut Velocity, &mut GravityScale, &mut Sprite, &mut PlayerAnimation), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    dash_timer: Res<DashTimer>,
    time: Res<Time>,
) {
    if player_query.iter().count() == 0 {return}
    let (mut velocity, mut gravity, mut sprite, mut player_anim) = player_query.single_mut();
    let speed = 170.0;

    let mut movement_vector: Vec2 = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) {
        movement_vector.x -= 1.0;
        sprite.flip_x = true;
        player_anim.face = PlayerFace::Left;
        
    }
    if input.pressed(KeyCode::KeyD) {
        movement_vector.x += 1.0;
        sprite.flip_x = false;
        player_anim.face = PlayerFace::Right;
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
        if dash_timer.timer.finished() {
            player_anim.set_state(AnimState::Walk);
        }
    } else {
        velocity.linvel.x = velocity.linvel.x * 0.05_f32.powf(time.delta_seconds()); // damping
        if dash_timer.timer.finished() {
            player_anim.set_state(AnimState::Idle);
        }
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
    mut player_query: Query<(&mut Velocity, &mut GravityScale, &mut PlayerAnimation), With<Player>>,
    mut dash_timer: ResMut<DashTimer>,
) {
    let last_two_inputs = input_stack.into_inner().stack.iter().rev().take(2).collect::<Vec<&(InputDirection, Instant)>>();
    if last_two_inputs.len() != 2 {return}

    let (direction_1, time_1) = last_two_inputs[0].clone();
    let (direction_2, time_2) = last_two_inputs[1].clone();

    if direction_1 == direction_2 {
        match direction_1 {
            InputDirection::Left => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < dash_timer.timer.duration().as_secs_f32() && dash_timer.timer.finished() {
                    player_query.single_mut().0.linvel.x = -500.0;
                    dash_timer.timer.reset();
                }
            }
            InputDirection::Right => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < dash_timer.timer.duration().as_secs_f32() && dash_timer.timer.finished() {
                    player_query.single_mut().0.linvel.x = 500.0;
                    dash_timer.timer.reset();
                }
            }
            InputDirection::Up => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < dash_timer.timer.duration().as_secs_f32() && dash_timer.timer.finished() {
                    player_query.single_mut().0.linvel.y = 500.0;
                    dash_timer.timer.reset();
                }
            }
            InputDirection::Down => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < dash_timer.timer.duration().as_secs_f32() && dash_timer.timer.finished() {
                    player_query.single_mut().0.linvel.y = -800.0;
                    dash_timer.timer.reset();
                }
            }
        }
    }

    for (_, _, mut player_anim) in player_query.iter_mut() {
        println!("{}", dash_timer.timer.elapsed_secs());
        if !dash_timer.timer.finished() {
            player_anim.set_state(AnimState::Dash);
        }
    }
}

#[derive(Resource)]
pub struct DashTimer {
    pub timer: Timer,
}
impl DashTimer {
    pub fn new (duration: f32) -> Self {
        let mut new_timer = Timer::from_seconds(duration, TimerMode::Once);
        new_timer.set_elapsed(Duration::from_secs_f32(duration));
        Self {
            timer: new_timer,
        }
    }
}
fn tick_dash_timer (
    mut dash_timer: ResMut<DashTimer>,
    time: Res<Time>,
) {
    dash_timer.timer.tick(time.delta());

}