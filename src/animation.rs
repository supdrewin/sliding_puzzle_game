use bevy::prelude::*;

pub struct Animation;

impl Animation {
    // setup an animation for each direction
    fn setup(mut commands: Commands) {
        // colorful rectangles
        [
            (Direction::BottomLeft, Color::BLUE),
            (Direction::BottomRight, Color::YELLOW),
            (Direction::TopLeft, Color::RED),
            (Direction::TopRight, Color::GREEN),
        ]
        .into_iter()
        .for_each(|(direction, color)| {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(direction);
        });
    }

    fn listen(
        windows: Res<Windows>,
        mut reader: EventReader<AnimationEvent>,
        mut query: Query<(&Direction, &mut Transform), With<Direction>>,
        // should we play an animation now
        mut local: Local<ShouldPlay>,
    ) {
        if let Some(window) = windows.get_primary() {
            reader.iter().for_each(|_| {
                let (x, y) = (
                    (window.physical_width() >> 1) as f32,
                    (window.physical_height() >> 1) as f32,
                );
                query.for_each_mut(|(direction, mut transform)| {
                    transform.scale = Vec3::new(x, y, 1.0);
                    transform.translation = match direction {
                        Direction::BottomLeft => Vec3::new(0.0, 0.0, 10.0),
                        Direction::BottomRight => Vec3::new(x, 0.0, 10.0),
                        Direction::TopLeft => Vec3::new(0.0, y, 10.0),
                        Direction::TopRight => Vec3::new(x, y, 10.0),
                    };
                });
                local.should_play = true;
            });
            if local.should_play {
                let (step_x, step_y) = (
                    (window.physical_width() >> 6) as f32,
                    (window.physical_height() >> 6) as f32,
                ); // 64 frames
                query.for_each_mut(|(direction, mut transform)| {
                    match direction {
                        Direction::BottomLeft => {
                            transform.translation.x -= step_x;
                            transform.translation.y -= step_y;
                        }
                        Direction::BottomRight => {
                            transform.translation.x += step_x;
                            transform.translation.y -= step_y;
                        }
                        Direction::TopLeft => {
                            transform.translation.x -= step_x;
                            transform.translation.y += step_y;
                        }
                        Direction::TopRight => {
                            transform.translation.x += step_x;
                            transform.translation.y += step_y;
                            if transform.translation.x > window.physical_width() as f32
                                && transform.translation.y > window.physical_height() as f32
                            {
                                local.should_play = false;
                            }
                        }
                    };
                });
                if !local.should_play {
                    query.for_each_mut(|(_, mut transform)| {
                        transform.translation.z = 1000.0; // disapear
                    });
                }
            }
        }
    }
}

impl Plugin for Animation {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvent>()
            .add_startup_system(Self::setup)
            .add_system(Self::listen);
    }
}

pub struct AnimationEvent;

#[derive(Component)]
enum Direction {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Default)]
struct ShouldPlay {
    should_play: bool,
}
