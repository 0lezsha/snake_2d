use crate::AppState;

use self::{events::*, systems::*};
use bevy::prelude::*;

mod components;
mod events;
mod systems;

impl Plugin for super::GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<SpawnFoodEvent>()
            .add_event::<SnakeGrowEvent>()
            .add_event::<GameOverEvent>()
            // OnEnter
            .add_systems(
                OnEnter(AppState::Game),
                (
                    //
                    init_state,
                ),
            )
            // Update
            .add_systems(
                Update,
                (
                    //
                    snake_direction_input,
                    spawn_food,
                    update_score_label,
                    check_game_over,
                )
                    .run_if(in_state(AppState::Game)),
            )
            // FixedUpdate
            .add_systems(
                FixedUpdate,
                (
                    //
                    (
                        snake_movement,
                        check_if_snake_eat_itself,
                        check_walls_collisions,
                        check_food_collisions,
                    )
                        .chain(),
                )
                    .run_if(in_state(AppState::Game)),
            )
            // OnExit
            .add_systems(
                OnExit(AppState::Game),
                (
                    //
                    super::clean_state,
                ),
            );
    }
}
