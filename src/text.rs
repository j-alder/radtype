use bevy::{input::keyboard::KeyboardInput, prelude::*};
use rand::Rng;

#[derive(Component)]
struct ColorText;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_text)
            .add_event::<KeyboardInput>()
            .add_systems((key_press_event, text_color_system));
    }
}

fn startup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("FiraCode-Regular.ttf"),
                    font_size: 300.0,
                    color: rand_color(),
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        },
        ColorText,
    ));
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        text.sections[0].style.color = color_from_seconds(time.elapsed_seconds());
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
                    let s = text.sections[0].value.clone();
                    let c = key_code_to_str(ev.key_code, s.as_str());
                    text.sections[0].value = String::from(c);
                }
            }
            ButtonState::Released => {}
        }
    }
}

fn color_from_seconds(seconds: f32) -> Color {
    Color::Rgba {
        red: (1.25 * seconds).sin() / 2.0 + 0.5,
        green: (0.75 * seconds).sin() / 2.0 + 0.5,
        blue: (0.50 * seconds).sin() / 2.0 + 0.5,
        alpha: 1.0,
    }
}

fn rand_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen(), rng.gen(), rng.gen())
}

pub fn key_code_to_str(k: Option<KeyCode>, f: &str) -> &str {
    match k.unwrap_or(KeyCode::F24) {
        KeyCode::A => "A",
        KeyCode::B => "B",
        KeyCode::C => "C",
        KeyCode::D => "D",
        KeyCode::E => "E",
        KeyCode::F => "F",
        KeyCode::G => "G",
        KeyCode::H => "H",
        KeyCode::I => "I",
        KeyCode::J => "J",
        KeyCode::K => "K",
        KeyCode::L => "L",
        KeyCode::M => "M",
        KeyCode::N => "N",
        KeyCode::O => "O",
        KeyCode::P => "P",
        KeyCode::Q => "Q",
        KeyCode::R => "R",
        KeyCode::S => "S",
        KeyCode::T => "T",
        KeyCode::U => "U",
        KeyCode::V => "V",
        KeyCode::W => "W",
        KeyCode::X => "X",
        KeyCode::Y => "Y",
        KeyCode::Z => "Z",
        _ => f,
    }
}

pub fn rand_key_code(f: KeyCode) -> KeyCode {
    match rand::thread_rng().gen_range(1..26) {
        1 => KeyCode::A,
        2 => KeyCode::B,
        3 => KeyCode::C,
        4 => KeyCode::D,
        5 => KeyCode::E,
        6 => KeyCode::F,
        7 => KeyCode::G,
        8 => KeyCode::H,
        9 => KeyCode::I,
        10 => KeyCode::J,
        11 => KeyCode::K,
        12 => KeyCode::L,
        13 => KeyCode::M,
        14 => KeyCode::N,
        15 => KeyCode::O,
        16 => KeyCode::P,
        17 => KeyCode::Q,
        18 => KeyCode::R,
        19 => KeyCode::S,
        20 => KeyCode::T,
        21 => KeyCode::U,
        22 => KeyCode::V,
        23 => KeyCode::W,
        24 => KeyCode::X,
        25 => KeyCode::Y,
        26 => KeyCode::Z,
        _ => f,
    }
}
