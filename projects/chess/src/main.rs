use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    picking::*,
    prelude::*,
    window::WindowTheme,
};

mod board;
mod pieces;

use board::*;
use pieces::*; // this use namespace

#[derive(Resource)]
struct Msaa {
    samples: u32,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".into(),
                name: Some("bevy.app".into()),
                // Tells Wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                // visible: false,
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, (setup, create_board, create_pieces))
        .add_systems(Update, (draw_mesh_intersections, color_squares))
        .add_plugins(MeshPickingPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(MeshPickingSettings {
            // Enable picking only for the right mouse button
            require_markers: true,
            ray_cast_visibility: RayCastVisibility::VisibleInView,
        })
        .run();
}

/// Set up a simple 3D scene
fn setup(mut commands: Commands) {
    //Camera
    commands.spawn((
        // mutable cause pub fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands
        Camera3d::default(),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        RayCastPickable,
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

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    // Add some materials
    let white_material = materials.add(Color::srgb(1., 0.8, 0.8));
    let black_material = materials.add(Color::srgb(0., 0.2, 0.2));

    spawn_rook(
        &mut commands,
        white_material.clone(),
        rook_handle.clone(),
        Vec3::new(0., 0., 0.),
    );
    spawn_knight(
        &mut commands,
        white_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(0., 0., 1.),
    );
    spawn_bishop(
        &mut commands,
        white_material.clone(),
        bishop_handle.clone(),
        Vec3::new(0., 0., 2.),
    );
    spawn_queen(
        &mut commands,
        white_material.clone(),
        queen_handle.clone(),
        Vec3::new(0., 0., 3.),
    );
    spawn_king(
        &mut commands,
        white_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        Vec3::new(0., 0., 4.),
    );
    spawn_bishop(
        &mut commands,
        white_material.clone(),
        bishop_handle.clone(),
        Vec3::new(0., 0., 5.),
    );
    spawn_knight(
        &mut commands,
        white_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(0., 0., 6.),
    );
    spawn_rook(
        &mut commands,
        white_material.clone(),
        rook_handle.clone(),
        Vec3::new(0., 0., 7.),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            white_material.clone(),
            pawn_handle.clone(),
            Vec3::new(1., 0., i as f32),
        );
    }

    spawn_rook(
        &mut commands,
        black_material.clone(),
        rook_handle.clone(),
        Vec3::new(7., 0., 0.),
    );
    spawn_knight(
        &mut commands,
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(7., 0., 1.),
    );
    spawn_bishop(
        &mut commands,
        black_material.clone(),
        bishop_handle.clone(),
        Vec3::new(7., 0., 2.),
    );
    spawn_queen(
        &mut commands,
        black_material.clone(),
        queen_handle.clone(),
        Vec3::new(7., 0., 3.),
    );
    spawn_king(
        &mut commands,
        black_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        Vec3::new(7., 0., 4.),
    );
    spawn_bishop(
        &mut commands,
        black_material.clone(),
        bishop_handle.clone(),
        Vec3::new(7., 0., 5.),
    );
    spawn_knight(
        &mut commands,
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(7., 0., 6.),
    );
    spawn_rook(
        &mut commands,
        black_material.clone(),
        rook_handle.clone(),
        Vec3::new(7., 0., 7.),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            black_material.clone(),
            pawn_handle.clone(),
            Vec3::new(6., 0., i as f32),
        );
    }
}

// https://rustic-chess.org/
// https://caballerocoll.com/blog/bevy-chess-tutorial/
