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
    spawn_knife_holder(commands, loaded, Vec2::ZERO);
}

// #region Knife Holder
#[derive(Component)]
pub struct KnifeHolder{
    pub timer: Timer,
    pub index: u8,
}

impl Default for KnifeHolder {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            index: 0,
        }
    }
}

impl KnifeHolder {
    pub fn new (seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
            index: 0,
        }
    }
}

fn tick_knife_holders (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    time: Res<Time>,
    mut query: Query<(&mut KnifeHolder, &Children, Entity)>,
    mut velocity_query: Query<(&mut Velocity, &Knife, &Transform)>,
) {
    for (mut knife_holder, children, entity) in query.iter_mut() {
        knife_holder.timer.tick(time.delta());
        if knife_holder.timer.finished() {
            knife_holder.index = (knife_holder.index + 1) % 5;
            
            for child in children.iter() {
                if let Ok((mut knife_velocity, knife_struct, knife_transform)) = velocity_query.get_mut(*child) {
                    if knife_struct.0 == knife_holder.index {
                        knife_velocity.linvel = Vec2::new(-300.0, 0.0);
                        let old_position = knife_transform.translation;
                        commands.entity(entity).with_children(|parent| {
                            spawn_knife(parent, loaded.get_typed("knife").unwrap(), old_position, knife_struct.0);
                        });
                    }
                }
            }
        }
    }
}
// #endregion

pub fn spawn_knife_holder (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    position: Vec2,
) {
    //knife base
    commands.spawn( (
        KnifeHolder::default(),
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

    });
}

#[derive(Component)]
pub struct Knife(u8);

fn spawn_knife (
    mut child_builder: &mut ChildBuilder,
    sprite: Handle<Image>,
    starting_pos: Vec3,
    index: u8,
) {
    child_builder.spawn( (
        Knife(index),
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
