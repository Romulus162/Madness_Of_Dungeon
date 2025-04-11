use bevy::prelude::*;
use std::time::Duration;

// use crate::game::player::{PlayerMarker, PlayerState};
use crate::game::player::player::{PlayerState, PlayerMarker};

#[derive(Component)]
pub struct PlayerSpriteMarker;

pub struct AnimationClip {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
    pub start: usize,
    pub end: usize,
    pub duration: Vec<u64>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct AnimationInfo {
    pub idle: AnimationClip,
    pub run: AnimationClip,
    // pub jump: AnimationClip,
    // pub fall: AnimationClip,
}

use bevy::ecs::world::FromWorld;

impl FromWorld for AnimationInfo {
    fn from_world(world: &mut World) -> Self {
        let idle_image;
        let run_image;
        {
            let asset_server = world.resource::<AssetServer>();
            idle_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Idle.png");
            run_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Run.png");
        }

        let mut atlases: Mut<'_, Assets<TextureAtlasLayout>> = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let idle_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 10, 1, None, None));
        let run_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));

        Self {
            idle: AnimationClip {
                image: idle_image,
                layout: idle_layout,
                start: 0,
                end: 9,
                duration: vec![100; 10],
            },
            run: AnimationClip {
                image: run_image,
                layout: run_layout,
                start: 0,
                end: 5,
                duration: vec![80; 6],
            },
        }
    }
}

//still wondering about if attaching child to parent player is necessary
pub fn animate_player(
    time: Res<Time>,
    animation_info: Res<AnimationInfo>,
    mut sprite_query: Query<
        (&mut Sprite, &mut AnimationTimer, &Parent),
        With<PlayerSpriteMarker>,
    >,
    state_query: Query<&PlayerState, With<PlayerMarker>>,
) {
    if let Ok((mut sprite, mut timer, parent)) = sprite_query.get_single_mut() {
        if let Ok(player_state) = state_query.get(parent.get()) {
            let clip = match *player_state {
                PlayerState::MovingLeft | PlayerState::MovingRight => &animation_info.run,
                PlayerState::Idle => &animation_info.idle,
                _ => &animation_info.idle,
            };

            timer.tick(time.delta());

            if timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index < clip.start || atlas.index > clip.end {
                        clip.start
                    } else if atlas.index == clip.end {
                        clip.start
                    } else {
                        atlas.index + 1
                    };

                    let frame_index = atlas.index - clip.start;
                    let duration = clip.duration.get(frame_index).copied().unwrap_or(100);
                    timer.set_duration(Duration::from_millis(duration));
                }
            }
        }
    }
}

pub fn attach_player_sprite(
    mut commands: Commands,
    animation_info: Res<AnimationInfo>,
    query: Query<Entity, (With<PlayerMarker>, Without<PlayerSpriteMarker>)>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    if let Ok(player_entity) = query.get_single() {
        let idle = &animation_info.idle;

        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                Sprite::from_atlas_image(
                    idle.image.clone(),
                    TextureAtlas {
                        layout: idle.layout.clone(),
                        index: idle.start,
                    },
                ),
                Transform::from_xyz(5.0, 35.0, 5.0),
                PlayerSpriteMarker,
                AnimationTimer(Timer::new(
                    Duration::from_millis(idle.duration[0]),
                    TimerMode::Repeating,
                    ))
            ));
        });

        *has_run = true;
    }
}
