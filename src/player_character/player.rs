use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{flex_load::*, PLAYER_Z};
use crate::player_character::*;

pub struct SquidPlugin;

impl Plugin for SquidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerUIPlugin, CameraTrackingPlugin, BaseMovementPlugin, PlayerAnimationPlugin, InkPlugin));
        app.insert_resource(InputStack::new());
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_squid);
        app.add_systems(Update, track_input.run_if(in_state(AssetLoadState::Ready)));
    }
}

#[derive(Component, Default)] 
#[require(Sprite, PlayerAnimation)]
pub struct Player {
    pub grounded: bool,
    pub has_jump: bool,
    pub health: Health,
}

impl Player {
    pub fn new () -> Self {
        Self {
            grounded: false,
            has_jump: false,
            health: Health::new(100.0),
        }
    }
}


#[derive(Component, Default)]
pub struct Health {
    pub health: f32,
    pub max_health: f32,
}

impl Health {
    pub fn new (max_health: f32) -> Self {
        Self {
            health: max_health,
            max_health,
        }
    }
    pub fn damage (&mut self, damage: f32) {
        self.health -= damage;
        if self.health < 0.0 {
            self.health = 0.0;
        }
    }
    pub fn heal (&mut self, heal: f32) {
        self.health += heal;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
    pub fn is_dead (&self) -> bool {
        self.health <= 0.0
    }
}

fn spawn_squid (
    mut commands: Commands,
    loaded: Res<LoadedAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let squid_map_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 4, None, None)
    );
    
    commands.spawn( 
        Player::default()
    ).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                texture: loaded.get_typed_clone::<Image>("squid_map").unwrap(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)), //just behind squid head
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            TextureAtlas {
                layout: squid_map_layout.clone(),
                index: 2,
            },
        ));
        parent.spawn((
            Name::new("feet"),
            Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)),
            Collider::ball(16.),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
    });
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize { width: 1280, height: 720 },
        PixelViewport
    ));
}

// fn spawn_squid (
//     mut commands: Commands,
//     loaded: Res<LoadedAssets>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     let squid_map_layout = texture_atlas_layouts.add(
//         TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 4, None, None)
//     );
    
//     commands.spawn((
//         SpriteBundle {
//             texture: loaded.get_typed::<Image>("squid_map").unwrap(),
//             transform: Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z)),
//             sprite: Sprite {
//                 custom_size: Some(Vec2::new(64.0, 64.0)),
//                 ..Default::default()
//             },
//             ..Default::default()
//         },
//         TextureAtlas {
//             layout: squid_map_layout.clone(),
//             index: 4,
//         },
//         PlayerAnimation::default(),
//         RigidBody::Dynamic,
//         Collider::capsule_y(3., 20.),
//         ActiveEvents::COLLISION_EVENTS,
//         Velocity::default(),
//         GravityScale(1.0),
//         Friction {
//             coefficient: 0.5,
//             ..default()
//         },
//         LockedAxes::ROTATION_LOCKED,
//         Player::new(),
//     )).with_children(|parent| {
//         parent.spawn((
//             SpriteBundle {
//                 texture: loaded.get_typed_clone::<Image>("squid_map").unwrap(),
//                 transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)), //just behind squid head
//                 sprite: Sprite {
//                     custom_size: Some(Vec2::new(64.0, 64.0)),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             TextureAtlas {
//                 layout: squid_map_layout.clone(),
//                 index: 2,
//             },
//         ));
//         parent.spawn((
//             Name::new("feet"),
//             Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)),
//             Collider::ball(16.),
//             Sensor,
//             ActiveEvents::COLLISION_EVENTS,
//         ));
//     });
//     commands.spawn((
//         Camera2dBundle::default(),
//         PixelZoom::FitSize { width: 1280, height: 720 },
//         PixelViewport
//     ));
// }