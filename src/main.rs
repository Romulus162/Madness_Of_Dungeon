use bevy::prelude::*;

pub mod game;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::game::GamePlugin))
        // .add_systems(Startup, setup_debug)
        .run();
}

// fn setup_debug(mut commands: Commands) {
//     commands.spawn(Sprite {
//             color: Color::srgb(0.0, 0.8, 0.8),
//             custom_size: Some(Vec2::new(200.0, 200.0)),
//             ..Default::default()
//     });
// }
// use bevy::prelude::*;
// use bevy_ecs_ldtk::prelude::*;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
//         .add_plugins(LdtkPlugin)
//         .add_systems(Startup, setup)
//         .insert_resource(LevelSelection::index(0))
//         .run();
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let mut camera = Camera2d::default();
//     // camera.projection.scale = 0.5;
//     // camera.transform.translation.x += 1280.0 / 4.0;
//     // camera.transform.translation.y += 720.0 / 4.0;
//     commands.spawn(camera);

//     commands.spawn(LdtkWorldBundle {
//         ldtk_handle: asset_server.load("Typical_2D_platformer_example.ldtk").into(),
//         ..Default::default()
//     });
// }
