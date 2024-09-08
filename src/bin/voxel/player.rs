use avian3d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::ecs::event::ManualEventReader;
use bevy::input::mouse::*;
use bevy::math::vec3;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::debug::debug::*;
use crate::input::CursorState;

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

#[derive(Debug, Component)]
pub struct Bullet {
    pub life: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            life: 5.0,
        }
    }
}

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
            speed: 250.,
        }
    }
}

pub fn spawn_player(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<StandardMaterial>>) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    let mut player = (
        RigidBody::Dynamic,
        Mass(90.0),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::ZERO,
        Collider::cuboid(3.0, 3.2, 3.0),
        // Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        Friction::new(1.0),
        GravityScale(2.0),
        Player,
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 1.759766, 8.447795),
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
                        fov: 70.0_f32.to_radians(),
                        ..default()
                    }
                        .into(),
                    ..default()
                },
            ));
            parent.spawn((
                Camera3dBundle {
                    camera: Camera {
                        // Bump the order to render on top of the world model.
                        order: 1,
                        ..default()
                    },
                    projection: PerspectiveProjection {
                        fov: 70.0_f32.to_radians(),
                        ..default()
                    }
                        .into(),
                    ..default()
                },
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

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


pub fn player_shoot(
    mut commands: Commands,
    mut player: Query<&mut Transform, With<Player>>,
    mut cam: Query<&mut Transform, (With<WorldModelCamera>, Without<Player>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cursor_state: ResMut<CursorState>,
) {
    let mut cam_transform = cam.single_mut();
    let mut player_transform = player.single_mut();

    if cursor_state.lock {
        for &key in buttons.get_pressed() {
            if key == MouseButton::Left {
                spawn_bullet(&mut commands, &mut meshes, &mut materials, &player_transform, &cam_transform);
            }

            if key == MouseButton::Right {
                spawn_block(&mut commands, &mut meshes, &mut materials, &player_transform, &cam_transform);
            }
        }
    }
}

fn spawn_bullet(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    player_transform: &Transform,
    cam_transform: &Transform,
) {
    let bullet = (
        Bullet::default(),
        RigidBody::Dynamic,
        Collider::cuboid(0.1, 0.1, 0.1),
        Mass(0.01),
        ShowAxes,
        GravityScale(1.0),
        ExternalForce::new(cam_transform.forward().as_vec3() * 20.0).with_persistence(false),
        Friction::new(100.0),
        AngularDamping(0.5),
        LinearDamping(3.0),
        MaterialMeshBundle {
            mesh: meshes.add(Sphere::new(0.1)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_matrix(Mat4::from_translation(player_transform.translation + cam_transform.forward().as_vec3() * 2.0)),
            ..default()
        }
    );

    commands.spawn(bullet);
}

fn spawn_block(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    player_transform: &Transform,
    cam_transform: &Transform,
) {
    let block_side = 1.0;

    let block = ((
        RigidBody::Dynamic,
        Collider::cuboid(block_side, block_side, block_side),
        // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Friction::new(0.4),
        Mass(90.0),
        ShowAxes,
        GravityScale(1.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_matrix(Mat4::from_translation(player_transform.translation + cam_transform.forward().as_vec3() * 3.0)),
            ..default()
        }
    ));

    commands.spawn(block);
}


pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<MovementSettings>,
    mut player: Query<&mut Transform, With<Player>>,
    mut cam: Query<&mut Transform, (With<WorldModelCamera>, Without<Player>)>,
    mut rigid_body: Query<&mut LinearVelocity, With<Player>>,
) {
    let mut transform = player.single_mut();
    let mut cam_transform = cam.single_mut();
    let mut rb = rigid_body.single_mut();
    let mut direction: Vec3 = Vec3::ZERO;

    for &key in keys.get_pressed() {
        // forward
        if key == KeyCode::KeyW {
            direction += cam_transform.forward().as_vec3();
        }

        // back
        if key == KeyCode::KeyS {
            direction += cam_transform.back().as_vec3();
        }

        // left
        if key == KeyCode::KeyA {
            direction += cam_transform.left().as_vec3();
        }

        // right
        if key == KeyCode::KeyD {
            direction += cam_transform.right().as_vec3();
        }

        if key == KeyCode::KeyZ {
            direction += transform.up().as_vec3();
            rb.y =  direction.y * settings.speed * time.delta_seconds();
        }

        if key == KeyCode::KeyX {
            direction += transform.down().as_vec3();
        }

        if key == KeyCode::Space && rb.y <= 0.0  {
            direction += transform.up().as_vec3();
            rb.y +=  direction.y * 1000.0 * time.delta_seconds();
        }


        direction = direction.normalize_or_zero();
        // println!("direction : {:?}", direction);
        let velocity = direction * settings.speed * time.delta_seconds();

        // transform.translation += velocity;
        rb.x =  direction.x * settings.speed * time.delta_seconds();
        rb.z =  direction.z * settings.speed * time.delta_seconds();
        println!("velocity : {:?}",  rb.0);

    }

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
            .add_systems(Update, (player_look, move_player, player_shoot));
    }
}