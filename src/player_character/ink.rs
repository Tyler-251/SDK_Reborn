use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{TextureDimension, TextureFormat, Extent3d};
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct InkPlugin;

impl Plugin for InkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SplotchRegistry {
            to_spawn: Vec::new(),
            spawned: Vec::new(),
        });
        app.add_systems(Update, (manage_spawns, tick_splotch).chain());
    }
}

#[derive(Resource)]
pub struct SplotchRegistry {
    pub to_spawn: Vec<Splotch>,
    pub spawned: Vec<(Splotch, Entity)>,
}

#[derive(Component)]
pub struct Splotch {
    pub size: usize,
    pub timer: Timer,
    pub position: Vec2,
    pub image: Handle<Image>,
}

pub fn spawn_splotch (
    splotch_registry: &mut ResMut<SplotchRegistry>,
    size: usize,
    position: Vec2,
) {
    splotch_registry.to_spawn.push(Splotch {
        size,
        timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        position,
        image: Handle::default(),
    });
}

pub fn spawn_splotch_cluster (
    mut splotch_registry: &mut ResMut<SplotchRegistry>,
    size: usize,
    position: Vec2,
) {
    spawn_splotch(&mut splotch_registry, size, position);
    
    let mut rng = rand::thread_rng();
    let count = rng.gen_range(1..=3);
    let spread = size as f32 / 2.0;

    for _ in 0..count {
        let mut x = position.x + rand::thread_rng().gen_range(-spread..spread);
        if x < 0.0 {
            x -= spread/3.0;
        } else {
            x += spread/3.0;
        }
        let mut y = position.y + rand::thread_rng().gen_range(-spread..spread);
        if y < 0.0 {
            y -= spread/3.0;
        } else {
            y += spread/3.0;
        }
        spawn_splotch(&mut splotch_registry, size/(2+count) + 20, Vec2::new(x, y));
    }
}

pub fn manage_spawns (
    mut commands: Commands,
    mut splotch_registry: ResMut<SplotchRegistry>,
    mut images: ResMut<Assets<Image>>,
) {
    for splotch in splotch_registry.to_spawn.iter() {
        let mut rng = rand::thread_rng();
        let perlin = Perlin::new(rng.gen_range(0..10000));
        let size = splotch.size;
        let scaled_size = size / 2;

        let mut texture_data = vec![0u8; scaled_size * scaled_size * 4];
        for y in 0..scaled_size {
            for x in 0..scaled_size {
                let value = perlin.get([5.0 * x as f64 / scaled_size as f64, 10.0 * y as f64 / scaled_size as f64]);
                let x_weight = 1.0 - (2.0 * (x as f64 / scaled_size as f64 - 0.5)).abs(); // linear 0.0 to 1.0 (0.0 at the edges, 1.0 in the middle)
                let y_weight = 1.0 - (2.0 * (y as f64 / scaled_size as f64 - 0.5)).abs();
                let weight = x_weight * y_weight;
                let value = (value + (weight * 1.5)) * weight;
                let mut pixel_value = ((value * 1000.0).min(255.0).max(0.0)) as u8;

                if pixel_value > 150 { // make toon
                    pixel_value = 200;
                } else {
                    pixel_value = 0;
                }

                let index = (y * scaled_size + x) * 4;
                texture_data[index] = 0; // R
                texture_data[index + 1] = 0; // G
                texture_data[index + 2] = 25; // B
                texture_data[index + 3] = pixel_value; // A
            }
        }

        let splotch_image = Image::new_fill(
            Extent3d {
                width: scaled_size as u32,
                height: scaled_size as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &texture_data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all()
        );

        let splotch_image = images.add(splotch_image);

        commands.spawn((
            SpriteBundle {
                texture: splotch_image.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size as f32, size as f32)),
                    ..Default::default()
                },
                transform: Transform::from_translation(splotch.position.extend(0.0)),
                ..Default::default()
            },
            Splotch {
                size,
                timer: Timer::from_seconds(1., TimerMode::Repeating),
                position: splotch.position,
                image: splotch_image,
            },
        ));
    }
    splotch_registry.to_spawn.clear();
}

fn tick_splotch (
    mut commands: Commands,
    mut splotch_query: Query<(&mut Splotch, &Transform, &mut Sprite, Entity)>,
    time: Res<Time>,
    mut images: ResMut<Assets<Image>>,
) {
    for (mut splotch, _transform, mut sprite, entity) in splotch_query.iter_mut() {
        splotch.timer.tick(time.delta());
        if splotch.timer.finished() {
            let new_alpha = sprite.clone().color.alpha() * 0.75;
            sprite.color.set_alpha(new_alpha);
            splotch.timer.reset();
        }
        if sprite.color.alpha() < 0.1 {
            images.remove(splotch.image.id()); // dealloc image
            commands.entity(entity).despawn(); // kill entity

        }
        
    }
}