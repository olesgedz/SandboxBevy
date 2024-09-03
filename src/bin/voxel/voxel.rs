use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::input::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, fly_camera_system)
        .run();
}

#[derive(Component)]
struct FlyCamera {
    speed: f32,
    sensitivity: f32,
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Camera with FlyCamera component
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCamera {
            speed: 5.0,
            sensitivity: 0.1,
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Create a grid of voxels
    let voxel_size = 1.0;
    let grid_size = 5;

    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                // commands.spawn(PbrBundle {
                //     mesh: meshes.add(mesh_fromCuboid::default(),
                //     material: materials.add(StandardMaterial {
                //         base_color: Color::rgb(
                //             (x as f32) / (grid_size as f32),
                //             (y as f32) / (grid_size as f32),
                //             (z as f32) / (grid_size as f32),
                //         ),
                //         ..Default::default()
                //     }),
                //     transform: Transform::from_xyz(
                //         x as f32 * voxel_size,
                //         y as f32 * voxel_size,
                //         z as f32 * voxel_size,
                //     ),
                //     ..Default::default()
                // });

                commands.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: materials.add(Color::srgb_u8(124, 144, 255)),
                    transform: Transform::from_xyz(
                        x as f32 * voxel_size,
                        y as f32 * voxel_size,
                        z as f32 * voxel_size,),
                    ..default()
                });
            }
        }
    }
}

fn fly_camera_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&FlyCamera, &mut Transform)>,
) {
    let (fly_camera, mut transform) = query.single_mut();
    let mut direction = Vec3::ZERO;

    // Keyboard movement
    // if keyboard_input.pressed(KeyCode::KeyW) {
    //     direction += transform.forward();
    // }
    // if keyboard_input.pressed(KeyCode::KeyS) {
    //     direction -= transform.forward();
    // }
    // if keyboard_input.pressed(KeyCode::KeyA) {
    //     direction -= transform.right();
    // }
    // if keyboard_input.pressed(KeyCode::KeyD) {
    //     direction += transform.right();
    // }
    // if keyboard_input.pressed(KeyCode::Space) {
    //     direction += transform.up();
    // }
    // if keyboard_input.pressed(KeyCode::ShiftLeft) {
    //     direction -= transform.up();
    // }

    // Apply movement
    transform.translation += time.delta_seconds() * fly_camera.speed * direction;

    // Mouse look
    // if mouse_input.pressed(MouseButton::Right) {
    //     for event in mouse_motion_events {
    //         let delta = event.delta * fly_camera.sensitivity;
    //         transform.rotate(Quat::from_rotation_y(-delta.x.to_radians()));
    //         transform.rotate_local_x(-delta.y.to_radians());
    //     }
    // }
}
