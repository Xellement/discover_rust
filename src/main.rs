#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION : f32 = 16.0 / 9.0;
pub const TILE_SIZE : f32 = 8.0;
pub const CAMERA_SIZE : f32 = TILE_SIZE * 4.0;
pub const PLAYER_BASE_SPEED : f32 = 3.0;

mod player;
mod debug;

use player::PlayerPlugin;
use debug::DebugPlugin;

fn main() {
    let height : f32 = 720.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "First App".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_tiles)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ships)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.top = CAMERA_SIZE;
    camera.orthographic_projection.bottom = -CAMERA_SIZE;
    camera.orthographic_projection.right = CAMERA_SIZE * RESOLUTION;
    camera.orthographic_projection.left = -CAMERA_SIZE * RESOLUTION;
    commands.spawn_bundle(camera);
}

struct TilesSheet(Handle<TextureAtlas>);

// chargement des assets 'tiles.png'
fn load_tiles(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = assets.load("tiles.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image, 
        Vec2::splat(16.0), 
        12, 
        10, 
        Vec2::splat(1.0));
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(TilesSheet(atlas_handle));
}

struct ShipsSheet(Handle<TextureAtlas>);

// chargement des assets 'ships.png'
fn load_ships(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = assets.load("ships.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image, 
        Vec2::splat(32.0), 
        4, 
        6, 
        Vec2::splat(1.0));
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(ShipsSheet(atlas_handle));
}