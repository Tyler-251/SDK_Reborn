use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{flex_load::*, PLAYER_Z};
use crate::player::*;

pub struct SquidPlugin;

impl Plugin for SquidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerUIPlugin, CameraTrackingPlugin));
        app.insert_resource(InputStack::new());
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_squid);
        app.add_systems(Update, (control_squid, track_input).run_if(in_state(AssetLoadState::Ready)));
    }
}

#[derive(Component)] 
pub struct Player;


#[derive(Component)]
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
    // Squid layout:
    // 0: full sprite 0
    // 1: full sprite 1
    // 2: legs sprite 0
    // 3: legs sprite 1
    // 4: head sprite 0
    // 5: head sprite 1
    // 6: head sprite 2
    // 7: head sprite 3
    let squid_map_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::splat(32), 3, 3, None, None)
    );
    
    commands.spawn((
        SpriteBundle {
            texture: loaded.get_typed::<Image>("squid_map").unwrap(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        TextureAtlas {
            layout: squid_map_layout.clone(),
            index: 4,
        },
        RigidBody::Dynamic,
        Collider::cuboid(25.0, 26.0),
        ActiveEvents::COLLISION_EVENTS,
        Velocity::default(),
        GravityScale(1.0),
        Friction {
            coefficient: 0.5,
            ..default()
        },
        LockedAxes::ROTATION_LOCKED,
        Player,
        Health::new(100.0),
    )).with_children(|parent| {
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
    });
    commands.spawn(Camera2dBundle::default());
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
        velocity.linvel.x = movement_vector.x * speed;
    } else {
        velocity.linvel.x = velocity.linvel.x * 0.05_f32.powf(time.delta_seconds());
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