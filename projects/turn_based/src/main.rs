mod unit;
mod logic;

use bevy::prelude::Color;
use bevy::{input::mouse::*, prelude::*};

use bevy::color::Color::Srgba;
use bevy::text::cosmic_text::Wrap::Word;
use logic::*;
use logic::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use unit::*;


const TILE_SIZE: f32 = 64.0;
const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 10;

fn is_player_turn(turn: Res<Turn>) -> bool {
    *turn == Turn::Player
}

fn is_ai_turn(turn: Res<Turn>) -> bool {
    *turn == Turn::AI
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HoveredTile(None))
        .insert_resource(SelectedUnit(None))
        .insert_resource(Turn::Player)
        .insert_resource(AIDone(true))
        .insert_resource(PlayerDone(false))
        .add_event::<EndTurnEvent>()
        .add_systems(Startup, (setup, setup_turn_queue))
        .add_systems(Update, highlight_tile_under_cursor)
        .add_systems(Update, (handle_clicks.run_if(is_player_turn), highlight_reachable_tiles.after(handle_clicks),))
        .add_systems(Update, end_player_turn)
        .add_systems(Update, ai_turn_system.run_if(is_ai_turn))
        .add_systems(Update, update_turn_text)
        .add_systems(Update, end_ai_turn)
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

#[derive(Component)]
struct TurnText;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
enum Turn {
    Player,
    AI,
}

#[derive(Resource, Default)]
struct PlayerDone(bool);

#[derive(Resource, Default)]
struct AIDone(bool);

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

    spawn_unit(
        &mut commands,
        1,
        1,
        Unit::Player,
        Color::srgb(0.2, 1.0, 0.2),
        offset_x,
        offset_y,
        Stats {
            hp: 10,
            max_hp: 10,
            attack: 4,
            defense: 1,
            movement: 3,
            range: 1,
            ..default()
        },
    );

    spawn_unit(
        &mut commands,
        8,
        8,
        Unit::Enemy,
        Color::srgb(1.0, 0.2, 0.2),
        offset_x,
        offset_y,
        Stats {
            hp: 6,
            max_hp: 6,
            attack: 3,
            defense: 0,
            movement: 2,
            range: 1,
            ..default()
        },
    );

    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("Player Turn"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font_size: 67.0,
            ..default()
        },
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        TurnText,
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
    turn: Res<Turn>,
    mut selected: ResMut<SelectedUnit>,
    mut unit_query: Query<(Entity, &mut TilePos, &mut Sprite, &Stats), (With<Unit>, Without<Tile>)>,
    mut tile_query: Query<(&TilePos, &Transform, &mut Sprite), (With<Tile>, Without<Unit>)>,
    mut unit_transforms: Query<&mut Transform, With<Unit>>,
    mut player_done: ResMut<PlayerDone>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, cam_transform) = camera_q.single();

    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_pos) else { return };
    let cursor_world = ray.origin.truncate();

    // Only handle player input during player turn
    if *turn != Turn::Player {
        return;
    }

    // Try to select a unit
    if try_select_unit(
        cursor_world,
        &mut selected,
        &mut unit_query,
    ) {
        return; // successfully selected, exit early
    }

    // If already selected, try to move it
    if let Some(selected_entity) = selected.0 {
        if try_move_selected_unit(
            selected_entity,
            cursor_world,
            &mut unit_query,
            &mut tile_query,
            &mut unit_transforms,
        ) {
            selected.0 = None;
            player_done.0 = true;
        }
    }
}

