use crate::{resources::Score, AppState};

use super::{components::*, events::*};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::Rng;

const GRID_SIZE: f32 = 30.;

pub fn init_state(
    mut commands: Commands,
    mut spawn_food_event: EventWriter<SpawnFoodEvent>,
    mut score: ResMut<Score>,
) {
    // Draw grid
    let grid_color = Color::DARK_GRAY;

    for x in -9..10 {
        let x = x as f32;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: grid_color,
                custom_size: Some(Vec2::new(1., 600.)),
                ..default()
            },
            transform: Transform::from_xyz(x * GRID_SIZE, 0., 0.),
            ..default()
        });
    }

    for y in -9..10 {
        let y = y as f32;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: grid_color,
                custom_size: Some(Vec2::new(600., 1.)),
                ..default()
            },
            transform: Transform::from_xyz(0., y * GRID_SIZE, 0.),
            ..default()
        });
    }
    //

    // Spawn snake head
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::splat(GRID_SIZE * 0.8)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        Head {
            direction: Vec2::new(1., 0.),
        },
    ));
    //

    // Spawn snake body
    let segment_color = Color::DARK_GREEN;

    let segments: Vec<Entity> = vec![
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: segment_color,
                        custom_size: Some(Vec2::splat(GRID_SIZE * 0.5)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-GRID_SIZE, 0., 1.),
                    ..default()
                },
                Segment,
            ))
            .id(),
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: segment_color,
                        custom_size: Some(Vec2::splat(GRID_SIZE * 0.5)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-GRID_SIZE * 2., 0., 1.),
                    ..default()
                },
                Segment,
            ))
            .id(),
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: segment_color,
                        custom_size: Some(Vec2::splat(GRID_SIZE * 0.5)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-GRID_SIZE * 3., 0., 1.),
                    ..default()
                },
                Segment,
            ))
            .id(),
    ];

    commands.spawn(Body {
        segments: segments.into(),
    });
    //

    // Spawn walls
    let walls_color = Color::MIDNIGHT_BLUE;

    let (width, height) = (GRID_SIZE * 9., GRID_SIZE * 1.);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: walls_color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(-300. + width / 2., 300. - height / 2., 2.),
            ..default()
        },
        Collider { width, height },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: walls_color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(300. - width / 2., 300. - height / 2., 2.),
            ..default()
        },
        Collider { width, height },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: walls_color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(-300. + width / 2., -300. + height / 2., 2.),
            ..default()
        },
        Collider { width, height },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: walls_color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(300. - width / 2., -300. + height / 2., 2.),
            ..default()
        },
        Collider { width, height },
    ));

    let (width, height) = (GRID_SIZE * 1., GRID_SIZE * 9.);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: walls_color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(-300. + width / 2., 300. - height / 2., 2.),
            ..default()
        },
        Collider { width, height },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: walls_color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(-300. + width / 2., -300. + height / 2., 2.),
            ..default()
        },
        Collider { width, height },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: walls_color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(300. - width / 2., 300. - height / 2., 2.),
            ..default()
        },
        Collider { width, height },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: walls_color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(300. - width / 2., -300. + height / 2., 2.),
            ..default()
        },
        Collider { width, height },
    ));
    //

    // Spawn food
    spawn_food_event.send_default();
    //

    //Reset current score
    score.current = 0;
    //

    // Spawn UI
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();

    let score_label = commands
        .spawn((
            TextBundle::from_section("score value", TextStyle::default()),
            UiScoreLabel,
        ))
        .id();

    commands.entity(root).push_children(&[score_label]);
    //
}

