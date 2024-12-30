use bevy::prelude::*;
use std::time::Duration;

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
