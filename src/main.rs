mod snake;
mod map;

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, window::WindowResizeConstraints};
use snake::SnakePlugin;
use map::MapPlugin;

const WINDOW_HEIGHT: f32 = 25.*25.;
const WINDOW_WIDTH: f32 = 25.*25.;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                    window: WindowDescriptor {
                        position: WindowPosition::Centered,
                        height: WINDOW_HEIGHT,
                        width: WINDOW_WIDTH,
                        title: String::from("Bevy Snake"),
                        resize_constraints: WindowResizeConstraints {
                            min_height: WINDOW_HEIGHT,
                            max_height: WINDOW_HEIGHT,
                            min_width: WINDOW_WIDTH,
                            max_width: WINDOW_WIDTH,
                            ..Default::default()
                        },
                        resizable: false,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            )
        )
        .add_plugin(SnakePlugin)
        .add_plugin(MapPlugin)
        .add_startup_system(load_system)
        .run();
}

// Load system
fn load_system(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK) ,
                ..Default::default()
            },
            ..Default::default()
        }
    );
}
