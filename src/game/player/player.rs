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
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Startup, setup_animations)
            .add_systems(Update,move_player)
            .add_systems(PostUpdate, execute_animations)
            .add_systems(PostUpdate, name_ldtk_players);
    }
}

#[derive(Default, Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct PlayerStatus {
    jump_cooldown: Timer,
    coyote_frames: Timer,
    jump_buffer: Timer,
    pub level_finished: bool,
    pub dead: bool,
    pub exiting: bool,
}

#[derive(Component, Hash, Debug, PartialEq, Eq)]
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
pub struct PlayerBundle {
    //currently unsure which spritesheet to insert as I need to understand better what and how this playerbundle works, leaving empty for now
    // FURTHERMORE, :).... I need to understand the numbers in the () I am leaving them in for rememberance sake
    #[sprite_sheet("Knight/Colour1/Outline/120x80_PNGSheets/_Idle.png", 120, 80, 10, 1, 0, 0, 0)]
    sprite_sheet: Sprite,
    animation_config: AnimationConfig,

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
    // animation_timer: AnimationTimer,
    player_state: PlayerState,
    // new stuff
    // texture: Handle<Image>,
}

impl Default for PlayerBundle {
    //chainging from fn default to fn new
    fn default() -> Self {
        // the bellow jump_cooldown_timer, i'm unsure how benefitial this may be right now so it will likely just be un-used till i understand it.
        let mut jump_cooldown_timer = Timer::new(Duration::from_millis(200), TimerMode::Once);
        jump_cooldown_timer.tick(Duration::from_millis(200));
        Self {
            sprite_sheet: Sprite {
                image: Handle::Weak(default()),
                texture_atlas: None,
                ..default()
            },
            animation_config: AnimationConfig::new(0, 9, 20),
            // render_layer: PLAYER_RENDER_LAYER,
            player_marker: PlayerMarker,
            player_status: PlayerStatus{
                jump_cooldown: jump_cooldown_timer,
                coyote_frames: Timer::new(Duration::from_millis(100), TimerMode::Once),
                jump_buffer: {
                    let mut timer = Timer::new(Duration::from_millis(100), TimerMode::Once);
                    timer.tick(Duration::from_millis(100));
                    timer
                },
                level_finished: false,
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
            // animation_timer: AnimationTimer(Timer::new(
            //     Duration::from_millis(100),
            //     TimerMode::Repeating))
        }
    }
}

fn name_ldtk_players(
    mut commands: Commands,
    query: Query<Entity, (Added<PlayerMarker>, Without<Name>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Name::new("Player"));
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
            // &mut AnimationConfig, //addition
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
        mut player_state,
        // mut animation_config, //addition
    )) = query_player.get_single_mut()
    {

        // println!("Current PlayerState: {:?}", *player_state);

        if !player_status.jump_cooldown.finished() {
            player_status.jump_cooldown.tick(time.delta());
        }

        //addition
        // animation_config.frame_timer.tick(time.delta());

        const VELOCITY: Vec2 = Vec2::new(55., 0.);
        const JUMP_VELOCITY: f32 = 130.;

        let mut moved = false;

        if player_status.dead || player_status.level_finished
        {
            return;
        }

        if keys.pressed(KeyCode::KeyD){
            player_velocity.linvel += VELOCITY;
            if *player_state == PlayerState::MovingLeft || *player_state == PlayerState::Idle {
                *player_state = PlayerState::MovingRight;
            }
            moved = true;
        }
        if keys.pressed(KeyCode::KeyA){
            player_velocity.linvel -= VELOCITY;
            if *player_state == PlayerState::MovingRight || *player_state == PlayerState::Idle {
                *player_state = PlayerState::MovingLeft;
            }
            moved = true;
        }
        if !moved{
            if *player_state == PlayerState::MovingLeft || *player_state == PlayerState::MovingRight
            {
                *player_state = PlayerState::Idle;
            }
        }
        if keys.just_pressed(KeyCode::KeyW)
            || !player_status.jump_buffer.finished()
            && player_status.jump_cooldown.finished()
            {

                if keys.just_pressed(KeyCode::ArrowUp) {
                    player_status.jump_buffer.reset();
                }
                let mut can_jump = false;
                if *player_state != PlayerState::Jumping
                    && *player_state != PlayerState::Falling
                    && *player_state != PlayerState::Sliding
                {
                    // jump from floor
                    can_jump = true;
                } else if !player_status.coyote_frames.finished() {
                    can_jump = true;
                }

                if can_jump {
                    player_velocity.linvel.y += JUMP_VELOCITY;
                    *player_state = PlayerState::Jumping;
                    player_status.jump_cooldown.reset();
                    player_status.coyote_frames.reset();
                }
        }
    }
}
