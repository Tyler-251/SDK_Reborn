use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{flex_load::*, player_character::player::*, scenes::*};

pub struct KnifePlugin;

impl Plugin for KnifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), init_test);
        app.add_systems(Update, (tick_knife_holders, handle_knife_collisions));
    }
}

pub fn init_test (
    commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    spawn_knife_holder(commands, loaded, Vec2::X * 200.,Quat::from_rotation_z(0.), KnifeHolder::new(2.0));
}

// #region Knife Holder
#[derive(Component)]
pub struct KnifeDebugText;

#[derive(Component, Clone)]
pub struct KnifeHolder{
    pub timer: Timer,
    pub index: i8,
    debug: bool
}


impl Default for KnifeHolder {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            index: -1,
            debug: false,
        }
    }
}

impl KnifeHolder {
    pub fn new (seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
            index: 0,
            debug: false,
        }
    }
    pub fn new_debug (seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
            index: 0,
            debug: true,
        }
    }
}

fn tick_knife_holders (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    time: Res<Time>,
    mut query: Query<(&mut KnifeHolder, &Children, Entity)>,
    mut knife_query: Query<(&mut Velocity, &mut KnifeHolderKnife, &Transform, &GlobalTransform, Entity), Without<Player>>,
    mut text_query: Query<&mut Text, With<KnifeDebugText>>,
) {
    // knife holder behavior
    for (mut knife_holder, children, entity) in query.iter_mut() {
        knife_holder.timer.tick(time.delta());
        if knife_holder.timer.finished() {
            knife_holder.index = (knife_holder.index + 1) % 5;
            
            for child in children.iter() {
                if let Ok((_, mut knife_struct, knife_transform, _,_)) = knife_query.get_mut(*child) {
                    if knife_struct.index == knife_holder.index && knife_struct.state == KnifeState::Waiting {
                        knife_struct.state = KnifeState::Shooting;
                        let old_position = knife_transform.translation;
                        commands.entity(entity).with_children(|parent| {
                            spawn_creeping_knife(parent, loaded.get_typed("small_knife").unwrap(), old_position, knife_struct.index);
                        });
                    }
                }
                if let Ok(mut text) = text_query.get_mut(*child) {
                    text.0 = format!("{}", knife_holder.index);
                }
            }
        }
    }
    // knife behavior
    for (mut knife_velocity, mut knife_struct, knife_transform, knife_global, knife_entity) in knife_query.iter_mut() {
        if knife_struct.ttl <= 0.0 {
            commands.entity(knife_entity).despawn_recursive();
        } else {
            knife_struct.ttl -= time.delta_secs();
        }

        //make unit direction vector
        let base_movement = Vec2::NEG_X;
        let rotation: Quat = knife_global.to_scale_rotation_translation().1;
        let movement = rotation.mul_vec3(base_movement.extend(0.0));
        let movement = movement.normalize().truncate();
        

        if knife_struct.state == KnifeState::Creeping && knife_transform.translation.x > knife_struct.target.x && knife_struct.ttl > knife_struct.start_ttl - 2.5 {
            //pass
        } else if knife_struct.state == KnifeState::Creeping && knife_transform.translation.x > knife_struct.target.x {
            knife_velocity.linvel = movement * 10.0;
        } else if knife_struct.state == KnifeState::Creeping {
            knife_struct.state = KnifeState::Waiting;
            knife_velocity.linvel = Vec2::ZERO;
        } else if knife_struct.state == KnifeState::Shooting {
            knife_velocity.linvel = movement * 300.0;
        }

    }
}
// #endregion

