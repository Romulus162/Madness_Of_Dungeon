use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
struct InterLevelTimer(Timer);

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_systems(Startup, spawn_ldtk_world)
            .add_systems(Startup, load_level);
    }
}

pub const LEVEL_IIDS: [&str; 3] = [
    "Level_01",
    "Level_02",
    "Level_03",
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
fn spawn_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("Typical_2D_platformer_example.ldtk");
    println!("LDtk Handle: {:?}", handle); // Debug print the handle
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: handle.into(),
        ..Default::default()
    });
}


fn load_level(
    mut commands: Commands,
    mut query_level_set: Query<&mut LevelSet>,
) {
    commands.spawn(InterLevelTimer(Timer::from_seconds(0.7, TimerMode::Once)));
    // if let Ok(mut level_set) = query_level_set.get_single_mut() {
    //     *level_set = LevelSet::from_iids([LEVEL_IIDS[target_level.0]]);
    //     if last_accessible_level.0 < target_level.0 {
    //         last_accessible_level.0 = target_level.0;
    //     }
    // }
}
