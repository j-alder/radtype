use rand::Rng;
use bevy::{prelude::*, input::keyboard::KeyboardInput};

#[derive(Resource)]
struct ScaleTimer(Timer);

#[derive(Component)]
struct AnimateScale;

#[derive(Component)]
struct ColorText;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(configure_camera)
            .add_event::<KeyboardInput>()
            .add_systems((
                key_press_event, 
                animate_scale,
                text_color_system
            ));
    }
}

fn configure_camera(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    /* create a 2d camera */
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Text2dBundle { 
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("FiraCode-Regular.ttf"),
                    font_size: 300.0,
                    color: rand_color(),
                }
            ).with_alignment(TextAlignment::Center),
            ..default()
        },
        ColorText
    ));
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    for mut transform in &mut query {
        transform.scale = Vec3::splat(transform.scale.x + time.delta_seconds() * 5.0);
    }
}

fn key_press_event(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<&mut Text, With<ColorText>>,
) {
    use bevy::input::ButtonState;

    for ev in events.iter() {
        match ev.state {
            ButtonState::Pressed => {
                for mut text in &mut query {
                    text.sections[0].value = String::from(
                        key_code_to_str(
                            ev.key_code, 
                            text.sections[0].value.as_str()
                        )
                    );
                }
            }
            ButtonState::Released => {}
        }
    }
}

fn rand_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen(), rng.gen(), rng.gen())
}

fn key_code_to_str(k: Option<KeyCode>, f: &str) -> &str {
    match k.unwrap_or(KeyCode::F24) {
        KeyCode::A=>"A",
        KeyCode::B=>"B",
        KeyCode::C=>"C",
        KeyCode::D=>"D",
        KeyCode::E=>"E",
        KeyCode::F=>"F",
        KeyCode::G=>"G",
        KeyCode::H=>"H",
        KeyCode::I=>"I",
        KeyCode::J=>"J",
        KeyCode::K=>"K",
        KeyCode::L=>"L",
        KeyCode::M=>"M",
        KeyCode::N=>"N",
        KeyCode::O=>"O",
        KeyCode::P=>"P",
        KeyCode::Q=>"Q",
        KeyCode::R=>"R",
        KeyCode::S=>"S",
        KeyCode::T=>"T",
        KeyCode::U=>"U",
        KeyCode::V=>"V",
        KeyCode::W=>"W",
        KeyCode::X=>"X",
        KeyCode::Y=>"Y",
        KeyCode::Z=>"Z",
        _=>f
    }
}