fn try_select_unit(
    cursor_world: Vec2,
    selected: &mut ResMut<SelectedUnit>,
    unit_query: &mut Query<(Entity, &mut TilePos, &mut Sprite, &Stats), (With<Unit>, Without<Tile>)>,
) -> bool {
    for (entity, pos, mut sprite, _) in unit_query.iter_mut() {
        let world_x = pos.x as f32 * TILE_SIZE - (GRID_WIDTH as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;
        let world_y = pos.y as f32 * TILE_SIZE - (GRID_HEIGHT as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;
        let half = TILE_SIZE * 0.5;

        if cursor_world.x >= world_x - half && cursor_world.x <= world_x + half &&
            cursor_world.y >= world_y - half && cursor_world.y <= world_y + half {

            // Deselect old unit (no need to reset alpha here unless you store previous selection)
            selected.0 = Some(entity);
            sprite.color.set_alpha(0.6);
            return true;
        }
    }
    false
}

fn try_move_selected_unit(
    selected_entity: Entity,
    cursor_world: Vec2,
    unit_query: &mut Query<(Entity, &mut TilePos, &mut Sprite, &Stats), (With<Unit>, Without<Tile>)>,
    tile_query: &mut Query<(&TilePos, &Transform, &mut Sprite), (With<Tile>, Without<Unit>)>,
    unit_transforms: &mut Query<&mut Transform, With<Unit>>,
) -> bool {
    let Ok((_, mut unit_pos, _, stats)) = unit_query.get_mut(selected_entity) else {
        return false;
    };

    for (tile_pos, tile_transform, _) in tile_query.iter_mut() {
        let world_pos = tile_transform.translation.truncate();
        let half = TILE_SIZE * 0.5;

        if cursor_world.x >= world_pos.x - half && cursor_world.x <= world_pos.x + half &&
            cursor_world.y >= world_pos.y - half && cursor_world.y <= world_pos.y + half {
            let dx = (tile_pos.x as i32 - unit_pos.x as i32).abs();
            let dy = (tile_pos.y as i32 - unit_pos.y as i32).abs();
            if dx + dy > stats.movement as i32 {
                info!("Tile too far");
                return false;
            }

            if let Ok(mut transform) = unit_transforms.get_mut(selected_entity) {
                transform.translation = tile_transform.translation + Vec3::new(0.0, 0.0, 1.0);
            }

            *unit_pos = tile_pos.clone();
            return true;
        }
    }

    false
}

fn end_ai_turn(mut turn: ResMut<Turn>, mut done: ResMut<AIDone>) {
    if done.0 {
        done.0 = false;
        *turn = Turn::Player;
        info!("Switching to Player turn");
    }
}

fn end_player_turn(mut turn: ResMut<Turn>, mut done: ResMut<PlayerDone>) {
    if done.0 {
        done.0 = false;
        *turn = Turn::AI;
        info!("Switching to AI turn");
    }
}

/// One AI turn: pick each enemy unit and move it to a random adjacent empty tile.
/// When every enemy has tried to move, mark the turn as done.
fn ai_turn_system(
    mut done: ResMut<AIDone>,
    mut unit_query: Query<(Entity, &mut TilePos, &mut Transform, &Unit)>,
) {
    // Build a quick‐lookup set of all occupied tiles.
    let mut occupied: HashSet<(u32, u32)> = HashSet::new();
    for (_, pos, _, _) in unit_query.iter() {
        occupied.insert((pos.x, pos.y));
    }

    // Directions the AI can try (right, left, up, down).
    let mut dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
    let mut rng = thread_rng();

    for (_, mut pos, mut transform, unit_kind) in unit_query.iter_mut() {
        // We only move enemy units.
        if !matches!(unit_kind, Unit::Enemy) {
            continue;
        }

        dirs.shuffle(&mut rng);

        // Try the four neighbouring tiles in random order.
        for (dx, dy) in dirs {
            let nx = pos.x as i32 + dx;
            let ny = pos.y as i32 + dy;

            // Stay inside the board.
            if nx < 0 || ny < 0 || nx >= GRID_WIDTH as i32 || ny >= GRID_HEIGHT as i32 {
                continue;
            }

            let nxu = nx as u32;
            let nyu = ny as u32;

            // Skip if another unit is already there.
            if occupied.contains(&(nxu, nyu)) {
                continue;
            }

            // === Move the unit ===
            occupied.remove(&(pos.x, pos.y)); // old spot is now free
            occupied.insert((nxu, nyu)); // new spot is taken

            pos.x = nxu;
            pos.y = nyu;

            // Convert grid coords → world coords.
            let offset_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;
            let offset_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0 + TILE_SIZE / 2.0;
            transform.translation = Vec3::new(
                nxu as f32 * TILE_SIZE + offset_x,
                nyu as f32 * TILE_SIZE + offset_y,
                1.0,
            );
            break; // move only once per unit
        }
    }

    // AI finished its turn.
    done.0 = true;
}

fn highlight_reachable_tiles(
    selected: Res<SelectedUnit>,
    unit_query: Query<(&TilePos, &Stats), With<Unit>>,
    mut tile_query: Query<(&TilePos, &mut Sprite), With<Tile>>,
) {
    // First, clear all highlights
    for (_, mut sprite) in tile_query.iter_mut() {
        sprite.color = Color::srgb(0.0, 0.0, 1.0);
    }

    // If no unit is selected, stop here
    let Some(selected_entity) = selected.0 else { return };

    // Get the selected unit's position and movement
    let Ok((unit_pos, stats)) = unit_query.get(selected_entity) else { return };

    // Highlight reachable tiles
    for (tile_pos, mut sprite) in tile_query.iter_mut() {
        let dx = (tile_pos.x as i32 - unit_pos.x as i32).abs();
        let dy = (tile_pos.y as i32 - unit_pos.y as i32).abs();
        if dx + dy <= stats.movement as i32 {
            sprite.color = Color::srgb(0.2, 0.4, 0.4); // Cyan-ish highlight
        }
    }
}

fn update_turn_text(turn: Res<Turn>, mut text_query: Query<&mut Text, With<TurnText>>) {
    if let Ok(mut text) = text_query.get_single_mut() {
        text.0 = match *turn {
            Turn::Player => "Player Turn".to_string(),
            Turn::AI => "AI Turn".to_string(),
        };
    }
}