pub fn spawn_knife_holder (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    position: Vec2,
    rotation: Quat,
    knife_holder: KnifeHolder,
) {
    //knife base
    commands.spawn( (
        knife_holder.clone(),
        Sprite {
            image: loaded.get_typed::<Image>("knife_holder_base").unwrap(),
            custom_size: Some(Vec2::new(64.0,64.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.)) * Transform::from_rotation(rotation),
        Collider::cuboid(30.0, 32.0),
        Platform::SOLID,
    )).with_children(|parent| {
        //knife holder mask 0
        parent.spawn( ( 
            Sprite {
                image: loaded.get_typed::<Image>("knife_holder_mask_0").unwrap(),
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0,0.0,0.2))
        )); 
        //knife holder mask 1
        parent.spawn( ( 
            Sprite {
                image: loaded.get_typed::<Image>("knife_holder_mask_1").unwrap(),
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0,0.0,0.4)),
        ));
        //back knives
        spawn_ready_knife(parent, loaded.get_typed("small_knife").unwrap(), Vec3::new(-25.0, 10.0, 0.1), 0);
        spawn_ready_knife(parent, loaded.get_typed("small_knife").unwrap(), Vec3::new(-25.0, -10.0, 0.1), 4);

        //front knives
        spawn_ready_knife(parent, loaded.get_typed("small_knife").unwrap(), Vec3::new(-17.0, 13.0, 0.3), 3);
        spawn_ready_knife(parent, loaded.get_typed("small_knife").unwrap(), Vec3::new(-17.0, -1.0, 0.3), 2);
        spawn_ready_knife(parent, loaded.get_typed("small_knife").unwrap(), Vec3::new(-17.0, -13.0, 0.3), 1);

        if knife_holder.debug {
            parent.spawn((
                KnifeDebugText,
                Text2d("test".to_string()),
                TextColor(Color::BLACK),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)),
            ));
        }
    });
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum KnifeState {
    Creeping, // slowly moving out of holder
    Waiting, // waiting to be fired
    Shooting // fired
}

#[derive(Component)]
pub struct KnifeHolderKnife {
    pub index: i8,
    pub start_ttl: f32,
    pub ttl: f32,
    pub state: KnifeState,
    target: Vec3,
}

impl KnifeHolderKnife {
    pub fn new (index: i8, ttl: f32, state: KnifeState, target: Vec3) -> Self {
        Self {
            index,
            start_ttl: ttl,
            ttl,
            state,
            target,
        }
    }
}

fn spawn_knife (
    child_builder: &mut ChildBuilder,
    sprite: Handle<Image>,
    target_pos: Vec3,
    index: i8,
    state: KnifeState,
) {
    let starting_pos: Vec3;
    if state == KnifeState::Creeping {
        starting_pos = target_pos + Vec3::new(20.0, 0.0, 0.0);
    } else {
        starting_pos = target_pos;
    }
    child_builder.spawn( (
        KnifeHolderKnife::new(index, 20.0, state, target_pos),
        Sprite {
            image: sprite,
            custom_size: Some(Vec2::new(50.0,10.0)),
            ..default()
        },
        Transform::from_translation(starting_pos),
        RigidBody::KinematicVelocityBased,
        Velocity::default(),
        Collider::cuboid(0., 0.), // must have collider to have velocity
        Sensor 
    )).with_children(|knife_parent| {
        knife_parent.spawn( (
            Collider::cuboid(14., 5.),
            Sensor,
            Transform::from_translation(Vec3::new(-10.0, 0.0, 0.0)),
        ));
    });
}

fn spawn_ready_knife (
    child_builder: &mut ChildBuilder,
    sprite: Handle<Image>,
    starting_pos: Vec3,
    index: i8,
) {
    spawn_knife(child_builder, sprite, starting_pos, index, KnifeState::Waiting);
}

fn spawn_creeping_knife (
    child_builder: &mut ChildBuilder,
    sprite: Handle<Image>,
    target_pos: Vec3,
    index: i8,
) {
    spawn_knife(child_builder, sprite, target_pos, index, KnifeState::Creeping);
}

fn handle_knife_collisions (
    mut collisions: EventReader<CollisionEvent>,
    mut knife_query: Query<(&mut KnifeHolderKnife, &Transform, &Children, Entity)>,
    mut player_query: Query<(&mut Player, Entity), Without<KnifeHolderKnife>>,
    mut commands: Commands
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(a, b, _) => {
                for (knife_struct, _, knife_children, knife_entity) in knife_query.iter_mut() {
                    let collider_child = knife_children.get(0).unwrap();
                    if (a == collider_child || b == collider_child) && knife_struct.state == KnifeState::Shooting {
                        let (mut player, player_entity) = player_query.single_mut();
                        if *b == player_entity || *a == player_entity {
                            player.health.damage(10.0);
                            commands.entity(knife_entity).despawn_recursive();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}