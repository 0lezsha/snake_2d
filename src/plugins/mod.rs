use bevy::prelude::*;

mod camera;
pub struct CameraPlugin;

mod game;
pub struct GameStatePlugin;

mod menu;
pub struct MenuStatePlugin;

//
pub fn clean_state(
    mut commands: Commands,
    entities_query: Query<Entity, (Without<Window>, Without<Camera>)>,
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn();
    }
}
//
