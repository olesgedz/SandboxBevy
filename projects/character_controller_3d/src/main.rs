use avian3d::prelude::*;
use bevy::input::mouse::{AccumulatedMouseMotion, MouseMotion, MouseWheel};
use bevy::pbr::PbrPlugin;
use bevy::prelude::*;
use std::fmt::DebugList;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .add_systems(Update, camera_follow_system)
        .add_systems(Update, camera_control_system)
        .run();
}

#[derive(Component)]
struct Target;
#[derive(Component)]
struct ThirdPersonCamera {
    offset: Vec3,
    pitch: f32,
    yaw: f32,
    distance: f32,
    sensitivity: f32,
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_size = 100.0;
    //  base
    // camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_translation(Vec3::new(0.0, 5.0, 10.0))
    //         .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    // ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 5.0, -10.0)).looking_at(Vec3::ZERO, Vec3::Y),
        ThirdPersonCamera {
            offset: Vec3::new(0.0, 2.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            distance: 10.0,
            sensitivity: 0.1,
        },
    ));
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0).into(),
            ..Default::default()
        })),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 3.0, 0.0),
        Rotates,
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
        Target,
    ));

    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));

    commands.spawn((
        Mesh3d(floor),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Collider::cuboid(20.0, 0.2, 20.0),
        RigidBody::Kinematic,
    ));

    // light
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).rotate_x(90.0),
    ));
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y-axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.15 * time.delta_secs());
    }
}

fn camera_follow_system(
    target_query: Query<&Transform, With<Target>>,
    mut camera_query: Query<(&mut Transform, &ThirdPersonCamera), Without<Target>>,
) {
    if let Ok(target_transform) = target_query.get_single() {
        for (mut camera_transform, camera) in &mut camera_query {
            // Calculate the new camera position based on the target
            let direction = Vec3::new(
                camera.yaw.to_radians().cos() * camera.pitch.to_radians().cos(),
                camera.pitch.to_radians().sin(),
                camera.yaw.to_radians().sin() * camera.pitch.to_radians().cos(),
            );

            camera_transform.translation =
                target_transform.translation + camera.offset - direction * camera.distance;

            camera_transform.look_at(target_transform.translation + camera.offset, Vec3::Y);
        }
    }
}

fn camera_control_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    // accumulated_mouse_motion: AccumulatedMouseMotion,
    mut camera_query: Query<&mut ThirdPersonCamera>,
) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        // Handle mouse motion for pitch and yaw
        for event in mouse_motion_events.read() {
            camera.yaw += event.delta.x * camera.sensitivity;
            camera.pitch = (camera.pitch - event.delta.y * camera.sensitivity).clamp(-89.0, 89.0);
        }

        // // Handle mouse wheel for zoom
        for event in mouse_wheel_events.read() {
            camera.distance = (camera.distance - event.y * 0.5).clamp(2.0, 20.0);
        }
    }

    // for event in mouse_motion_events.read() {
    //     info!("{:?}", event);
    // }
}
