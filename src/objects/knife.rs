use bevy::{log::tracing_subscriber::fmt::time, prelude::*};
use bevy_rapier2d::{na, prelude::*};
use crate::{flex_load::*, player::squid::*};

pub struct KnifePlugin;

impl Plugin for KnifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), init_test);
        app.add_systems(Update, (tick_knife_holders, handle_knife_collisions));
    }
}

pub fn init_test (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    spawn_knife_holder(commands, loaded, Vec2::X * 200., KnifeHolder::new(2.0));
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
    mut knife_query: Query<(&mut Velocity, &mut Knife, &Transform, Entity), Without<Player>>,
    player_query: Query<(&Transform, &Player, &Collider), Without<Knife>>,
    mut text_query: Query<&mut Text, With<KnifeDebugText>>,
) {
    // knife holder behavior
    for (mut knife_holder, children, entity) in query.iter_mut() {
        knife_holder.timer.tick(time.delta());
        if knife_holder.timer.finished() {
            knife_holder.index = (knife_holder.index + 1) % 5;
            
            for child in children.iter() {
                if let Ok((_, mut knife_struct, knife_transform, _,)) = knife_query.get_mut(*child) {
                    if knife_struct.index == knife_holder.index && knife_struct.state == KnifeState::Waiting {
                        knife_struct.state = KnifeState::Shooting;
                        let old_position = knife_transform.translation;
                        commands.entity(entity).with_children(|parent| {
                            spawn_creeping_knife(parent, loaded.get_typed("knife").unwrap(), old_position, knife_struct.index);
                        });
                    }
                }
                if let Ok(mut text) = text_query.get_mut(*child) {
                    text.sections[0].value = format!("{}", knife_holder.index);
                }
            }
        }
    }
    // knife behavior
    for (mut knife_velocity, mut knife_struct, knife_transform, knife_entity) in knife_query.iter_mut() {
        if knife_struct.ttl <= 0.0 {
            commands.entity(knife_entity).despawn();
        } else {
            knife_struct.ttl -= time.delta_seconds();
        }

        if knife_struct.state == KnifeState::Creeping && knife_transform.translation.x > knife_struct.target.x && knife_struct.ttl > knife_struct.start_ttl - 2.5 {
            //pass
        } else if knife_struct.state == KnifeState::Creeping && knife_transform.translation.x > knife_struct.target.x {
            knife_velocity.linvel = Vec2::NEG_X * 10.0;
        } else if knife_struct.state == KnifeState::Creeping {
            knife_struct.state = KnifeState::Waiting;
            knife_velocity.linvel = Vec2::ZERO;
        } else if knife_struct.state == KnifeState::Shooting {
            knife_velocity.linvel = Vec2::NEG_X * 300.0;
        }

    }
}
// #endregion

pub fn spawn_knife_holder (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    position: Vec2,
    knife_holder: KnifeHolder,
) {
    //knife base
    commands.spawn( (
        knife_holder.clone(),
        SpriteBundle {
            texture: loaded.get_typed::<Image>("knife_holder_base").unwrap(),
            transform: Transform::from_translation(position.extend(0.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0,64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Collider::cuboid(30.0, 32.0),
    )).with_children(|parent| {
        //knife holder mask 0
        parent.spawn( ( 
            SpriteBundle {
                texture: loaded.get_typed::<Image>("knife_holder_mask_0").unwrap(),
                transform: Transform::from_translation(Vec3::new(0.0,0.0,0.2)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        )); 
        //knife holder mask 1
        parent.spawn( ( 
            SpriteBundle {
                texture: loaded.get_typed::<Image>("knife_holder_mask_1").unwrap(),
                transform: Transform::from_translation(Vec3::new(0.0,0.0,0.4)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
        //back knives
        spawn_ready_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-18.0, 10.0, 0.1), 0);
        spawn_ready_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-18.0, -10.0, 0.1), 4);

        //front knives
        spawn_ready_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-10.0, 13.0, 0.3), 3);
        spawn_ready_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-10.0, -1.0, 0.3), 2);
        spawn_ready_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-10.0, -13.0, 0.3), 1);

        if knife_holder.debug {
            parent.spawn((
                KnifeDebugText,
                Text2dBundle {
                    text: Text::from_section("test", TextStyle {
                        color: Color::BLACK,
                        ..default()
                    }),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)),
                    ..Default::default()
                }
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
pub struct Knife {
    pub index: i8,
    pub start_ttl: f32,
    pub ttl: f32,
    pub state: KnifeState,
    target: Vec3,
}

impl Knife {
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
        starting_pos = target_pos + Vec3::new(25.0, 0.0, 0.0);
    } else {
        starting_pos = target_pos;
    }
    child_builder.spawn( (
        Knife::new(index, 20.0, state, target_pos),
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_translation(starting_pos),
            sprite: Sprite {
                custom_size: Some(Vec2::new(62.0,32.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::KinematicVelocityBased,
        Velocity::default(),
        Collider::cuboid(0., 0.),
        Sensor
    )).with_children(|knife_parent| {
        knife_parent.spawn( (
            Collider::cuboid(21., 8.),
            Sensor,
            Transform::from_translation(Vec3::new(-5.0, 0.0, 0.0)),
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
    mut knife_query: Query<(&mut Knife, &Transform, &Children, Entity)>,
    mut player_query: Query<(&mut Player, &mut Health, Entity), Without<Knife>>,
    mut commands: Commands
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(a, b, _) => {
                for (knife_struct, _, knife_children, knife_entity) in knife_query.iter_mut() {
                    let collider_child = knife_children.get(0).unwrap();
                    if (a == collider_child || b == collider_child) && knife_struct.state == KnifeState::Shooting {
                        let (_, mut player_health, player_entity) = player_query.single_mut();
                        if *b == player_entity || *a == player_entity {
                            player_health.damage(10.0);
                            commands.entity(knife_entity).despawn_recursive();
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_a, _b, _) => {
                //pass
            }
        }
    }
}