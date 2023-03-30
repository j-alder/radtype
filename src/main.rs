mod text;
mod gabe;

use bevy::{prelude::*, DefaultPlugins};
// use gabe::GabePlugin;
use text::TextPlugin;

/** create a 2d camera */
fn startup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_startup_system(startup_camera)
        .add_plugins(DefaultPlugins)
        // .add_plugin(GabePlugin)
        .add_plugin(TextPlugin)
        .run();
}
