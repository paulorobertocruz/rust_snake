use std::collections::VecDeque;

use crate::{
    consts::*,
    frog::{spawn_frog, Frog},
};
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct SnakePlugin;

struct SnakeTimer {
    timer: Timer,
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeTimer {
            timer: Timer::from_seconds(0.05, true),
        });
        app.add_startup_system(spawn_snake_head);
        app.add_system(snake_move);
        app.add_system(snake_input);
        app.add_system(collide_frog);
        app.add_system(collide_snake_body);
    }
}

enum SnakeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Component)]
struct Snake {
    dir: SnakeDirection,
    body: VecDeque<Entity>,
}

#[derive(Component)]
struct SnakeBodyNode;

fn spawn_snake_head(mut commands: Commands) {
    let id_head = snake_body_node(
        &mut commands,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );

    commands.spawn().insert(Snake {
        body: VecDeque::from([id_head]),
        dir: SnakeDirection::RIGHT,
    });
}

fn snake_move(
    time: Res<Time>,
    mut query_snake: Query<&mut Snake>,
    mut snake_timer: ResMut<SnakeTimer>,
    mut query_snake_body: Query<&mut Transform, With<SnakeBodyNode>>,
) {
    if snake_timer.timer.tick(time.delta()).just_finished() {
        let mut snake = query_snake.single_mut();

        let &first_node = snake.body.front().unwrap();
        let &last_node = snake.body.back().unwrap();

        if first_node == last_node {
            let mut first_transform = query_snake_body.get_mut(first_node).unwrap();
            let head_translation =
                get_next_head_translation(first_transform.translation, &snake.dir);
            first_transform.translation = head_translation;
        } else {
            let [first_transform, mut last_transform] =
                query_snake_body.many_mut([first_node, last_node]);

            let head_translation =
                get_next_head_translation(first_transform.translation, &snake.dir);
            last_transform.translation = head_translation;

            let last = snake.body.pop_back().unwrap();
            snake.body.push_front(last);
        }
    }
}

fn get_next_head_translation(current: Vec3, dir: &SnakeDirection) -> Vec3 {
    let mut new_position = Vec3 { ..current };
    match dir {
        SnakeDirection::UP => {
            new_position.y += SIZE;
        }
        SnakeDirection::DOWN => {
            new_position.y -= SIZE;
        }
        SnakeDirection::LEFT => {
            new_position.x -= SIZE;
        }
        SnakeDirection::RIGHT => {
            new_position.x += SIZE;
        }
    }
    if new_position.x > WINDOW_SIZE_HALF {
        new_position.x = -WINDOW_SIZE_HALF;
    } else if new_position.x < -WINDOW_SIZE_HALF {
        new_position.x = WINDOW_SIZE_HALF;
    }

    if new_position.y > WINDOW_SIZE_HALF {
        new_position.y = -WINDOW_SIZE_HALF;
    } else if new_position.y < -WINDOW_SIZE_HALF {
        new_position.y = WINDOW_SIZE_HALF;
    }

    return new_position;
}

fn snake_input(mut query_snake: Query<&mut Snake>, keyboard: Res<Input<KeyCode>>) {
    let snake_result = query_snake.get_single_mut();

    match snake_result {
        Ok(mut snake) => {
            if keyboard.pressed(KeyCode::Left) {
                snake.dir = SnakeDirection::LEFT;
            } else if keyboard.pressed(KeyCode::Right) {
                snake.dir = SnakeDirection::RIGHT;
            } else if keyboard.pressed(KeyCode::Up) {
                snake.dir = SnakeDirection::UP;
            } else if keyboard.pressed(KeyCode::Down) {
                snake.dir = SnakeDirection::DOWN;
            }
        }
        Err(_) => todo!(),
    }
}

fn collide_snake_body(
    mut commands: Commands,
    mut query_snake: Query<&mut Snake>,
    query_snake_body: Query<(Entity, &Transform, &Sprite), With<SnakeBodyNode>>,
) {
    let mut snake = query_snake.get_single_mut().expect("snake query");
    let (snake_head, snake_head_tranform, snake_head_sprite) =
        query_snake_body.get(snake.body[0]).expect("msg");
    let snake_head_size = snake_head_sprite.custom_size.expect("msg");

    for (snake_body, snake_body_tranform, snake_body_sprite) in &query_snake_body {
        if snake_head == snake_body {
            continue;
        }
        let snake_body_size = snake_body_sprite.custom_size.expect("msg");
        let collided = collide(
            snake_head_tranform.translation,
            snake_head_size,
            snake_body_tranform.translation,
            snake_body_size,
        );

        if let Some(_) = collided {
            let len = snake.body.len();
            if len < 2 {
                return;
            }

            for index in 1..len {
                let entity = snake.body[index];
                commands.entity(entity).despawn();
            }

            snake.body = VecDeque::from([snake.body[0]]);
        }
    }
}

fn collide_frog(
    mut commands: Commands,
    mut query_snake: Query<&mut Snake>,
    query_snake_body: Query<(&Transform, &Sprite), With<SnakeBodyNode>>,
    query_frog: Query<(Entity, &Transform, &Sprite), With<Frog>>,
) {
    let mut snake = query_snake.get_single_mut().expect("whats??");
    let (snake_head_transform, snake_head_sprite) =
        query_snake_body.get(snake.body[0]).expect("whats??");

    let snake_head_size = snake_head_sprite.custom_size.expect("não tem ");

    for (frog, frog_transform, frog_sprite) in query_frog.iter() {
        let frog_size = frog_sprite.custom_size.expect("msg");

        let collided = collide(
            snake_head_transform.translation,
            snake_head_size,
            frog_transform.translation,
            frog_size,
        );

        if let Some(_) = collided {
            let snake_body_entity = snake_body_node(
                &mut commands,
                Vec3 {
                    x: WINDOW_SIZE,
                    y: WINDOW_SIZE,
                    z: 0.0,
                },
            );
            snake.body.push_back(snake_body_entity);
            spawn_frog(&mut commands);
            commands.entity(frog).despawn();
        }
    }
}

fn snake_body_node(commands: &mut Commands, translation: Vec3) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                ..Default::default()
            },
            transform: Transform::from_translation(translation),

            ..Default::default()
        })
        .insert(SnakeBodyNode)
        .id()
}
