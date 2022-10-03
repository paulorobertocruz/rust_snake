use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(game_startup_system);
    }
}

fn game_startup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
