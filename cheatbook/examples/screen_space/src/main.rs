use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

use bevy::window::PrimaryWindow;
/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .init_resource::<MyWorldCoords>()
        .add_plugins(DefaultPlugins)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, my_cursor_system)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn a camera to be able to see anything
    commands.spawn((Camera2d::default(), MainCamera));


    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiAllEntries::default());
}


fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let Ok((camera, camera_transform)) = q_camera.single() else { return; };
    // 
    // // There is only one primary window, so we can similarly get it from the query:
    // let window = q_window.single().unwrap();
    // let a = camera.viewport_to_world(camera_transform, window.cursor_position().unwrap());
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    // if let Some(world_position) = window.cursor_position()
    //     .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    //     .map(|ray| ray.origin.truncate())
    // {
    //     mycoords.0 = world_position;
    //     eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    // }
}