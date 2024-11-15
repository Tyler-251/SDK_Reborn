use bevy::prelude::*;
use super::squid::*;
use crate::flex_load::*;
use super::input_track::*;

pub struct PlayerUIPlugin;

impl Plugin for PlayerUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), setup);
        app.add_systems(Update, (update_input_stack, update_health_bar).run_if(in_state(AssetLoadState::Ready)));
    }
}

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct InputStackText;

fn setup (
    mut commands: Commands,
) {
    commands.spawn((
        TextBundle::from_section("Health: X/X", 
            TextStyle {
                color: Color::WHITE,
                ..default()
            }
        ).with_style(
            Style {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                ..Default::default()
            }
        ),
        HealthBar,
    ));
    commands.spawn((
        TextBundle::from_section("Input Stack: X", 
            TextStyle {
                color: Color::WHITE,
                ..default()
            }
        ).with_style(
            Style {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(40.0),
                ..Default::default()
            }
        ),
        InputStackText,
    ));
}

fn update_input_stack (
    mut query: Query<&mut Text, With<InputStackText>>,
    input_stack: Res<InputStack>,
) { 
    let mut input_stack_clone = input_stack.stack.clone();
    let mut output: String = "".to_string();
    for _ in 0..5 {
        if input_stack_clone.len() > 0 {
            output = format!("{} {}", enum_to_arrow(input_stack_clone.pop().unwrap().0), output);
        } else {
            output = format!("  {}", output);
        }
    }   
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Input Stack: {}", output);
    }
}

fn update_health_bar (
    mut query: Query<&mut Text, With<HealthBar>>,
    player_query: Query<&Player>,
) {
    for mut text in query.iter_mut() {
        let player = player_query.single();
        text.sections[0].value = format!("Health: {}/{}", player.health.health, player.health.max_health);
    }
}

fn enum_to_arrow (direction: InputDirection) -> String {
    match direction {
        InputDirection::Up => "^".to_string(),
        InputDirection::Down => "v".to_string(),
        InputDirection::Left => "<".to_string(),
        InputDirection::Right => ">".to_string(),
    }
}