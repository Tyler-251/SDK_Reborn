use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::{Duration, Instant};
use crate::{player_character::*, scenes::*};
use rand::*;
pub struct BaseMovementPlugin;

impl Plugin for BaseMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DashTimer::new(0.5));
        app.add_systems(Update, ((control_squid, manage_dash).chain(), tick_dash_timer, manage_feet));
    }
}

fn control_squid (
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut player_query: Query<(&mut Player, &mut Velocity, &mut GravityScale, &mut PlayerAnimation, &Transform)>,
    mut splotch_registry: ResMut<SplotchRegistry>,
    input: Res<ButtonInput<KeyCode>>,
    dash_timer: Res<DashTimer>,
    time: Res<Time>,
) {
    if player_query.iter().count() == 0 {return}
    let (mut player_struct, mut velocity, mut gravity, mut player_anim, player_transform) = player_query.single_mut();
    let speed = 170.0;

    let mut movement_vector: Vec2 = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) {
        movement_vector.x -= 1.0;
        player_anim.face = PlayerFace::Left;
        
    }
    if input.pressed(KeyCode::KeyD) {
        movement_vector.x += 1.0;
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

    if input.just_pressed(KeyCode::Space) && player_struct.has_jump {
        velocity.linvel.y = 300.0;
        player_struct.has_jump = false;
        player_struct.grounded = false;
        spawn_splotch(&mut splotch_registry, 50, player_transform.translation.xy() + Vec2::new(0.0, -20.0));
    }

    if input.pressed(KeyCode::Space) && velocity.linvel.y > 0.0 {
        gravity.0 = 0.7;
    } else if input.pressed(KeyCode::KeyS) {
        gravity.0 = 2.5;
    } else {
        gravity.0 = 1.3;
    } 

}

fn manage_feet (
    mut player_query: Query<(&mut Player, &Children)>,
    mut platform_query: Query<Entity, With<Platform>>,
    mut collision_events: EventReader<CollisionEvent>
) {
    if player_query.iter().count() == 0 {return}
    if platform_query.iter().count() == 0 {return}
    let (mut player_struct, player_children) = player_query.single_mut();
    let feet_entity = player_children.iter().nth(1);

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                if (feet_entity == Some(a) || feet_entity == Some(b)) && (platform_query.get_mut(*a).is_ok() || platform_query.get_mut(*b).is_ok()) {
                    player_struct.grounded = true;
                    player_struct.has_jump = true;
                }
            },
            CollisionEvent::Stopped(a, b, _) => {
                if (feet_entity == Some(a) || feet_entity == Some(b)) && (platform_query.get_mut(*a).is_ok() || platform_query.get_mut(*b).is_ok()) {
                    player_struct.grounded = false;
                }
            }
        }
    }
}

fn manage_dash (
    input_stack: ResMut<InputStack>,
    mut player_query: Query<(&mut Velocity, &mut GravityScale, &mut PlayerAnimation, &Transform), With<Player>>,
    mut dash_timer: ResMut<DashTimer>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut splotch_registry: ResMut<SplotchRegistry>
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
                    dash_timer.direction = InputDirection::Left;
                    spawn_splotch_cluster(&mut splotch_registry, 60, player_query.single_mut().3.translation.xy() + Vec2::new(10.0, -10.0));
                }
            }
            InputDirection::Right => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < dash_timer.timer.duration().as_secs_f32() && dash_timer.timer.finished() {
                    player_query.single_mut().0.linvel.x = 500.0;
                    dash_timer.timer.reset();
                    dash_timer.direction = InputDirection::Right;
                    spawn_splotch_cluster(&mut splotch_registry, 60, player_query.single_mut().3.translation.xy() + Vec2::new(-10.0, -10.0));
                }
            }
            InputDirection::Up => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < dash_timer.timer.duration().as_secs_f32() && dash_timer.timer.finished() {
                    player_query.single_mut().0.linvel.y = 500.0;
                    dash_timer.timer.reset();
                    dash_timer.direction = InputDirection::Up;
                    spawn_splotch_cluster(&mut splotch_registry, 60, player_query.single_mut().3.translation.xy() + Vec2::new(0.0, -20.0));
                }
            }
            InputDirection::Down => {
                if time_1.elapsed().as_secs_f32() - time_2.elapsed().as_secs_f32() < 0.1 && time_2.elapsed().as_secs_f32() < dash_timer.timer.duration().as_secs_f32() && dash_timer.timer.finished() {
                    player_query.single_mut().0.linvel.y = -800.0;
                    dash_timer.timer.reset();
                    dash_timer.direction = InputDirection::Down;
                    spawn_splotch_cluster(&mut splotch_registry, 60, player_query.single_mut().3.translation.xy() + Vec2::new(0.0, -20.0));
                }
            }
            
        }
    }

    for (_, _, mut player_anim, _) in player_query.iter_mut() {
        if !dash_timer.timer.finished() {
            player_anim.set_state(AnimState::Dash);
        }
    }
}

#[derive(Resource)]
pub struct DashTimer {
    pub timer: Timer,
    pub direction: InputDirection,
}
impl DashTimer {
    pub fn new (duration: f32) -> Self {
        let mut new_timer = Timer::from_seconds(duration, TimerMode::Once);
        new_timer.set_elapsed(Duration::from_secs_f32(duration));
        Self {
            timer: new_timer,
            direction: InputDirection::Right,
        }
    }
}
fn tick_dash_timer (
    mut dash_timer: ResMut<DashTimer>,
    time: Res<Time>,
    mut splotch_registry: ResMut<SplotchRegistry>,
    player_query: Query<&Transform, With<Player>>
) {
    dash_timer.timer.tick(time.delta());
    for transform in player_query.iter() {
        let mut rng = rand::thread_rng();
        println!("{}", 1.0 - (dash_timer.timer.elapsed().as_secs_f32() / 0.25));
        if dash_timer.timer.elapsed().as_secs_f32() < 0.25 && !dash_timer.timer.finished() && rng.gen_range(0..=6) == 0 {
            spawn_splotch(
                &mut splotch_registry, 
                45 - (dash_timer.timer.elapsed().as_secs_f32() * 120.0) as usize, 
                transform.translation.xy()
            );
        }
    }
}