mod game;
mod snake;
mod frog;
mod consts;

use bevy::prelude::*;
use game::GamePlugin;
use snake::SnakePlugin;
use frog::FrogPlugin;
use consts::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            title: "Snake Game".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(SnakePlugin)
        .add_plugin(FrogPlugin)
        .run();
}
