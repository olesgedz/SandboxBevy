use avian3d::prelude::*;
use bevy::math::vec3;
use bevy::prelude::*;

fn main() {
    App::new()
        // Enable physics
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .run();
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Static physics object with a collision shape
//     commands.spawn((
//         RigidBody::Static,
//         Collider::cylinder(4.0, 0.2),
//         PbrBundle {
//             mesh: meshes.add(Circle::new(4.0)),
//             material: materials.add(Color::WHITE),
//             ..default()
//         },
//     ));
//
//     commands.spawn((
//         RigidBody::Static,
//         Collider::cylinder(4.0, 0.2),
//         PbrBundle {
//             mesh: meshes.add(Circle::new(4.0)),
//             material: materials.add(Color::WHITE),
//             transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
//                 .with_translation(vec3(0.0, 0.0, 0.0)),
//             ..default()
//         }));
//     commands.spawn((
//         RigidBody::Dynamic,
//         Collider::cuboid(1.0, 1.0, 1.0),
//         // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
//         GravityScale(1.0),
//         PbrBundle {
//             mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
//             material: materials.add(Color::srgb_u8(124, 144, 255)),
//             transform: Transform::from_xyz(0.0, 2.0, 0.0),
//             ..default()
//         }));
//
//     // Dynamic physics object with a collision shape and initial angular velocity
//     commands.spawn((
//         RigidBody::Dynamic,
//         Collider::cuboid(1.0, 1.0, 1.0),
//         AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
//         PbrBundle {
//             mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
//             material: materials.add(Color::srgb_u8(124, 144, 255)),
//             transform: Transform::from_xyz(0.0, 4.0, 0.0),
//             ..default()
//         },
//     ));
//
//     // Light
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });
//
//     // Camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
//         ..default()
//     });
// }

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(5.0, 1.0, 5.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(5.0, 1.0, 5.0)),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
        ..default()
    });
}