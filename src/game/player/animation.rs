// this whole file is no where close to what I will be using I suspect, right now I'm just trying to figure out the flow, which I think I've done, now I need to actually make it work effeciently.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use std::{collections::HashMap, time::Duration};

//again another example of why I need to start refactoring, this looks so redundant
use crate::game::player::player::PlayerState;

#[derive(Resource)]
pub struct AnimationInfo {
    pub state_animations: HashMap<PlayerState, SpriteSheet>,

}

impl AnimationInfo {
    pub fn new(player_state: PlayerState, player_sprite_sheet: SpriteSheet) ->  Self{

        let mut state_animations = HashMap::new();
        state_animations.insert(player_state, player_sprite_sheet);

        Self{state_animations}
    }
}

#[derive(Component)]
pub struct SpriteSheet {
    pub sprite: Sprite,
}

impl SpriteSheet {
    fn new(image: Handle<Image>, texture_atlas: Option<TextureAtlas>) -> Self{
        Self {
            sprite: Sprite {
                image,
                texture_atlas,
                ..default()
            }
        }
    }
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_frame: usize,
    pub last_frame: usize,
    pub fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_frame: first,
            last_frame: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

// copied from bevy 0.15 spritesheet example, right now this only animates once
//fn animate

// I will mess with this further in the future, right now it will likely be un-used
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_frame {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_frame;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

pub fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    // load the sprite sheet using the `AssetServer`
    let idle_texture = asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Idle.png");

    // the sprite sheet has 10 sprites arranged in a row, and they are all 120px x 80px
    let idle_layout = TextureAtlasLayout::from_grid(UVec2::new(120, 80), 10, 1, None, None);

    // the first idle sprite runs at 20 FPS
    let idle_config = AnimationConfig::new(0, 9, 20);

    // Initialize all the values
    let idle_sprite_sheet = SpriteSheet::new(
        idle_texture,
        Some(TextureAtlas {
            layout: texture_atlas_layouts.add(idle_layout),
            index: idle_config.first_frame,
        })
    );

    commands.insert_resource(AnimationInfo::new(PlayerState::Idle, idle_sprite_sheet));

}
