use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};

#[derive(Component)]
struct AnimateIdle;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, PartialEq)]
enum Direction {
    Left,
    Right
}

#[derive(Component, PartialEq)]
enum Velocity {
    Running,
    Stopped
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub struct GabePlugin;

impl Plugin for GabePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(sprite_run)
            .add_system(key_press_event);
    }
}

fn sprite_run(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut Transform,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Direction,
        &mut Velocity
    )>
) {
    for (mut indices, mut transform, mut timer, mut sprite, mut direction, mut velocity) in &mut query {
        if *velocity == Velocity::Running {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
            match *direction {
                Direction::Left => transform.translation.x -= 300. * time.delta_seconds(),
                Direction::Right => transform.translation.x += 300. * time.delta_seconds(),
            }
            if transform.translation.x > 500. {
                *velocity = Velocity::Stopped;
                *direction = Direction::Left;
                *indices = AnimationIndices { first: 0, last: 0 };
                sprite.index = indices.first;
                sprite.flip_x = true;
            }
            if transform.translation.x < -500. {
                *velocity = Velocity::Stopped;
                *direction = Direction::Right;
                *indices = AnimationIndices { first: 0, last: 0 };
                sprite.index = indices.first;
                sprite.flip_x = false;
            }
        }
    }
}

fn key_press_event(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &mut AnimationIndices)>
) {
    for ev in events.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if ev.key_code == Some(KeyCode::A) {
                    for (mut velocity, mut indices) in &mut query {
                        if *velocity != Velocity::Running {
                            *indices = AnimationIndices { first: 1, last: 6 };
                            *velocity = Velocity::Running;
                        }
                    }
                }
            }
            ButtonState::Released => {}
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gabe/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 0 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform { 
                translation: Vec3 { x: -500., y: -250., z: 0. }, 
                scale: Vec3::splat(6.0),
                ..default()
            },
            ..default()
        },
        animation_indices,
        Direction::Right,
        Velocity::Stopped,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
