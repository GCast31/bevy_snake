use bevy::app::{App, Plugin};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Color, Commands, Res, Transform, Windows};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;
use bevy::window::WindowId;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(map_system)
        ;
    }
}

fn map_system(
    window: Res<Windows>,
    mut commands: Commands,
) {
    if let Some(w) = window.get(WindowId::primary()) {
        let (center_x, center_y) = (w.width() / 2., w.height() / 2.);
        for l in 0..25 {
            for c in 0..25 {
                commands.spawn(
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::GRAY,
                            custom_size: Some(Vec2::new(24f32, 24f32)),
                            ..default()
                        },
                        transform: Transform::from_translation(
                            Vec3::new((c * 25) as f32 - center_x + 12.5, -(l * 25) as f32 + center_y - 12.5, 0f32)
                        ),
                        ..default()
                    }
                );
            }
        }
    }
}