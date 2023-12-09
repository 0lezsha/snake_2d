use crate::AppState;

use self::systems::*;
use bevy::prelude::*;

mod components;
mod systems;

impl Plugin for super::MenuStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_systems(
                OnEnter(AppState::Menu),
                (
                    //
                    init_state,
                ),
            )
            .add_systems(
                Update,
                (
                    //
                    check_interactions,
                )
                    .run_if(in_state(AppState::Menu)),
            )
            // OnExit
            .add_systems(
                OnExit(AppState::Menu),
                (
                    //
                    super::clean_state,
                ),
            );
    }
}
