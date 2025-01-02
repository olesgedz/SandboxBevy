use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::render::prelude::*;

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
    let mut data = Some(msa.unwrap());
    if data.is_some() {
        println!("MSa value : {:?} ", data.unwrap().samples);
    }
}
