mod gabe;
mod text;

use bevy::{prelude::*, DefaultPlugins};
use gabe::GabePlugin;
use text::TextPlugin;

/** create a 2d camera */
fn startup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(startup_camera)
        .add_plugin(GabePlugin)
        .add_plugin(TextPlugin)
        .run();
}
