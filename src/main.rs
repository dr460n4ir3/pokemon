#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};

// This draws the screen
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

// This will point the main function to the player plugin and the player.rs file
mod ascii;
mod debug;
mod player;
mod tilemap;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use player::PlayerPlugin;
use tilemap::TileMapPlugin;

// This is the game states
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    // Menu,
    // Playing,
    // Paused,
    // GameOver,
    Overworld,
    Battle,
}

// This is the main function also hold systems
fn main() {
    let height = 900.0;
    App::new()
    .add_state(GameState::Overworld)
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Dr460n4ir3".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)        
        .add_plugin(AsciiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TileMapPlugin)
        .run();
}

// This is the game camara
fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}