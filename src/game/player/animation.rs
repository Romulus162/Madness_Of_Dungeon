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
    pub attack: AnimationClip,
    pub crouch: AnimationClip,
    pub crouch_walk: AnimationClip,
    pub jump: AnimationClip,
    pub jump_fall_between: AnimationClip,
    pub fall: AnimationClip,
    pub turn_around: AnimationClip,
}

use bevy::ecs::world::FromWorld;

impl FromWorld for AnimationInfo {
    fn from_world(world: &mut World) -> Self {

        let idle_image;
        let run_image;
        let attack_image;
        let crouch_image;
        let crouch_walk_image;
        let jump_image;
        let jump_fall_between_image;
        let fall_image;
        let turn_around_image;

        {
            let asset_server = world.resource::<AssetServer>();

            idle_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Idle.png");
            run_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Run.png");
            attack_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Attack.png");
            crouch_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Crouch.png");
            crouch_walk_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_CrouchWalk.png");
            jump_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Jump.png");
            jump_fall_between_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_JumpFallInbetween.png");
            fall_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Fall.png");
            turn_around_image = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_TurnAround.png");

        }

        let mut atlases: Mut<'_, Assets<TextureAtlasLayout>> = world.resource_mut::<Assets<TextureAtlasLayout>>();

        let idle_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 10, 1, None, None));
        let run_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));
        let attack_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));
        let crouch_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));
        let crouch_walk_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));
        let jump_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));
        let jump_fall_between_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));
        let fall_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));
        let turn_around_layout = atlases.add(TextureAtlasLayout::from_grid(UVec2::new(120, 80), 6, 1, None, None));

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
            attack: AnimationClip {
                image: attack_image,
                layout: attack_layout,
                start: 0,
                end: 3,
                duration: vec![80; 4],
            },
            crouch: AnimationClip {
                image: crouch_image,
                layout: crouch_layout,
                start: 0,
                end: 0,
                duration: vec![80; 1],
            },
            crouch_walk: AnimationClip {
                image: crouch_walk_image,
                layout: crouch_walk_layout,
                start: 0,
                end: 7,
                duration: vec![80; 8],
            },
            jump: AnimationClip {
                image: jump_image,
                layout: jump_layout,
                start: 0,
                end: 2,
                duration: vec![80; 3],
            },
            jump_fall_between: AnimationClip {
                image: jump_fall_between_image,
                layout: jump_fall_between_layout,
                start: 0,
                end: 1,
                duration: vec![80; 2],
            },
            fall: AnimationClip {
                image: fall_image,
                layout: fall_layout,
                start: 0,
                end: 2,
                duration: vec![80; 3],
            },
            turn_around: AnimationClip {
                image: turn_around_image,
                layout: turn_around_layout,
                start: 0,
                end: 2,
                duration: vec![80; 3],
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
    mut last_state: Local<Option<PlayerState>>,
) {
    if let Ok((mut sprite, mut timer, parent)) = sprite_query.get_single_mut() {

        if let Ok(player_state) = state_query.get(parent.get()) {
        println!("Using clip: {:?}", player_state);

            // println!("Sprite sees state: {:?}", player_state);
            let clip = match *player_state {
                PlayerState::MovingLeft | PlayerState::MovingRight => &animation_info.run,
                PlayerState::Idle => &animation_info.idle,
                _ => &animation_info.idle,
            };

            if Some(*player_state) != *last_state {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = clip.start;
                }
                timer.set_duration(Duration::from_millis(clip.duration[0]));
                timer.reset();
                *last_state = Some(*player_state);
            }

            timer.tick(time.delta());

            if timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index < clip.start || atlas.index > clip.end {
                        println!(
                            "Animating: state={:?}, index={}, start={}, end={}",
                            player_state, atlas.index, clip.start, clip.end
                        );

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
            sprite.flip_x = matches!(*player_state, PlayerState::MovingLeft);
        }
        // println!(
        //     "Timer finished: {}, current frame: {}",
        //     timer.just_finished(),
        //     sprite.texture_atlas.as_ref().map(|a| a.index).unwrap_or(999)
        // );

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
