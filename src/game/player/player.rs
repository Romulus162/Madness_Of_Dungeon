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
            .init_resource::<AnimationInfo>()
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(PostUpdate, (
                attach_player_sprite,
                name_ldtk_players,
                add_controllable_to_player,
            ))
            // .add_systems(Startup, setup_animations)
            .add_systems(Update, animate_player)
            //deprecated, trying new system
            // .add_systems(Update,move_player)
            .add_systems(Update, (
                collect_input,
                apply_movement,
                jump_system,
            ));
            // .add_systems(PostUpdate, execute_animations)
            // .add_systems(Update, debug_player_entities)
    }
}

//Player entity in world inspector is located at Entity (11v1)/Level_0(12v1)/GameEntities(282v1)/Player(283v1)

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

// Ldtk macro here dictates playerbundle cannot have non-static values such as sprite
// only solid shit like physics bitch
#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    //currently unsure which spritesheet to insert as I need to understand better what and how this playerbundle works, leaving empty for now
    // FURTHERMORE, :).... I need to understand the numbers in the () I am leaving them in for rememberance sake
    // I may have to re-add this later
    // sprite_sheet: Sprite,
    // animation_config: AnimationConfig,

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
            // sprite_sheet: Sprite {
            //     image: Handle::Weak(default()),
            //     texture_atlas: None,
            //     ..default()
            // },
            // animation_config: AnimationConfig::new(0, 9, 20),
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
            animation_timer: AnimationTimer(Timer::new(
                Duration::from_millis(100),
                TimerMode::Repeating))
        }
    }
}

fn name_ldtk_players(
    mut commands: Commands,
    query: Query<Entity, (With<PlayerMarker>, Without<Name>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Name::new("Player"));
    }
}

fn debug_player_entities(query: Query<(Entity, &Name), With<PlayerMarker>>) {
    for (entity, name) in query.iter() {
        println!("Found player: {entity:?} with name {:?}", name);
    }
}

//new movement system
#[derive(Component)]
pub struct Controllable {
    pub max_speed: f32,
    pub jump_velocity: f32,
    pub acceleration: f32,
    pub grounded: bool,
    pub left: bool,
    pub right: bool,
    pub jump_requested: bool,
    pub jump_buffer_timer: Timer,
}

impl Default for Controllable {
    fn default() -> Self {
        Self{
            max_speed: 120.0,
            jump_velocity: 200.0,
            acceleration: 700.0,
            grounded: false,
            left: false,
            right: false,
            jump_requested: false,
            jump_buffer_timer: Timer::from_seconds(0.15, TimerMode::Once),
        }
    }
}

pub fn collect_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Controllable>,
) {
    for mut control in query.iter_mut() {
        control.left = keys.pressed(KeyCode::KeyA);
        control.right = keys.pressed(KeyCode::KeyD);

        if keys.just_pressed(KeyCode::Space) {
            control.jump_requested = true;
            control.jump_buffer_timer.reset();
        }

        control.jump_buffer_timer.tick(time.delta());
    }
}

pub fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Controllable)>,
) {
    for (mut velocity, control) in query.iter_mut() {
        let mut desired_velocity = 0.0;

        if control.left {
            desired_velocity -= control.max_speed;
        }
        if control.right {
            desired_velocity += control.max_speed;
        }

        let diff = desired_velocity - velocity.linvel.x;
        let accel = control.acceleration * time.delta_secs();

        velocity.linvel.x += diff.clamp(-accel, accel);
    }
}

pub fn jump_system(
    mut query: Query<(&mut Velocity, &mut Controllable)>,
) {
    for (mut velocity, mut control) in query.iter_mut() {
        control.grounded = true;

        if control.jump_requested && control.grounded {
            velocity.linvel.y = control.jump_velocity;
            control.jump_requested = false;
        }

        if control.jump_buffer_timer.finished() {
            control.jump_requested = false;
        }
    }
}

fn add_controllable_to_player(
    mut commands: Commands,
    query: Query<Entity, (With<PlayerMarker>, Without<Controllable>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Controllable::default());
    }
}
