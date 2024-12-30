// //! This example showcases a 2D top-down camera with smooth player tracking.
// //!
// //! ## Controls
// //!
// //! | Key Binding          | Action        |
// //! |:---------------------|:--------------|
// //! | `W`                  | Move up       |
// //! | `S`                  | Move down     |
// //! | `A`                  | Move left     |
// //! | `D`                  | Move right    |

// use bevy::{core_pipeline::bloom::Bloom, prelude::*};

// /// Player movement speed factor.
// const PLAYER_SPEED: f32 = 100.;

// /// How quickly should the camera snap to the desired location.
// const CAMERA_DECAY_RATE: f32 = 2.;

// #[derive(Component)]
// struct Player;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
//         .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
//         .add_systems(Update, move_player)
//         .add_systems(Update, update_camera)
//         .add_systems(Update, animate_sprite)
//         .run();
// }

// #[derive(Component)]
// struct AnimationIndices {
//     first: usize,
//     last: usize,
// }

// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);

// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite), With<Player>>,
// ) {
//     for (indices, mut timer, mut sprite) in &mut query {
//         timer.tick(time.delta());

//         if timer.just_finished() {
//             if let Some(atlas) = &mut sprite.texture_atlas {
//                 atlas.index = if atlas.index == indices.last {
//                     indices.first
//                 } else {
//                     atlas.index + 1
//                 };
//             }
//         }
//     }
// }

// fn setup_scene(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     // World where we move the player
//     commands.spawn((
//         Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
//         MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
//     ));

//     let texture = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Idle.png");
//     let layout = TextureAtlasLayout::from_grid(UVec2::new(120, 80), 10, 1, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);
//     // Use only the subset of sprites in the sheet that make up the run animation
//     let animation_indices = AnimationIndices { first: 1, last: 9 };
//     commands.spawn((
//         Player,
//         Sprite::from_atlas_image(
//             texture,
//             TextureAtlas {
//                 layout: texture_atlas_layout,
//                 index: animation_indices.first,
//             },
//         ),
//         Transform::from_scale(Vec3::splat(1.5)),
//         animation_indices,
//         AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
//     ));
// }

// fn setup_instructions(mut commands: Commands) {
//     commands.spawn((
//         Text::new("Move the light with WASD.\nThe camera will smoothly track the light."),
//         Node {
//             position_type: PositionType::Absolute,
//             bottom: Val::Px(12.0),
//             left: Val::Px(12.0),
//             ..default()
//         },
//     ));
// }

// fn setup_camera(mut commands: Commands) {
//     commands.spawn((
//         Camera2d,
//         Camera {
//             hdr: true, // HDR is required for the bloom effect
//             ..default()
//         },
//         Bloom::NATURAL,
//     ));
// }

// /// Update the camera position by tracking the player.
// fn update_camera(
//     mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
//     player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
//     time: Res<Time>,
// ) {
//     let Ok(mut camera) = camera.get_single_mut() else {
//         return;
//     };

//     let Ok(player) = player.get_single() else {
//         return;
//     };

//     let Vec3 { x, y, .. } = player.translation;
//     let direction = Vec3::new(x, y, camera.translation.z);

//     // Applies a smooth effect to camera movement using stable interpolation
//     // between the camera position and the player position on the x and y axes.
//     camera
//         .translation
//         .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
// }

// /// Update the player position with keyboard inputs.
// /// Note that the approach used here is for demonstration purposes only,
// /// as the point of this example is to showcase the camera tracking feature.
// ///
// /// A more robust solution for player movement can be found in `examples/movement/physics_in_fixed_timestep.rs`.
// fn move_player(
//     mut player: Query<&mut Transform, With<Player>>,
//     time: Res<Time>,
//     kb_input: Res<ButtonInput<KeyCode>>,
// ) {
//     let Ok(mut player) = player.get_single_mut() else {
//         return;
//     };

//     let mut direction = Vec2::ZERO;

//     if kb_input.pressed(KeyCode::KeyW) {
//         direction.y += 1.;
//     }

//     if kb_input.pressed(KeyCode::KeyS) {
//         direction.y -= 1.;
//     }

//     if kb_input.pressed(KeyCode::KeyA) {
//         direction.x -= 1.;
//     }

//     if kb_input.pressed(KeyCode::KeyD) {
//         direction.x += 1.;
//     }

//     // Progressively update the player's position over time. Normalize the
//     // direction vector to prevent it from exceeding a magnitude of 1 when
//     // moving diagonally.
//     let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
//     player.translation += move_delta.extend(0.);
// }

use bevy::prelude::*;

pub mod game;

fn main() {
    App::new()
        .add_plugins((game::game::GamePlugin, DefaultPlugins))
        .run();
}
