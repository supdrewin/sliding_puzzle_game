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
        mut counter: Local<Counter>, // how many frames remaining
    ) {
        if windows.is_changed() {
            if let Some(window) = windows.get_primary() {
                let size = (window.width(), window.height());
                counter.bottom = (size.0 / 4.0, size.1 / 4.0);
                counter.top = (size.0 - counter.bottom.0, size.1 - counter.bottom.1);
                counter.scale = (
                    counter.top.0 - counter.bottom.0,
                    counter.top.1 - counter.bottom.1,
                );
                counter.step = (
                    counter.scale.0 / counter.frames.1 as f32,
                    counter.scale.1 / counter.frames.1 as f32,
                );
            }
        }
        reader.iter().for_each(|_| {
            query.for_each_mut(|(direction, mut transform)| {
                transform.scale = Vec3::new(counter.scale.0, counter.scale.1, 1.0);
                transform.translation = match direction {
                    Direction::BottomLeft => Vec3::new(counter.bottom.0, counter.bottom.1, 10.0),
                    Direction::BottomRight => Vec3::new(counter.top.0, counter.bottom.1, 10.0),
                    Direction::TopLeft => Vec3::new(counter.bottom.0, counter.top.1, 10.0),
                    Direction::TopRight => Vec3::new(counter.top.0, counter.top.1, 10.0),
                };
            });
            counter.frames.0 = counter.frames.1;
        });
        if counter.frames.0 > 0 {
            query.for_each_mut(|(direction, mut transform)| {
                match direction {
                    Direction::BottomLeft => {
                        transform.translation.x -= counter.step.0;
                        transform.translation.y -= counter.step.1;
                    }
                    Direction::BottomRight => {
                        transform.translation.x += counter.step.0;
                        transform.translation.y -= counter.step.1;
                    }
                    Direction::TopLeft => {
                        transform.translation.x -= counter.step.0;
                        transform.translation.y += counter.step.1;
                    }
                    Direction::TopRight => {
                        transform.translation.x += counter.step.0;
                        transform.translation.y += counter.step.1;
                    }
                };
            });
            counter.frames.0 -= 1;
            if counter.frames.0 == 0 {
                query.for_each_mut(|(_, mut transform)| {
                    transform.translation.z = 1000.0; // disapear
                });
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

struct Counter {
    bottom: (f32, f32),
    top: (f32, f32),
    scale: (f32, f32),
    step: (f32, f32),
    frames: (usize, usize),
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            bottom: Default::default(),
            top: Default::default(),
            scale: Default::default(),
            step: Default::default(),
            // remaining and totals
            frames: (0, 60),
        }
    }
}
