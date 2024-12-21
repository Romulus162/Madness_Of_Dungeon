use bevy::prelude::*;
use camera::CameraPlugin;
use movement::MovementPlugin;
use world::WorldPlugin;

mod camera;
mod world;
mod movement;
// mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        // .add_plugins(PlayerPlugin)
        .run();
}
