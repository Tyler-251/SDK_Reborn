use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{TextureDimension, TextureFormat, Extent3d};
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct InkPlugin;

impl Plugin for InkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_splotch);
    }
}

#[derive(Component)]
pub struct Splotch {
    pub size: usize,
    pub timer: Timer,
}

pub fn spawn_splotch (
    commands: &mut Commands,
    size: usize,
    position: Vec2,
    mut images: ResMut<Assets<Image>>,
) {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(rng.gen_range(0..1000));

    let mut texture_data = vec![0u8; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let value = perlin.get([5. * x as f64 / size as f64, 10. * y as f64 / size as f64]);
            let x_weight = 1.0 - (2.0 * (x as f64 / size as f64 - 0.5)).abs(); // linear 0.0 to 1.0 (0.0 at the edges, 1.0 in the middle)
            let y_weight = 1.0 - (2.0 * (y as f64 / size as f64 - 0.5)).abs();
            let weight = x_weight * y_weight;
            let value= (value + (weight * 1.5))* weight;
            let mut pixel_value = ((value * 1000.)).min(255.).max(0.) as u8;

            if pixel_value > 150 { // make toon
                pixel_value = 200;
            } else {
                pixel_value = 0;
            }

            let index = (y * size + x) * 4;
            texture_data[index] = 0; // R
            texture_data[index + 1] = 0; // G
            texture_data[index + 2] = 0; // B
            texture_data[index + 3] = pixel_value // A
        }
    }

    let splotch_image = Image::new_fill(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all()
    );


    commands.spawn((
        Splotch {
            size,
            timer: Timer::from_seconds(1., TimerMode::Repeating)
        },
        SpriteBundle {
            texture: images.add(splotch_image),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size as f32, size as f32)),
                ..Default::default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        }
    ));
}

fn tick_splotch (
    mut commands: Commands,
    mut splotch_query: Query<(&mut Splotch, &Transform, &mut Sprite, Entity)>,
    time: Res<Time>,
) {
    for (mut splotch, _transform, mut sprite, entity) in splotch_query.iter_mut() {
        splotch.timer.tick(time.delta());
        if splotch.timer.finished() {
            splotch.size = (splotch.size as f32 * 0.75) as usize;
            sprite.custom_size = Some(Vec2::splat(splotch.size as f32));
            splotch.timer.reset();
        }
        if splotch.size < 5 {
            commands.entity(entity).despawn();
        }
        
    }
}