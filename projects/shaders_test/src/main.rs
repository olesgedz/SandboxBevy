use avian3d::prelude::*;
use bevy::prelude::*;
use std::fmt::DebugList;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
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
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 5.0, 10.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 3.0, 0.0),
        Rotates,
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
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
