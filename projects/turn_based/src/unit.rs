use crate::{TilePos, Unit, TILE_SIZE};
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Default)]
pub struct Stats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub movement: u32, // number of tiles the unit can move per turn
    pub range: u32, // attack range in tiles
    pub speed: u32,
    pub max_move: u32,
}

pub fn spawn_unit(
    commands: &mut Commands,
    x: u32,
    y: u32,
    kind: Unit,
    color: Color,
    offset_x: f32,
    offset_y: f32,
    stats: Stats,
) {
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(TILE_SIZE * 0.6)),
            ..default()
        },
        Transform::from_xyz(
            x as f32 * TILE_SIZE + offset_x,
            y as f32 * TILE_SIZE + offset_y,
            1.0,
        ),
        kind,
        TilePos { x, y },
        stats, // add this
    ));
}
