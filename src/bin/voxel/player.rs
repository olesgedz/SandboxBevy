use avian3d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::input::mouse::*;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Debug, Component)]
struct WorldModelCamera;

const PLAYER_SPEED: f32 = 10.0;


/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;


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


pub fn move_player(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    let mut transform = player.single_mut();


    // for motion in mouse_motion.read() {
    //     let yaw = -motion.delta.x * 0.003;
    //     let pitch = -motion.delta.y * 0.002;
    //     // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
    //     transform.rotate_y(yaw);
    //     println!("rad : {}", f32::to_degrees(transform.rotation.y));
    //     if (transform.rotation.y >= f32::to_radians(90.0)) {
    //         transform.rotation.y = f32::to_radians(90.0);
    //     }
    //     transform.rotate_local_x(pitch);
    // }


    // let mut rotation = transform.rotation;
    //
    // let mut forward: Vec3 = transform.forward().as_vec3();
    // let mut right: Vec3 = transform.right().as_vec3();
    // let mut up: Vec3 = forward.cross(right).normalize();
    // let mut direction: Vec3 = Vec3::ZERO;


    // for (mut player_transform, player_speed) in player_q.iter_mut() {
    //     let cam = match cam_q.get_single() {
    //         Ok(c) => c,
    //         Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
    //     };
    //
    //     let mut direction = Vec3::ZERO;
    //
    //     // forward
    //     if keys.pressed(KeyCode::KeyW) {
    //         direction += *cam.forward();
    //     }
    //
    //     // back
    //     if keys.pressed(KeyCode::KeyS) {
    //         direction += *cam.back();
    //     }
    //
    //     // left
    //     if keys.pressed(KeyCode::KeyA) {
    //         direction += *cam.left();
    //     }
    //
    //     // right
    //     if keys.pressed(KeyCode::KeyD) {
    //         direction += *cam.right();
    //     }
    //
    //     direction.y = 0.0;
    //     let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
    //     player_transform.translation += movement;
    // }
}