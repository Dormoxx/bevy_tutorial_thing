use crate::ascii::{spawn_ascii_sprite, AsciiSheet};
use crate::fadeout::create_fadeout;
use crate::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
pub struct CombatPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(test_exit_combat.run_in_state(GameState::Combat))
            .add_enter_system(GameState::Combat, spawn_enemy)
            .add_exit_system(GameState::Combat, despawn_enemy)
            .add_system(combat_camera.run_in_state(GameState::Combat));
    }
}

fn combat_camera(mut camera_q: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_q.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(mut commands: Commands, sheet: Res<AsciiSheet>) {
    let sprite = spawn_ascii_sprite(
        &mut commands,
        &sheet,
        'b' as usize,
        Color::rgb(0.8, 0.8, 0.8),
        Vec3::new(0.0, 0.5, 100.0),
    );

    commands
        .entity(sprite)
        .insert(Enemy)
        .insert(Name::new("Bat"));
}

fn despawn_enemy(mut commands: Commands, enemy_q: Query<Entity, With<Enemy>>) {
    for entity in enemy_q.iter() {
        commands.entity(entity).despawn_recursive();
        //despawn_recursive() removes entity along with children.
    }
}

fn test_exit_combat(mut commands: Commands, keyboard: Res<Input<KeyCode>>, ascii: Res<AsciiSheet>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("exiting combat");
        create_fadeout(&mut commands, Some(GameState::Overworld), &ascii);
        //commands.insert_resource(NextState(GameState::Overworld));
    }
}
