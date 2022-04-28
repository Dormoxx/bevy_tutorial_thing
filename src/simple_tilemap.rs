use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    GLOBAL_SIZE,
};
use bevy::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct SimpleTileMapPlugin;

#[derive(Component)]
pub struct SimpleTileCollider;

#[derive(Component)]
pub struct EncounterSpawner;

impl Plugin for SimpleTileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map);
    }
}

fn create_map(mut commands: Commands, sheet: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("no file found: assets/map.txt");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &sheet,
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * GLOBAL_SIZE, -(y as f32) * GLOBAL_SIZE, 100.0),
                );
                if char == '#' {
                    commands.entity(tile).insert(SimpleTileCollider);
                }
                if char == '~' {
                    commands.entity(tile).insert(EncounterSpawner);
                }
                tiles.push(tile);
            }
        }
    }
    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}
