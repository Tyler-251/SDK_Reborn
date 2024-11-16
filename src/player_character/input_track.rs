use std::time::Instant;

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InputDirection {
    Left,
    Right,
    Up, 
    Down,
}

#[derive(Resource)]
pub struct InputStack {
    pub stack: Vec<(InputDirection, Instant)>,
}

impl InputStack {
    pub fn new () -> Self {
        Self {
            stack: Vec::new(),
        }
    }
    pub fn push (&mut self, direction: InputDirection) {
        self.stack.push((direction, Instant::now()));
    }
    pub fn pop (&mut self) -> Option<(InputDirection, Instant)> {
        self.stack.pop()
    }
    pub fn clear (&mut self) {
        self.stack.clear();
    }
}

pub fn track_input (
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