pub fn snake_movement(
    mut commands: Commands,
    mut head_query: Query<(&mut Transform, &Head)>,
    mut body_query: Query<&mut Body>,
    mut snake_grow_event: EventReader<SnakeGrowEvent>,
) {
    let (mut head_transform, head) = head_query
        .get_single_mut()
        .expect("cant't get single mut head");

    // Head movement
    head_transform.translation.x += head.direction.x * GRID_SIZE;
    head_transform.translation.y += head.direction.y * GRID_SIZE;

    if head_transform.translation.x > 300. {
        head_transform.translation.x = -300.;
    } else if head_transform.translation.x < -300. {
        head_transform.translation.x = 300.;
    }

    if head_transform.translation.y > 300. {
        head_transform.translation.y = -300.;
    } else if head_transform.translation.y < -300. {
        head_transform.translation.y = 300.;
    }
    //

    let mut body = body_query
        .get_single_mut()
        .expect("can't get single mut snake body!");

    // Body movement
    if let Some(last) = body.segments.back() {
        if snake_grow_event.is_empty() {
            // Remove last segment
            commands.entity(*last).despawn();
            body.segments.pop_back();
            //
        }

        snake_grow_event.clear();

        // Spawn new segment
        body.segments.push_front(
            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::DARK_GREEN,
                            custom_size: Some(Vec2::splat(GRID_SIZE * 0.5)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            head_transform.translation.x - GRID_SIZE * head.direction.x,
                            head_transform.translation.y - GRID_SIZE * head.direction.y,
                            1.,
                        ),
                        ..default()
                    },
                    Segment,
                ))
                .id(),
        );
        //
    }
    //
}

pub fn snake_direction_input(mut head_query: Query<&mut Head>, keyboard: Res<Input<KeyCode>>) {
    let mut head = head_query
        .get_single_mut()
        .expect("cant't get single mut head");

    if keyboard.just_pressed(KeyCode::A) {
        head.direction = Vec2::new(-1., 0.);
    } else if keyboard.just_pressed(KeyCode::D) {
        head.direction = Vec2::new(1., 0.);
    } else if keyboard.just_pressed(KeyCode::W) {
        head.direction = Vec2::new(0., 1.);
    } else if keyboard.just_pressed(KeyCode::S) {
        head.direction = Vec2::new(0., -1.);
    }
}

pub fn check_if_snake_eat_itself(
    mut game_over_event: EventWriter<GameOverEvent>,
    head_query: Query<&Transform, With<Head>>,
    segments_query: Query<&Transform, With<Segment>>,
) {
    let head = head_query.get_single().expect("can't get single head");

    for segment in segments_query.iter() {
        if head == segment {
            game_over_event.send_default();
        }
    }
}

pub fn check_walls_collisions(
    mut game_over_event: EventWriter<GameOverEvent>,
    head_query: Query<&Transform, With<Head>>,
    colliders_query: Query<(&Transform, &Collider)>,
) {
    let head = head_query.get_single().expect("can't get single head");

    for (collider_transform, collider_dimension) in colliders_query.iter() {
        if collide(
            head.translation,
            Vec2::splat(GRID_SIZE),
            collider_transform.translation,
            Vec2::new(collider_dimension.width, collider_dimension.height),
        )
        .is_some()
        {
            game_over_event.send_default();
        }
    }
}

pub fn spawn_food(mut commands: Commands, mut spawn_food_event: EventReader<SpawnFoodEvent>) {
    for _ in spawn_food_event.read() {
        let x = rand::thread_rng().gen_range(-8..=8) as f32 * GRID_SIZE;
        let y = rand::thread_rng().gen_range(-8..=8) as f32 * GRID_SIZE;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::splat(GRID_SIZE * 0.6)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 1.),
                ..default()
            },
            Food,
        ));
    }
}

pub fn check_food_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut spawn_food_event: EventWriter<SpawnFoodEvent>,
    mut snake_grow_event: EventWriter<SnakeGrowEvent>,
    head_query: Query<&Transform, With<Head>>,
    food_query: Query<(&Transform, Entity), With<Food>>,
) {
    let head = head_query.get_single().expect("can't get single head");

    for (food_transform, food_entity) in food_query.iter() {
        if head == food_transform {
            score.current += 1;
            spawn_food_event.send_default();
            commands.entity(food_entity).despawn();
            snake_grow_event.send_default();
        }
    }
}

pub fn update_score_label(
    mut score_label: Query<&mut Text, With<UiScoreLabel>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        let mut score_label = score_label
            .get_single_mut()
            .expect("can't get single score label");

        score_label.sections[0].value = format!("Your score is: {}", score.current);
    }
}

pub fn check_game_over(
    mut game_over_event: EventReader<GameOverEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    mut score: ResMut<Score>,
) {
    for _ in game_over_event.read() {
        app_state.set(AppState::Menu);

        if score.current > score.max {
            score.max = score.current;
        }
    }
}
