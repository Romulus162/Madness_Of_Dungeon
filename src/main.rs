use bevy::prelude::*;

pub mod game;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), game::game::GamePlugin))
        // .add_systems(Startup, setup_debug)
        .run();
}
// use bevy::prelude::*;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
//         .add_systems(Startup, setup)
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
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
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

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     let texture = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Idle.png");
//     let layout = TextureAtlasLayout::from_grid(UVec2::new(120,80), 10, 1, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);
//     // Use only the subset of sprites in the sheet that make up the run animation
//     let animation_indices = AnimationIndices { first: 0, last: 9 };
//     commands.spawn(Camera2d);
//     commands.spawn((
//         Sprite::from_atlas_image(
//             texture,
//             TextureAtlas {
//                 layout: texture_atlas_layout,
//                 index: animation_indices.first,
//             },
//         ),
//         Transform::from_scale(Vec3::splat(6.0)),
//         animation_indices,
//         AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
//     ));
// }
