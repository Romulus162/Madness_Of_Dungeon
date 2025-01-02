use bevy::prelude::*;

pub mod game;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::game::GamePlugin))
        // .add_systems(Startup, setup_debug)
        .run();
}
