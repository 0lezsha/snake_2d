use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Head {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Body {
    pub segments: VecDeque<Entity>,
}

#[derive(Component)]
pub struct Segment;

#[derive(Component)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct UiScoreLabel;
