use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE,
};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct EncounterSpawn;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_map);
    }
}

fn spawn_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("Failed to load map file"); // this code makes the game read the map.tx file
    let mut tiles = Vec::new();

    // NOTE: this code runs through the map.txt file and spawns the sprites
    /* # = Out of bounds,
       W = Wall  
       G = Grass Encounter
       */
    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(1.0, 0.4745, 0.0),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                if char == 'W' {
                    commands.entity(tile).insert(TileCollider);
                }
                if char == 'G' {
                    commands.entity(tile).insert(EncounterSpawn);
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
