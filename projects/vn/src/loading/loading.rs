use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::tasks::ConditionalSendFuture;
use std::io::Error;

/// A simple string asset type
#[derive(Asset, TypePath, Debug)]
pub struct StringAsset(pub String);

/// Custom loader that just loads the file contents as a String
#[derive(Default)]
pub struct StringAssetLoader;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Level,
}

impl AssetLoader for StringAssetLoader {
    type Asset = StringAsset;
    type Settings = ();
    type Error = Error;
    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut contents = String::new();
            reader.read_to_string(&mut contents).await?;
            Ok(StringAsset(contents))
        })
    }
    fn extensions(&self) -> &[&str] {
        &["json", "txt"]
    }
}

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .init_asset::<StringAsset>()
//         .init_asset_loader::<StringAssetLoader>()
//         .init_state::<AppState>()
//         .add_systems(Startup, load_json)
//         .add_systems(Startup, check_loaded.run_if(in_state(AppState::Loading)))
//         .run();
// }

#[derive(Resource)]
struct JsonHandle(Handle<StringAsset>);

fn load_json(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<StringAsset> = asset_server.load("text/story.json");
    commands.insert_resource(JsonHandle(handle));
    // Print asset
}

fn check_loaded(
    json: Res<JsonHandle>,
    assets: Res<Assets<StringAsset>>,
    state: ResMut<NextState<AppState>>,
) {
    // if let Some(asset) = assets.get(&json.0) {
    //     info!("Loaded JSON string: {}", asset.0);
    //     state.set(AppState::Level);
    // }
}
