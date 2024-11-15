use bevy::{prelude::*, sprite};
use bevy_rapier2d::prelude::*;
use crate::player::*;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_squid);
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum AnimState {
    #[default] Idle,
    Walk,
    Jump,
    Fall,
    Dash,
}

#[derive(Default, PartialEq, Eq)]
pub enum PlayerFace {
    Left,
    #[default] Right,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub state: AnimState,
    pub face: PlayerFace,
    pub frame: usize,
    timer: Timer,
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self {
            state: AnimState::default(),
            face: PlayerFace::default(),
            frame: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

impl PlayerAnimation {
    /// Will reset timer
    pub fn change_state (&mut self, state: AnimState) {
        self.state = state;
        self.frame = 0;
    }
    pub fn set_state (&mut self, state: AnimState) {
        if self.state == state {return}
        self.change_state(state);
    }
}

// Squid layout:
// 0: full sprite 0
// 1: full sprite 1
// 2: legs sprite 0
// 3: legs sprite 1
// 4: head sprite 0
// 5: head sprite 1
// 6: head sprite 2
// 7: head sprite 3
// 8: angry head sprite 0
// 9: full body dash 0
// 10: full body dash 1
// 11: full body fall 0
// 12: full body fall 1
// 13: head jump
// 14: leg jump

fn animate_squid (
    mut player_query: Query<(&Player, &Velocity, &mut PlayerAnimation, &mut Sprite, &mut TextureAtlas, &Children)>,
    mut sprite_query: Query<(&mut Sprite, &mut TextureAtlas), Without<Player>>,
    time: Res<Time>,
) {
    if player_query.iter().count() == 0 {return}
    let (player_struct, player_velocity, mut player_anim, mut head_sprite, mut head_atlas, player_children) = player_query.single_mut();
    let (mut leg_sprite, mut leg_atlas) = sprite_query.get_mut(player_children[0]).unwrap();
    player_anim.timer.tick(time.delta());
    if player_anim.timer.finished() {
        player_anim.frame += 1;
    }
    
    // recursive face flip
    if player_anim.face == PlayerFace::Left {
        head_sprite.flip_x = true;
        leg_sprite.flip_x = true;

    } else if player_anim.face == PlayerFace::Right {
        head_sprite.flip_x = false;
        leg_sprite.flip_x = false;
    }

    // determine if falling
    if player_anim.state != AnimState::Dash && !player_struct.grounded {
        if player_velocity.linvel.y > 0.0 {
            player_anim.set_state(AnimState::Jump);
        } else if player_velocity.linvel.y < -20.0 {
            player_anim.set_state(AnimState::Fall);
        }
    }

    // animation states
    match player_anim.state {
        AnimState::Idle => {
            head_atlas.index = 4;
            leg_atlas.index = 2;

            if player_anim.frame % 20 == 17 { // every 20 frames blink
                head_atlas.index = 5;
            } else if player_anim.frame % 20 == 18 {
                head_atlas.index = 6;
            } else if player_anim.frame % 20 == 19 {
                head_atlas.index = 7;
            }
        },
        AnimState::Walk => {
            head_atlas.index = 4;

            if player_anim.frame % 2 == 0 {
                leg_atlas.index = 2;
            } else {
                leg_atlas.index = 3;
            }
        },
        AnimState::Dash => {
            head_atlas.index = player_anim.frame % 2 + 9;
            leg_atlas.index = player_anim.frame % 2 + 9;
        },
        AnimState::Fall => {
            head_atlas.index = player_anim.frame % 2 + 11;
            leg_atlas.index = player_anim.frame % 2 + 11;
        },
        AnimState::Jump => {
            head_atlas.index = 13;
            leg_atlas.index = 14;
        },
        _ => {},
    }
}