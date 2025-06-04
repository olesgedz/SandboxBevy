use bevy::{
    prelude::*,
    input::mouse::*
};

const TILE_SIZE: f32 = 64.0;
const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HoveredTile(None))
        .add_systems(Startup, setup)
        .add_systems(Update, highlight_tile_under_cursor)
        .run();
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct TilePos {
    x: u32,
    y: u32,
}

#[derive(Resource)]
struct HoveredTile(Option<Entity>);


fn setup(mut commands: Commands) {
    // Spawn 2D camera
    commands.spawn(Camera2d);

    // Center offset
    let offset_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;
    let offset_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;

    // Spawn grid tiles
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.2, 0.2, 0.8),
                    custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)), // 2px gap between tiles
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE + offset_x,
                    y as f32 * TILE_SIZE + offset_y,
                    0.0,
                ),
                Tile,
                TilePos { x, y },
            ));
        }
    }
}

fn highlight_tile_under_cursor(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut tiles: Query<(Entity, &mut Sprite, &Transform), With<Tile>>,
    mut hovered: ResMut<HoveredTile>,
) {
    let window = windows.single();
    let (camera, cam_transform) = camera_q.single();

    if let Some(cursor_pos) = window.cursor_position() {
        // Convert screen coordinates to world space
        if let Ok(world_pos) = camera.viewport_to_world(cam_transform, cursor_pos) {
            let cursor_world = world_pos.origin.truncate();

            // Find tile under cursor
            let mut new_hovered = None;

            for (entity, mut sprite, transform) in &mut tiles {
                let pos = transform.translation.truncate();
                let half_size = TILE_SIZE / 2.0;

                let in_bounds = (cursor_world.x >= pos.x - half_size
                    && cursor_world.x <= pos.x + half_size
                    && cursor_world.y >= pos.y - half_size
                    && cursor_world.y <= pos.y + half_size);

                if in_bounds {
                    sprite.color = Color::srgb(0.2, 0.8, 0.2); // hover color
                    new_hovered = Some(entity);
                } else if Some(entity) == hovered.0 {
                    // Restore color for previously hovered tile
                    sprite.color = Color::srgb(0.2, 0.2, 0.8);
                }
            }

            hovered.0 = new_hovered;
        }
    }
}