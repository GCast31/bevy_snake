
use bevy::{prelude::*, window::WindowId};


const SNAKE_SIZE_HEIGHT: f32 = 25f32;
const SNAKE_SIZE_WIDTH: f32 = 25f32;

#[derive(Resource)]
struct TimerSnakeMove(Timer);

pub struct SnakePlugin;

#[derive(Component)]
struct Snake;

#[derive(PartialEq, Clone, Copy)]
enum EDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
struct Direction(EDirection);


#[derive(Component)]
struct NextDirection(EDirection);

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(snake_system)
            .insert_resource(TimerSnakeMove(Timer::from_seconds(0.2, TimerMode::Repeating)))
            .add_system(snake_movement)
            .add_system(snake_inputs_movement)
        ;        
    }
}

// Create snake
fn snake_system(
    mut commands: Commands,
    window: Res<Windows>,

) {

    if let Some(w) = window.get(WindowId::primary()) {

        let (center_x, center_y) = (w.width() / 2., w.height() / 2.);

        commands.spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2 { x: SNAKE_SIZE_WIDTH, y: SNAKE_SIZE_HEIGHT }),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-center_x + (SNAKE_SIZE_WIDTH / 2.), center_y - (SNAKE_SIZE_HEIGHT / 2.), 0.)),
                ..Default::default()
            }
        )
        .insert(Snake)
        .insert(Direction(EDirection::Right))
        .insert(NextDirection(EDirection::Right))
        ;

    }
}

// Get Input for changing snake direction
fn snake_inputs_movement(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut NextDirection, &Direction)>
) {
    
    let (mut next, actual) = query.single_mut();

    if kb.pressed(KeyCode::Left) && actual.0 != EDirection::Right {
        next.0 = EDirection::Left;
    }
    if kb.pressed(KeyCode::Right) && actual.0 != EDirection::Left {
        next.0 = EDirection::Right;
    }
    if kb.pressed(KeyCode::Down) && actual.0 != EDirection::Up {
        next.0 = EDirection::Down;
    }
    if kb.pressed(KeyCode::Up) && actual.0 != EDirection::Down {
        next.0 = EDirection::Up;
    }
}

// Move snake every x seconds
fn snake_movement(
    time: Res<Time>,
    mut timer: ResMut<TimerSnakeMove>,
    mut query: Query<(&mut Transform, &mut Direction, &NextDirection), With<Snake>>
) {
    if timer.0.tick(time.delta()).just_finished() {

        for (mut tf, mut actual_direction, next_direction) in query.iter_mut() {

            actual_direction.0 = next_direction.0;

            match &actual_direction.0 {
                EDirection::Down => tf.translation.y -= SNAKE_SIZE_HEIGHT,
                EDirection::Up => tf.translation.y += SNAKE_SIZE_HEIGHT,
                EDirection::Left => tf.translation.x -= SNAKE_SIZE_WIDTH,
                EDirection::Right => tf.translation.x += SNAKE_SIZE_WIDTH,
            }
        }
    }
}