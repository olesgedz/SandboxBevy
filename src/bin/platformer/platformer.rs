// use avian2d::prelude::*;
// use bevy::{prelude::*, sprite::*, window::WindowResolution};
// 
// const WINDOW_WIDTH: f32 = 1024.0;
// const WINDOW_HEIGHT: f32 = 720.0;
// 
// const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
// const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
// 
// const COLOR_BACKGROUND: Color = Color::rgb(0.13, 0.13, 0.23);
// const COLOR_PLATFORM: Color = Color::rgb(0.29, 0.31, 0.41);
// const COLOR_PLAYER: Color = Color::rgb(0.60, 0.55, 0.60);
// 
// fn main() {
//     App::new()
//         .insert_resource(ClearColor(COLOR_BACKGROUND))
//         .add_plugins(DefaultPlugins.set(WindowPlugin {
//             primary_window: Some(Window {
//                 title: "Bevy Platformer".to_string(),
//                 resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
//                 resizable: true,
//                 ..Default::default()
//             }),
//             ..Default::default()
//         }))
//         .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
//         .add_systems(Startup, setup)
//         .run();
// }
// 
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: COLOR_PLATFORM,
//             ..Default::default()
//         },
//         transform: Transform {
//             translation: Vec3::new(-100.0, WINDOW_BOTTOM_Y + (200.0 / 2.0), 0.0),
//             scale: Vec3::new(75.0, 200.0, 1.0),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// 
//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: COLOR_PLATFORM,
//             ..Default::default()
//         },
//         transform: Transform {
//             translation: Vec3::new(100.0, WINDOW_BOTTOM_Y + (350.0 / 2.0), 0.0),
//             scale: Vec3::new(50.0, 350.0, 1.0),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// 
//     commands.spawn((
//         Sprite {
//             color: COLOR_PLATFORM,
//             ..Default::default()
//         },
//         Transform {
//             translation: Vec3::new(350.0, WINDOW_BOTTOM_Y + (250.0 / 2.0), 0.0),
//             scale: Vec3::new(150.0, 250.0, 1.0),
//             ..Default::default()
//         },
//     ));
// 
//     commands.spawn(Camera2d::default());
// 
//     commands.spawn((
//         Mesh2d( meshes.add(Circle::new(10.0))),
//         // RigidBody::Static,
//         // Collider::circle(10.0),
//     ));
// 
//     // MeshMaterial2d (
//     //     materials.add(ColorMaterial::from(COLOR_PLAYER)),
//     //     transform: Transform {
//     //         translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
//     //         scale: Vec3::new(1.0, 1.0, 1.0),
//     //         ..Default::default()
//     //     },
//     //     ..default()
//     // ),
//     // RigidBody::Static,
//     // Collider::circle(10.0),
//     // ));
// }
