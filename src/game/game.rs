use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use super::{camera::camera, level::{level, tiles}, player::player};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            //later I would like to look into what "NoUserData" is
            RapierPhysicsPlugin::<NoUserData>::default(),
            player::PlayerPlugin,
            level::LevelManagementPlugin,
            camera::CameraPlugin,
            tiles::WallPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        ));
    }
}
