use bevy::prelude::*;

pub mod game;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), game::game::GamePlugin))
        // .add_systems(Startup, setup_debug)
        .run();
}
