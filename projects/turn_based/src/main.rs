use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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
    commands.spawn(Camera2d);

    // cube
    commands.spawn((
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));

    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));

    commands.spawn((
        Mesh3d(floor),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.5, 0.0),
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