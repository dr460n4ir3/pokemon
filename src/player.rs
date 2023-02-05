use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    tilemap::TileCollider,
    TILE_SIZE,
};

// use bevy::input::gamepad::Gamepad; // this is for gamepad functionality

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    pub speed: f32, // this is the speed of the player
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
        .add_system(camara_follow.after("movement"))
        .add_system(player_controller.label("movement"));
    }
}

// This function Player movement camera
fn camara_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

// This is the player movement controller
fn player_controller(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>, // this tells the game if the player is colliding with a wall
    keyboard_input: Res<Input<KeyCode>>, // this tells the game if the key is pressed
    // gamepad: Res<Gamepad>, // this tells the game if the gamepad is pressed
    time: Res<Time>, // this tells the game how much time has passed
) {
    let (player, mut transform) = player_query.single_mut(); // this tells the game to only have one player


    // NOTE: Only use this if you want to use the arrow keys and WASD keys at the same time PRODUCTION only
    // if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
    //     transform.translation.y += player.speed * time.delta_seconds();
    // }

    // if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
    //     transform.translation.y -= player.speed * time.delta_seconds();
    // }

    // if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
    //     transform.translation.x -= player.speed * time.delta_seconds();
    // }

    // if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
    //     transform.translation.x += player.speed * time.delta_seconds();
    // }

    // NOTE: its best to use this for Development so arrow keys can be used for testing/editing
    let mut y_delta = 0.0;
    if keyboard_input.pressed(KeyCode::W) {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    let mut x_delta = 0.0;
    if keyboard_input.pressed(KeyCode::A) {
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::D) {
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }
    

    // transform.translation += Vec3::new(x_delta, y_delta, 0.0);
    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if collision_check(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if collision_check(target, &wall_query) {
        transform.translation = target;
    }

    // NOTE: need to implement gamepad functionality
}

// This function controls collision with walls
fn collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

// This add the player sprite to the game
fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(1.0, 6.5, 0.0),
        Vec3::new(0.810, -0.636, 900.0), //this is where the player will spawn
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 4.0, // this is the speed of the player
        })
        .id();

    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.5, 0.5, 0.5),
        Vec3::new(0.0, 0.0, -1.0),
    );

    commands
        .entity(background)
        .insert(Name::new("Background"))
        .id(); // this give back the entity after it has been created

    commands.entity(player).push_children(&[background]);
}
