use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use iyes_loopless::prelude::*;
//use std::time::Duration;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const GLOBAL_SIZE: f32 = 0.1;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
// struct MyFixedUpdate;

mod ascii;
mod debug;
mod player;
mod simple_tilemap;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use player::PlayerPlugin;
use simple_tilemap::SimpleTileMapPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    Overworld,
    Combat,
}

fn main() {
    let height = 900.0;
    let window_params = WindowDescriptor {
        width: height * RESOLUTION,
        height,
        title: "Bevy Youtube Tutorial".to_string(),
        present_mode: PresentMode::Fifo,
        ..Default::default()
    };

    // let mut test_stage = SystemStage::parallel();
    // test_stage.add_system(test_system);

    App::new()
        .insert_resource(window_params)
        .add_plugins(DefaultPlugins)
        .add_loopless_state(GameState::Overworld)
        .add_loopless_state(GameState::Combat)
        .add_plugin(AsciiPlugin)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_startup_system(hot_reload)
        .add_plugin(DebugPlugin)
        .add_plugin(SimpleTileMapPlugin)
        //.add_stage_before(CoreStage::Update, MyFixedUpdate, FixedTimestepStage::new(Duration::from_micros(16660)).with_stage(test_stage))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.left = -1. * RESOLUTION;
    camera.orthographic_projection.right = 1. * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

fn hot_reload(server: Res<AssetServer>) {
    server.watch_for_changes().unwrap();
}

// fn test_system(info: Res<FixedTimestepInfo>) {
//     println!(
//         "Fixed Timestep duration: {:?} ({} Hz)",
//         info.timestep(),
//         info.rate()
//     );
//     println!(
//         "Overstepped by {:.2?} ({:.2}%).",
//         info.remaining(),
//         info.overstep() * 100.0
//     );
// }
