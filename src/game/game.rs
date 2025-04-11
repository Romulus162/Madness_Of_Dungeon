use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use bevy_rapier2d::prelude::*;


use super::{camera::camera, level::{level, tiles}, player::player};

fn set_rapier_configuration(mut config: Query<&mut RapierConfiguration>) {
    if let Ok(mut config) = config.get_single_mut() {
        config.gravity.y = -500.0;
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            //later I would like to look into what "NoUserData" is
            RapierPhysicsPlugin::<NoUserData>::default(),
            player::PlayerPlugin,
            level::LevelManagementPlugin,
            camera::CameraPlugin,
            tiles::WallPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, set_rapier_configuration);
    }
}
