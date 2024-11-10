use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::flex_load::*;
use super::ui::PlayerUIPlugin;

pub struct SquidPlugin;

impl Plugin for SquidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerUIPlugin);
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
        gravity.0 = 2.5;
    } else {
        gravity.0 = 1.3;
    } 

}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InputDirection {
    Left,
    Right,
    Up, 
    Down,
}

#[derive(Resource)]
pub struct InputStack {
    pub stack: Vec<InputDirection>,
}

impl InputStack {
    pub fn new () -> Self {
        Self {
            stack: Vec::new(),
        }
    }
    pub fn push (&mut self, direction: InputDirection) {
        self.stack.push(direction);
    }
    pub fn pop (&mut self) -> Option<InputDirection> {
        self.stack.pop()
    }
    pub fn clear (&mut self) {
        self.stack.clear();
    }
}

fn track_input (
    input: Res<ButtonInput<KeyCode>>,
    mut input_stack: ResMut<InputStack>,
) {
    if input.just_pressed(KeyCode::KeyA) {
        input_stack.push(InputDirection::Left);
    }
    if input.just_pressed(KeyCode::KeyD) {
        input_stack.push(InputDirection::Right);
    }
    if input.just_pressed(KeyCode::KeyW) {
        input_stack.push(InputDirection::Up);
    }
    if input.just_pressed(KeyCode::KeyS) {
        input_stack.push(InputDirection::Down);
    }
}