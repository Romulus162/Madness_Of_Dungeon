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
            .insert_resource(TargetLevel(0))
            .insert_resource(LastAccessibleLevel(0))
            // .add_systems(Startup, spawn_ldtk_world)
            .add_systems(OnEnter(LevelLoadingState::Loading), load_level);
    }
}

pub const LEVEL_IIDS: [&str; 1] = [
    // might not be my correct iid anymore, but whatever works for now, and I will have to revist this in the future anyways \_0_/
    "a315ac10-66b0-11ec-9cd7-99f223ad6ade",
];

//deprecated function, load_level takes over for now
fn spawn_ldtk_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    target_level: Res<TargetLevel>,
    existing_world_query: Query<Entity, With<LdtkProjectHandle>>,
) {

    if existing_world_query.iter().next().is_some() {
        info!("LDtk world already exists, skipping spawn.");
        return;
    }


    let handle = asset_server.load("Dungeon.ldtk");
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
