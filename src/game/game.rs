use bevy::prelude::*;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use super::player::player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            //later I would like to look into what "NoUserData" is
            RapierPhysicsPlugin::<NoUserData>::default(),
            player::PlayerPlugin
        ));
    }
}