use avian3d::prelude::*;
use bevy::prelude::*;

mod input;
mod setup;

mod player;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
                      PhysicsPlugins::default(),
        ))
        .add_systems(Startup, (setup::setup, input::cursor_grab, player::spawn_player))
        .add_systems(Update, (input::exit_on_esc))
        .run();
}

