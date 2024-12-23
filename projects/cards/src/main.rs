use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    window::PrimaryWindow,
};
#[derive(Debug, Component)]
struct Card;

const SHADER_ASSET_PATH: &str = "post_process.wgsl";

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_motion)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(Card)
        .insert(Sprite::from_image(asset_server.load("card.png")));
}

fn mouse_motion(
    mut cards: Query<&mut Transform, With<Card>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mut card_transform = cards.single_mut();
    let window = q_windows.single();
    if let Some(position) = q_windows.single().cursor_position() {
        card_transform.translation = Vec3::new(
            position.x - window.resolution.size().x / 2.0,
            -position.y + window.resolution.size().y / 2.0,
            0.0,
        );
    }
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
