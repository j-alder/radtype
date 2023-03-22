mod text;

use bevy::{prelude::*, DefaultPlugins};
use text::TextPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TextPlugin)
        .run();
}
