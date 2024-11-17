use bevy::{prelude::*, render::camera};
use bevy_rapier2d::prelude::*;

pub struct ParallaxPlugin;

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate, 
            manage_parallax_movement
                .after(PhysicsSet::Writeback)
                .before(TransformSystem::TransformPropagate)
        );
    }
}

#[derive(Component)]
pub struct ParallaxLayer {
    /// For every 100 units the camera moves in the x direciton, the layer moves by this amount
    pub speed_x: f32,
    /// For every 100 units the camera moves in the y direciton, the layer moves by this amount
    pub speed_y: f32,
    pub offset: Vec2,
}

fn manage_parallax_movement (
    mut layer_query: Query<(&ParallaxLayer, &mut Transform), Without<Camera>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    if camera_query.iter().count() == 0 {return}
    let camera_transform = camera_query.single();
    for (layer, mut transform) in layer_query.iter_mut() {
        transform.translation.x = camera_transform.translation.x + ((camera_transform.translation.x/100.0) * layer.speed_x) + layer.offset.x;
        transform.translation.y = camera_transform.translation.y + ((camera_transform.translation.y/100.0) * layer.speed_y) + layer.offset.y;
    }
}