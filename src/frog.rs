use crate::consts::*;
use bevy::prelude::*;
use rand::prelude::*;

pub struct FrogPlugin;

impl Plugin for FrogPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_first_frog);
    }
}

#[derive(Component)]
pub struct Frog;

fn spawn_first_frog(mut commands: Commands) {
    spawn_frog(&mut commands);
}


pub fn spawn_frog(commands: &mut Commands){
    commands.spawn_bundle(get_frog_sprite_bundle()).insert(Frog);
}

fn get_frog_sprite_bundle() -> SpriteBundle {
    let mut rng = thread_rng();
    let step = WINDOW_SIZE_HALF / SIZE;
    let x: i32 = rng.gen_range(-step as i32..step as i32);
    let y: i32 = rng.gen_range(-step as i32..step as i32);

    SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3 { x: x as f32 * SIZE, y: y as f32 * SIZE, z: 0.0 },
            ..Default::default()
        },
        ..Default::default()
    }
}
