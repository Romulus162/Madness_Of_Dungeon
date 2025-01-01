use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::game_state::{LevelLoadingState, TargetLevel};

#[derive(Component)]
struct InterLevelTimer(Timer);

#[derive(Resource)]
pub struct LastAccessibleLevel(pub usize);

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LastAccessibleLevel(0))
            .add_systems(Startup, spawn_ldtk_world)
            .add_systems(OnEnter(LevelLoadingState::Loading), load_level);
    }
}

pub const LEVEL_IIDS: [&str; 1] = [
    "a315ac10-66b0-11ec-9cd7-99f223ad6ade",
];

//actual function
// fn spawn_ldtk_world(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     // target_level: Res<TargetLevel>,
// ) {
//     commands.spawn(LdtkWorldBundle {
//         ldtk_handle: asset_server.load("Typical_2D_platformer_example.ldtk").into(),
//         // level_set: LevelSet::from_iids([LEVEL_IIDS[target_level.0]]),
//         ..default()
//     });
// }

//chatgtp testing code ignore
fn spawn_ldtk_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    target_level: Res<TargetLevel>,
) {
    let handle = asset_server.load("Typical_2D_platformer_example.ldtk");
    println!("LDtk Handle: {:?}", handle); // Debug print the handle
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: handle.into(),
        level_set: LevelSet::from_iids([LEVEL_IIDS[target_level.0]]),
        ..Default::default()
    });
}


fn load_level(
    mut commands: Commands,
    target_level: Res<TargetLevel>,
    mut last_accessible_level: ResMut<LastAccessibleLevel>,
    mut query_level_set: Query<&mut LevelSet>,
) {
    // commands.spawn(InterLevelTimer(Timer::from_seconds(0.7, TimerMode::Once)));
    if let Ok(mut level_set) = query_level_set.get_single_mut() {
            *level_set = LevelSet::from_iids([LEVEL_IIDS[target_level.0]]);
            if last_accessible_level.0 < target_level.0 {
                last_accessible_level.0 = target_level.0;
            }
    };
}
