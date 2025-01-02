use avian3d::prelude::*;
use bevy::{prelude::*, render::prelude::*};

#[derive(Resource)]
struct Msaa {
    samples: u32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(Msaa { samples: 4 })
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut msa: Option<Res<Msaa>>,
) {
    // Board
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(8.0)))),
        MeshMaterial3d(materials.add(Color::srgb(1., 0.9, 0.9))),
        Transform::from_translation(Vec3::new(4., 0., 4.)),
    ));

    //Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
    ));
}
