use bevy::{
    prelude::*,
    sprite::*,
    input::mouse::MouseMotion,
    window::PrimaryWindow,
};
use std::f32::consts::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::texture::BevyDefault;
#[derive(Debug, Component)]
struct Card;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_motion)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Card)
        .insert(
            SpriteBundle {
                texture: asset_server.load("card.png"),
                sprite: Sprite {
                    ..Default::default()
                },
                ..Default::default()
            });
}

fn mouse_motion(
    mut cards: Query<&mut Transform, With<Card>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mut card_transform = cards.single_mut();
    let window = q_windows.single();
    if let Some(position) = q_windows.single().cursor_position() {
        card_transform.translation = Vec3::new(position.x - window.resolution.size().x / 2.0,
                                               -position.y + window.resolution.size().y / 2.0, 0.0);
    }
}