use bevy::prelude::*;

use crate::AsciiSheet;

// use bevy::input::gamepad::Gamepad; // this is for gamepad functionality

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub speed: f32, // this is the speed of the player
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
        app.add_system(player_controller);
    }
}

// This is the player movement controller
fn player_controller(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>, // this tells the game if the key is pressed
    // gamepad: Res<Gamepad>, // this tells the game if the gamepad is pressed
    time: Res<Time>, // this tells the game how much time has passed
) {
    let (player, mut transform) = player_query.single_mut(); // this tells the game to only have one player
    // change the following to work with multiple players: let (player, mut transform) = player_query.single_mut().unwrap();
    // let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += player.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= player.speed * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += player.speed * time.delta_seconds();
    }

    // NOTE: need to implement gamepad functionality


}

// This add the player sprite to the game
fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(1.0, 6.5, 0.0);
    sprite.custom_size = Some(Vec2::splat(1.0));

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform { 
                translation: Vec3::new(0.0, 0.0, 900.0), 
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 4.0, // this is the speed of the player
        })
        .id();

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    background_sprite.custom_size = Some(Vec2::splat(1.0));

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: background_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .id();

    commands.entity(player).push_children(&[background]);
}