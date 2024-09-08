use avian3d::prelude::*;
use bevy::prelude::*;
use crate::player::PlayerPlugin;

mod input;
mod setup;

mod player;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
                      PhysicsPlugins::default(),   PhysicsDebugPlugin::default(), PlayerPlugin
        ))
        .add_systems(Startup, (setup::setup, input::cursor_grab))
        .add_systems(Update, (input::exit_on_esc))
        .run();
}

