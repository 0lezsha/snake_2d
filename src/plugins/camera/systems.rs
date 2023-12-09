use bevy::prelude::*;

pub fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
