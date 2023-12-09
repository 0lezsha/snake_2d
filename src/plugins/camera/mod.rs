use self::systems::*;
use bevy::prelude::*;

mod systems;

impl Plugin for super::CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
    }
}
