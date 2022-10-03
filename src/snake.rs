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
    body: Vec<Entity>,
}

#[derive(Component)]
struct SnakeBodyNode;

fn spawn_snake_head(mut commands: Commands) {
    let id_head = snake_body_node(&mut commands);

    commands.spawn().insert(Snake {
        body: Vec::from([id_head]),
        dir: SnakeDirection::RIGHT,
    });
}

fn snake_move(
    time: Res<Time>,
    query_snake: Query<&Snake>,
    mut snake_timer: ResMut<SnakeTimer>,
    mut query_snake_body: Query<(&mut Transform, &SnakeBodyNode)>,
) {
    if snake_timer.timer.tick(time.delta()).just_finished() {
        let snake_result = query_snake.get_single();
        if let Ok(snake) = snake_result {
            move_snake_tail(snake, &mut query_snake_body);
            move_snake_head(snake, &mut query_snake_body);
        }
    }
}

fn move_snake_tail(snake: &Snake, query_snake_body: &mut Query<(&mut Transform, &SnakeBodyNode)>) {
    let len = snake.body.len();
    if len <= 1 {
        return;
    }

    for index in (1..len).rev() {
        let current = snake.body[index];
        let prev = snake.body[index - 1];

        let [(mut current_transform, _), (prev_transform, _)] =
            query_snake_body.get_many_mut([current, prev]).expect("msg");

        current_transform.translation = prev_transform.translation;
    }
}

fn move_snake_head(snake: &Snake, query_snake_body: &mut Query<(&mut Transform, &SnakeBodyNode)>) {
    let (mut snake_body_transform, _) = query_snake_body.get_mut(snake.body[0]).expect("msg");

    match snake.dir {
        SnakeDirection::UP => {
            snake_body_transform.translation.y += SIZE;
        }
        SnakeDirection::DOWN => {
            snake_body_transform.translation.y -= SIZE;
        }
        SnakeDirection::LEFT => {
            snake_body_transform.translation.x -= SIZE;
        }
        SnakeDirection::RIGHT => {
            snake_body_transform.translation.x += SIZE;
        }
    }
    if snake_body_transform.translation.x > WINDOW_SIZE_HALF {
        snake_body_transform.translation.x = -WINDOW_SIZE_HALF;
    } else if snake_body_transform.translation.x < -WINDOW_SIZE_HALF {
        snake_body_transform.translation.x = WINDOW_SIZE_HALF;
    }

    if snake_body_transform.translation.y > WINDOW_SIZE_HALF {
        snake_body_transform.translation.y = -WINDOW_SIZE_HALF;
    } else if snake_body_transform.translation.y < -WINDOW_SIZE_HALF {
        snake_body_transform.translation.y = WINDOW_SIZE_HALF;
    }
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
            
            snake.body = Vec::from([snake.body[0]]);
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
            let snake_body_entity = snake_body_node(&mut commands);
            snake.body.push(snake_body_entity);
            spawn_frog(&mut commands);
            commands.entity(frog).despawn();
        }
    }
}

fn snake_body_node(commands: &mut Commands) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(SnakeBodyNode)
        .id()
}
