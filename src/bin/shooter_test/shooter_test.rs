//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::prelude::*,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::{CursorGrabMode, PresentMode, WindowLevel, WindowTheme},
    render::{settings::*},
};
use bevy::render::RenderPlugin;
use iyes_perf_ui::prelude::*;
use rand::RngCore;

const PLAYER_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: (500., 300.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }).set(ImagePlugin::default_nearest()),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        )) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (animate_sprite, events, spawn_enemies, move_enemies),
        )
        .add_plugins(PerfUiPlugin)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy {
    health: u32,
}

trait Update {
    fn update() -> Self;
}
impl Update for Enemy {
    fn update() -> Self {
        Self { health: 100 }
    }

}
fn events(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        player.translation.y += PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        player.translation.y -= PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        player.translation.x -= PLAYER_SPEED * time.delta_seconds();
        player.scale.x = -3.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        player.translation.x += PLAYER_SPEED * time.delta_seconds();
        player.scale.x = 3.0;
    }

    // // Calculate the new horizontal paddle position based on player input
    // let new_paddle_position =
    //     paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();
    //
    // // Update the paddle position,
    // // making sure it doesn't cause the paddle to leave the arena
    // let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    // let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;
    //
    // paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    // for (indices, mut timer, mut atlas) in &mut query {
    //     timer.tick(time.delta());
    //     if timer.just_finished() {
    //         atlas.index = if atlas.index == indices.last {
    //             indices.first
    //         } else {
    //             atlas.index + 1
    //         };
    //     }
    // }
}

fn spawn_enemies(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        let mut number = rng.next_u64();
        let shape = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                    0.0, 0.0, 0.0,
                ),

                ..default()
            },
            Enemy { health: 100 },
        ));
    }
}

fn move_enemies(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Enemy), Without<Player>>,
    mut player_query: Query<(&mut Transform, &Player)>,
) {
    let player = player_query.single_mut();
    let player_position = player.0.translation;

    for (mut transform, enemy) in &mut query {
        let mut direction: Vec3 = (player_position - transform.translation).normalize();

        transform.translation += direction * 50.0 * time.delta_seconds();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(3.0)),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
    ));



    commands.spawn((
        PerfUiRoot {
            display_labels: true,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
}
