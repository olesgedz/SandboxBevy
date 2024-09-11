use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn ui_example_system(mut contexts: EguiContexts,
                         frame: Res<FrameCount>,
                         time: Res<Time>
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        ui.label(format!("FPS: {}", frame.0 as f32 / time.elapsed_seconds()));
        if ui.button("Click me").clicked() {
            println!("Button clicked!");
        }

    });
}