use std::process::Command;
use bevy::ecs::event::ManualEventReader;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::*;


#[derive(Resource)]
pub struct CursorState {
    pub(crate) lock: bool
}

impl CursorState {
    pub fn default() -> Self {
        CursorState {
            lock: true
        }
    }
}

pub(crate) fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut command: Commands,
) {
    let mut primary_window = q_windows.single_mut();

    command.insert_resource(CursorState::default());
    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor.visible = false;
}

pub fn exit_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_state: ResMut<CursorState>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }

    if keyboard_input.just_pressed(KeyCode::KeyP) {
        let mut primary_window = q_windows.single_mut();

        if (cursor_state.lock) {
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
        } else {
            primary_window.cursor.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor.visible = false;
        }
        cursor_state.lock = !cursor_state.lock;
    }

}