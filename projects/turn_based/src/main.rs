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
        .insert_resource(SelectedUnit(None))
        .add_systems(Startup, setup)
        .add_systems(Update,
                     (
                         highlight_tile_under_cursor,
                         handle_clicks,
                     ))
        .run();
}

#[derive(Component)]
struct Tile;

#[derive(Component, Clone, PartialEq, Eq)]
struct TilePos {
    x: u32,
    y: u32,
}

#[derive(Resource)]
struct HoveredTile(Option<Entity>);

#[derive(Resource)]
struct SelectedUnit(Option<Entity>);

#[derive(Component)]
enum Unit {
    Player,
    Enemy,
}


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

    // Spawn player unit at (1, 1)
    spawn_unit(&mut commands, 1, 1, Unit::Player, Color::srgb(0.2, 1.0, 0.2), offset_x, offset_y);

    // Spawn enemy unit at (8, 8)
    spawn_unit(&mut commands, 8, 8, Unit::Enemy, Color::srgb(1.0, 0.2, 0.2), offset_x, offset_y);
}

fn spawn_unit(
    commands: &mut Commands,
    x: u32,
    y: u32,
    kind: Unit,
    color: Color,
    offset_x: f32,
    offset_y: f32,
) {
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(TILE_SIZE * 0.6)), // smaller than tile
            ..default()
        },
        Transform::from_xyz(
            x as f32 * TILE_SIZE + offset_x,
            y as f32 * TILE_SIZE + offset_y,
            1.0, // put on top of tiles
        ),
        kind,
        TilePos { x, y },
    ));
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

                let in_bounds = cursor_world.x >= pos.x - half_size
                    && cursor_world.x <= pos.x + half_size
                    && cursor_world.y >= pos.y - half_size
                    && cursor_world.y <= pos.y + half_size;

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

fn handle_clicks(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut unit_query: Query<(Entity, &mut TilePos, &mut Sprite), (With<Unit>, Without<Tile>)>,
    tile_query: Query<(&TilePos, &Transform), (With<Tile>, Without<Unit>)>,
    mut selected: ResMut<SelectedUnit>,
    mut unit_transforms: Query<&mut Transform, With<Unit>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, cam_transform) = camera_q.single();

    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_pos) else { return };
    let cursor_world = ray.origin.truncate();

    // First: check if we clicked on a unit
    for (entity, pos, sprite) in unit_query.iter() {
        let world_x = pos.x as f32 * TILE_SIZE - (GRID_WIDTH as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;
        let world_y = pos.y as f32 * TILE_SIZE - (GRID_HEIGHT as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;

        let half = TILE_SIZE * 0.5;
        if cursor_world.x >= world_x - half && cursor_world.x <= world_x + half &&
            cursor_world.y >= world_y - half && cursor_world.y <= world_y + half {

            // Deselect previously selected
            if let Some(prev) = selected.0 {
                if let Ok(mut old_sprite) = unit_query.get(prev).map(|(_, _, sprite)| sprite.clone()) {
                    old_sprite.color.set_alpha(1.0); // Reset alpha
                }
            }

            // Select this unit
            selected.0 = Some(entity);
            if let Ok(mut sprite) = unit_query.get_mut(entity) {
                sprite.2.color.set_alpha(0.6); // sprite.2 = Sprite component
            }
            return;
        }
    }

    // Then: if a unit is selected, and we clicked a tile, move the unit
    if let Some(selected_entity) = selected.0 {
        for (tile_pos, tile_transform) in &tile_query {
            let world_pos = tile_transform.translation.truncate();
            let half = TILE_SIZE * 0.5;
            if cursor_world.x >= world_pos.x - half && cursor_world.x <= world_pos.x + half &&
                cursor_world.y >= world_pos.y - half && cursor_world.y <= world_pos.y + half {

                if let Ok(mut transform) = unit_transforms.get_mut(selected_entity) {
                    transform.translation = tile_transform.translation + Vec3::new(0.0, 0.0, 1.0);
                }

                // Update unit’s logical position
                if let Ok((_, mut unit_tile_pos, _)) = unit_query.get_mut(selected_entity) {
                    *unit_tile_pos = tile_pos.clone();
                }

                selected.0 = None; // Deselect
                return;
            }
        }
    }
}