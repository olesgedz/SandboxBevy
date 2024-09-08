use avian3d::prelude::*;
use bevy::math::*;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let base_size = 100.0;
    //  base
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(base_size, 3.0, base_size),
        Friction::new(0.4),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(base_size, 3.0, base_size)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }));
    // cube
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        GravityScale(1.0),
        Friction::new(0.4),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        }));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });


    // player
}