// this whole file is no where close to what I will be using I suspect, right now I'm just trying to figure out the flow, which I think I've done, now I need to actually make it work effeciently.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use std::{collections::HashMap, time::Duration};

//again another example of why I need to start refactoring, this looks so redundant
use crate::game::player::player::PlayerState;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_player_animation)
            .add_systems(Update, setup_player_state_animations);
    }
}

#[derive(Resource)]
pub struct AnimationInfo {
    jumping: usize,
    falling: usize,
    sliding: usize,

}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationInfo {
    fn default() -> Self {
        Self{
            jumping: 0,
            falling: 0,
            sliding: 0,
        }
    }
}

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl AnimationConfig {
    pub fn new(
        first: usize,
        last: usize,
        fps: u8,
        texture: Handle<Image>,
        texture_atlas_layout: Handle<TextureAtlasLayout>,
    ) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            texture,
            texture_atlas_layout,
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}

#[derive(Component)]
pub struct StateAnimationMap {
    pub animations: HashMap<PlayerState, AnimationConfig>,
}

impl StateAnimationMap {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    pub fn add_animation(&mut self, state: PlayerState, config: AnimationConfig) {
        self.animations.insert(state, config);
    }
}

fn setup_player_state_animations(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load textures for each animation
    let idle_texture = asset_server.load("idl/spritesheet/path");
    let running_texture: Handle<Image> = asset_server.load("running/spritesheet/path");
    let jumping_texture: Handle<Image> = asset_server.load("jumping/spritesheet/path");
    let falling_texture: Handle<Image> = asset_server.load("falling/spritesheet/path");

    // Define texture atlas layouts (assumes uniform grid spritesheets)
    let idle_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 2, 1, None, None));
    let running_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 2, 1, None, None));
    let jumping_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 2, 1, None, None));
    let falling_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 2, 1, None, None));

    // Create the animation map
    //keep in mind these animation_maps below are very likely bullshit values, right now I'm just trying to figure out the flow
    let mut animation_map = StateAnimationMap::new();
    animation_map.add_animation(
        PlayerState::Idle,
        AnimationConfig::new(0,3,10, idle_texture.clone(), idle_layout.clone()),
    )
}

fn update_player_animation(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationConfig,
        &mut Sprite,
        &PlayerState,
        &StateAnimationMap,
    )>,
) {
    for (mut anim_config, mut sprite, player_state, animation_map) in query.iter_mut() {
        // Get the animation configuration for the current state
        if let Some(state_anim_config) = animation_map.animations.get(player_state) {
            // again, the below is likely bullshit, I am suspicious of all the clone() calls, but for now, I just want to get some framework and maybe get something to work.
            if sprite.image != state_anim_config.texture {
                sprite.image = state_anim_config.texture.clone();
                sprite.texture_atlas = Some(TextureAtlas {
                    layout: state_anim_config.texture_atlas_layout.clone(),
                    index: state_anim_config.first_sprite_index,
                });
                anim_config.frame_timer = state_anim_config.frame_timer.clone();
            }
            // Update the animation frame timer
            anim_config.frame_timer.tick(time.delta());

            if anim_config.frame_timer.just_finished() {
                // Update the sprite atlas index based on the animation state
                if let Some(atlas) = sprite.texture_atlas.as_mut() {
                    let atlas = sprite.texture_atlas.as_mut().unwrap();
                    atlas.index = if atlas.index >= state_anim_config.last_sprite_index {
                        state_anim_config.first_sprite_index
                    } else {
                        atlas.index + 1
                    };
                }
            }
        }
    }
}
