use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::render::view::RenderLayers;
use bevy::color::palettes::tailwind;
use bevy::pbr::NotShadowCaster;
use avian3d::prelude::*;
use bevy::math::vec3;

const PLAYER_SPEED: f32 = 10.0;


/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
                      PhysicsPlugins::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component)]
struct WorldModelCamera;
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    // circular base
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(4.0, 0.2),
       PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(vec3(0.0, 0.0, 0.0)),
        ..default()
    }));
    // cube
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        // AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        GravityScale(1.0),
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


    commands
        .spawn((
            RigidBody::Static,
            // Mass::default(),
            GravityScale(1.0),
            Collider::cylinder(1.0, 5.2),
            Player,
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 3.0, 0.0),
                ..default()
            },
        ))
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

fn move_player(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = player.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        transform.rotate_y(yaw);
        transform.rotate_local_x(pitch);
    }
    let mut forward: Dir3 = transform.forward();
    let mut right: Dir3 = transform.right();
    let mut up: Vec3 = forward.cross(right.as_vec3()).normalize();

    if input.pressed(KeyCode::KeyW) {
        transform.translation += forward * PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation -= forward * PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation -= right * PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation += right * PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::KeyZ) {
        transform.translation += up * PLAYER_SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::KeyX) {
        transform.translation -= up * PLAYER_SPEED * time.delta_seconds();
    }
}