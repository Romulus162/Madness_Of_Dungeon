use bevy::{prelude::*, render::view::RenderLayers};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;
// pub mod animation;

use super::animation::*;

// this works for now, but i'm really annoyed with this file structure, at some point i need to refactor this.
// use crate::game::camera::camera::PLAYER_RENDER_LAYER;

// use crate::sound_effects::{SoundEffectEvent, SoundEffectType};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationInfo::default())
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

#[derive(Default, Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct PlayerStatus {
    coyote_frames: Timer,
    pub dead: bool,
    pub exiting: bool,
}

#[derive(Component, Debug, PartialEq, Eq)]
pub enum PlayerState{
    Idle,
    MovingRight,
    MovingLeft,
    Jumping,
    Falling,
    Sliding,
}

#[derive(Component, Debug)]
pub struct PlayerInventory {
    //inventory items
    pub num_keys: usize,
}

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    //currently unsure which spritesheet to insert as I need to understand better what and how this playerbundle works, leaving empty for now
    // FURTHERMORE, :).... I need to understand the numbers in the () I am leaving them in for rememberance sake
    // #[sprite_sheet("Knight/Colour1/Outline/120x80_PNGSheets/_Idle.png", 16, 16, 11, 2, 1, 0, 0)]
    sprite_sheet: Sprite,
    //maybe unsure
    // render_layer: RenderLayers,
    player_marker: PlayerMarker,
    //potentially unsure
    player_status: PlayerStatus,
    player_inventory: PlayerInventory,
    rigid_body: RigidBody,
    collider: Collider,
    //unsure
    mass: AdditionalMassProperties,
    velocity: Velocity,
    friction: Friction,
    //unsure
    restitution: Restitution,
    //likely unsure
    locked_axes: LockedAxes,
    animation_timer: AnimationTimer,
    player_state: PlayerState,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        // the bellow jump_cooldown_timer, i'm unsure how benefitial this may be right now so it will likely just be un-used till i understand it.
        let mut jump_cooldown_timer = Timer::new(Duration::from_millis(200), TimerMode::Once);
        jump_cooldown_timer.tick(Duration::from_millis(200));
        Self {
            sprite_sheet:
            //Ldtk 0.10.0 (no longer used in 0.11.0)
            // LdtkSpriteSheetBundle::default(),
            Sprite::default(),
            // render_layer: PLAYER_RENDER_LAYER,
            player_marker: PlayerMarker,
            player_status: PlayerStatus{
                coyote_frames: Timer::new(Duration::from_millis(100), TimerMode::Once),
                dead: false,
                exiting: false,
            },
            player_inventory: PlayerInventory{
                num_keys: 0
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::round_cuboid(6., 3., 2.),
            mass: AdditionalMassProperties::Mass(50.),
            velocity: Velocity::default(),
            friction: Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
            restitution: Restitution {
                coefficient:  0.,
                combine_rule: CoefficientCombineRule::Min,
            },
            locked_axes: LockedAxes::ROTATION_LOCKED,
            player_state: PlayerState::Idle,
            animation_timer: AnimationTimer(Timer::new(
                Duration::from_millis(100),
                TimerMode::Repeating))
        }
    }
}

pub fn move_player(
    mut query_player: Query<
        (
            &mut Velocity,
            &mut Sprite,
            &mut PlayerInventory,
            &mut PlayerStatus,
            &mut PlayerState,
        ),
        With<PlayerMarker>,
    >,
    // camera_planning_state: Res<CameraPanning>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    // mut sound_effect_event_writer: EventWriter<SoundEffectEvent>,
) {
    if let Ok((
        mut player_velocity,
        mut sprite,
        mut player_inventory,
        mut player_status,
        mut player_state
    )) = query_player.get_single_mut()
    {
        const VELOCITY: Vec2 = Vec2::new(55., 0.);

        if keys.pressed(KeyCode::KeyD){
            player_velocity.linvel += VELOCITY;

            if *player_state == PlayerState::MovingLeft || *player_state == PlayerState::Idle {
                *player_state = PlayerState::MovingRight;
            }
        }

        if keys.pressed(KeyCode::KeyA){
            player_velocity.linvel -= VELOCITY;
            if *player_state == PlayerState::MovingRight || *player_state == PlayerState::Idle {
                *player_state = PlayerState::MovingLeft;
            }
        }

        if keys.just_pressed(KeyCode::KeyW) {
            let mut can_jump = false;
            if *player_state != PlayerState::Jumping
                && *player_state != PlayerState::Falling
                && *player_state != PlayerState::Sliding {
                    can_jump = true;
                } else if !player_status.coyote_frames.finished() {
                    can_jump = true;
                }
        }
    }
}
