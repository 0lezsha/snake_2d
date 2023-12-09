use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub current: u32,
    pub max: u32,
}