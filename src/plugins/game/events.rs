use bevy::prelude::*;

#[derive(Event, Default)]
pub struct SpawnFoodEvent;

#[derive(Event, Default)]
pub struct SnakeGrowEvent;

#[derive(Event, Default)]
pub struct GameOverEvent;