use bevy::{log::tracing_subscriber::fmt::time, prelude::*};
use bevy_rapier2d::prelude::*;
use crate::flex_load::*;

pub struct KnifePlugin;

impl Plugin for KnifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), init_test);
        app.add_systems(Update, tick_knife_holders);
    }
}

pub fn init_test (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
) {
    spawn_knife_holder(commands, loaded, Vec2::ZERO, KnifeHolder::new_debug(2.0));
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
    mut knife_query: Query<(&mut Velocity, &mut Knife, &Transform, Entity)>,
    mut text_query: Query<&mut Text, With<KnifeDebugText>>,
) {
    for (mut knife_holder, children, entity) in query.iter_mut() {
        knife_holder.timer.tick(time.delta());
        if knife_holder.timer.finished() {
            knife_holder.index = (knife_holder.index + 1) % 5;
            
            for child in children.iter() {
                if let Ok((mut knife_velocity, knife_struct, knife_transform, _)) = knife_query.get_mut(*child) {
                    if knife_struct.index == knife_holder.index {
                        knife_velocity.linvel = Vec2::new(-300.0, 0.0);
                        let old_position = knife_transform.translation;
                        commands.entity(entity).with_children(|parent| {
                            spawn_knife(parent, loaded.get_typed("knife").unwrap(), old_position, knife_struct.index);
                        });
                    }
                }
                if let Ok(mut text) = text_query.get_mut(*child) {
                    text.sections[0].value = format!("{}", knife_holder.index);
                }
            }
        }
    }
    for (_, mut knife, _, knife_entity) in knife_query.iter_mut() {
        if knife.ttl <= 0.0 {
            commands.entity(knife_entity).despawn();
        } else {
            knife.ttl -= time.delta_seconds();
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
        spawn_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-18.0, 10.0, 0.1), 4);
        spawn_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-18.0, -10.0, 0.1), 3);

        //front knives
        spawn_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-10.0, 13.0, 0.3), 2);
        spawn_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-10.0, -1.0, 0.3), 1);
        spawn_knife(parent, loaded.get_typed("knife").unwrap(), Vec3::new(-10.0, -13.0, 0.3), 0);

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

#[derive(Component)]
pub struct Knife {
    pub index: i8,
    pub ttl: f32
}

fn spawn_knife (
    child_builder: &mut ChildBuilder,
    sprite: Handle<Image>,
    starting_pos: Vec3,
    index: i8,
) {
    child_builder.spawn( (
        Knife{index, ttl: 20.0},
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
        Collider::cuboid(31., 10.),
        Sensor,
    ));
}
