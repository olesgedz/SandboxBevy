use avian3d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::ecs::event::ManualEventReader;
use bevy::input::mouse::*;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::{CursorGrabMode, PrimaryWindow};

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}


#[derive(Debug, Component)]
pub struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Debug, Component)]
pub struct WorldModelCamera;

const PLAYER_SPEED: f32 = 10.0;


/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

pub fn spawn_player(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<StandardMaterial>>) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    let mut player = (
        RigidBody::Static,
        // Mass::default(),
        GravityScale(1.0),
        Collider::cylinder(1.0, 5.2),
        Player,
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            ..default()
        },
    );


    commands
        .spawn(player)
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3dBundle {
                    projection: PerspectiveProjection {
                        fov: 90.0_f32.to_radians(),
                        ..default()
                    }
                        .into(),
                    ..default()
                },
            ));
            // parent.spawn((
            //     Camera3dBundle {
            //         camera: Camera {
            //             // Bump the order to render on top of the world model.
            //             order: 1,
            //             ..default()
            //         },
            //         projection: PerspectiveProjection {
            //             fov: 70.0_f32.to_radians(),
            //             ..default()
            //         }
            //             .into(),
            //         ..default()
            //     },
            //     // Only render objects belonging to the view model.
            //     RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            // ));

            // Spawn the player's right arm.
            parent.spawn((
                MaterialMeshBundle {
                    mesh: arm,
                    material: arm_material,
                    transform: Transform::from_xyz(0.2, -0.1, -0.25),
                    ..default()
                },
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ));
        });
}


pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<MovementSettings>,
    mut player: Query<&mut Transform, With<Player>>,
    mut cam: Query<&mut Transform, (With<WorldModelCamera>, Without<Player>)>,
) {
    let mut transform = player.single_mut();
    let mut cam_transform = cam.single_mut();
    let mut direction: Vec3 = Vec3::ZERO;

    // forward
    if keys.pressed(KeyCode::KeyW) {
        direction += cam_transform.forward().as_vec3();
    }

    // back
    if keys.pressed(KeyCode::KeyS) {
        direction += cam_transform.back().as_vec3();
    }

    // left
    if keys.pressed(KeyCode::KeyA) {
        direction += cam_transform.left().as_vec3();
    }

    // right
    if keys.pressed(KeyCode::KeyD) {
        direction += cam_transform.right().as_vec3();
    }

    if keys.pressed(KeyCode::KeyZ) {
        direction += cam_transform.up().as_vec3();
    }

    if keys.pressed(KeyCode::KeyX) {
        direction += cam_transform.down().as_vec3();
    }


    direction = direction.normalize_or_zero();
    // println!("direction : {:?}", direction);
    let velocity = direction * settings.speed * time.delta_seconds();

    transform.translation += velocity;
}


pub fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<WorldModelCamera>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_look, move_player));
    }
}