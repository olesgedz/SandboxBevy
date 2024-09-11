use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin};
use crate::player::PlayerPlugin;

mod input;
mod debug;
mod player;
mod setup;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
                      PhysicsPlugins::default(),   PhysicsDebugPlugin::default(), PlayerPlugin
        ))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup::setup, input::cursor_grab))
        .add_systems(Update, (ui::ui_example_system, input::exit_on_esc, debug::debug::draw_axes))
        .run();
}