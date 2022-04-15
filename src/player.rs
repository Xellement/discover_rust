use bevy::prelude::*;

use crate::{ShipsSheet, TilesSheet, TILE_SIZE, PLAYER_BASE_SPEED};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::Z) {
        transform.translation.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Q) {
        transform.translation.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += player.speed * TILE_SIZE * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, ships: Res<ShipsSheet>, tiles: Res<TilesSheet>) {
    let mut player_sprite = TextureAtlasSprite::new(9);
    player_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: player_sprite,
            texture_atlas: ships.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: PLAYER_BASE_SPEED
        })
        .id();

    let mut bg_sprite = TextureAtlasSprite::new(50);
    bg_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let bg = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: bg_sprite,
            texture_atlas: tiles.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .id();

    commands.entity(player).push_children(&[bg]);
}
