use bevy::log::tracing_subscriber::fmt::time;
use bevy::prelude::*;
use bevy::render::camera;
use crate::player::*;
use crate::flex_load::*;

pub struct CameraTrackingPlugin;

impl Plugin for CameraTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_tracking.run_if(in_state(AssetLoadState::Ready)));
    }
}

pub fn update_camera_tracking (
    mut camera_query: Query<(&mut Transform, &Camera), Without<Player>>,
    player_query: Query<(&Transform, &Player), Without<Camera>>,
    time: Res<Time>,
) {
    for (player_transform, _) in player_query.iter() {
        for (mut camera_transform, _) in camera_query.iter_mut() {
            if player_transform.translation.x > camera_transform.translation.x + 150.0 {
                camera_transform.translation.x = player_transform.translation.x - 150.0;
            }
            if player_transform.translation.x < camera_transform.translation.x - 150.0 {
                camera_transform.translation.x = player_transform.translation.x + 150.0;
            }
            if player_transform.translation.y > camera_transform.translation.y + 100.0 {
                camera_transform.translation.y = player_transform.translation.y - 100.0;
            }
            if player_transform.translation.y < camera_transform.translation.y - 100.0 {
                camera_transform.translation.y = player_transform.translation.y + 100.0;
            }

            let difference: Vec2 = player_transform.translation.xy() - camera_transform.translation.xy();
            camera_transform.translation = camera_transform.translation + (difference.extend(0.0) * 2.0 * time.delta_seconds());
        }
    }
}
