use bevy::prelude::*;
use plugins::*;
use resources::*;

mod plugins;
mod resources;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Menu,
    // #[default]
    Game,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Simple 2D Snake Game".to_string(),
                    resolution: (600., 600.).into(),
                    ..default()
                }),
                ..default()
            }),
            CameraPlugin,
            GameStatePlugin,
            MenuStatePlugin,
        ))
        .insert_resource(Time::<Fixed>::from_seconds(0.2))
        .init_resource::<Score>()
        .add_state::<AppState>()
        .run();
}
