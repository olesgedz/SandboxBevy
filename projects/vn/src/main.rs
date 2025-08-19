use bevy::asset::AssetPath;
use bevy::ecs::error::HandleError;
use bevy::ecs::system::ParamSet;
use bevy::log::tracing_subscriber::fmt::writer::MakeWriterExt;
use bevy::prelude::*;
use rand::TryRngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::process::exit;

// Game states
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
}

// Story data structures
#[derive(Serialize, Deserialize, Clone)]
struct DialogueLine {
    character: String,
    text: String,
    character_sprite: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Story {
    title: String,
    lines: Vec<DialogueLine>,
}

// Components
#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct DialogueUI;

#[derive(Component)]
struct DialogueText;

#[derive(Component)]
struct CharacterName;

#[derive(Component)]
struct CharacterSprite;

#[derive(Component)]
struct CharacterSpriteImage(Handle<Image>);

#[derive(Component)]
struct Test;

// Resources
#[derive(Resource)]
struct StoryData {
    story: Story,
    current_line: usize,
}

#[derive(Resource)]
struct GameAssets {
    character_sprites: HashMap<String, Handle<Image>>,
}

#[derive(Resource, Default)]
struct TypewriterState {
    full_text: String,
    visible_chars: usize,
    is_animating: bool,
    chars_per_second: f32,
    char_progress_accum: f32,
}

// Events
#[derive(Event)]
struct NextDialogue;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Turn - Visual Novel".into(),
                resolution: (800., 600.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_event::<NextDialogue>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(GameState::Playing), spawn_dialogue_ui)
        .add_systems(OnEnter(GameState::Playing), start_first_dialogue)
        .add_systems(OnExit(GameState::Playing), despawn_dialogue_ui)
        .add_systems(Update, handle_input.run_if(in_state(GameState::Playing)))
        .add_systems(Update, update_dialogue.run_if(in_state(GameState::Playing)))
        .add_systems(
            Update,
            animate_typewriter.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            handle_menu_input.run_if(in_state(GameState::MainMenu)),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);

    // print current working directory
    println!("Current working directory: {:?}", std::env::current_dir());
    // Load story from JSON file
    //get bevy assets path

    let a = AssetPath::path;
    println!("Asset path: {:?}", AssetPath::default());
    let story_json = std::fs::read_to_string("path").expect("Failed to read story.json");

    // asset_server.load("story .json");

    let story: Story = serde_json::from_str(&story_json).expect("Failed to parse story.json");

    println!("Loaded story: {}", story.title);
    println!("Total dialogue lines: {}", story.lines.len());

    // Load character sprites
    let mut character_sprites = HashMap::new();

    // Load Phoenix sprites
    character_sprites.insert(
        "phoenix".to_string(),
        asset_server.load("sprites/phoenix/talking/frame_0_delay-0.2s.png"),
    );

    character_sprites.insert(
        "phoenix_talking".to_string(),
        asset_server.load("sprites/phoenix/talking/frame_1_delay-0.2s.png"),
    );
    character_sprites.insert(
        "phoenix_confident".to_string(),
        asset_server.load("sprites/phoenix/confident-talking/frame_0.png"),
    );
    // Load Trucy sprites
    character_sprites.insert(
        "trucy".to_string(),
        asset_server.load("sprites/trucy/talking/frame_00.png"),
    );
    character_sprites.insert(
        "trucy_talking".to_string(),
        asset_server.load("sprites/trucy/talking/frame_01.png"),
    );
    character_sprites.insert(
        "trucy_confident".to_string(),
        asset_server.load("sprites/trucy/confident-talking/frame_0.png"),
    );

    // Load a default sprite for characters without specific sprites
    character_sprites.insert(
        "default".to_string(),
        asset_server.load("sprites/phoenix/talking/frame_0_delay-0.2s.png"),
    );

    commands.insert_resource(StoryData {
        story,
        current_line: 0,
    });
    commands.insert_resource(GameAssets { character_sprites });
    commands.insert_resource(TypewriterState {
        full_text: String::new(),
        visible_chars: 0,
        is_animating: false,
        chars_per_second: 120.0,
        char_progress_accum: 0.0,
    });
}

fn spawn_main_menu(mut commands: Commands) {
    // Main menu title
    commands.spawn((
        Text::new("The Mysterious Letter"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(50.0),
            ..default()
        },
        MainMenuUI,
    ));

    // Start button text
    commands.spawn((
        Text::new("Press SPACE to start"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(50.0),
            ..default()
        },
        MainMenuUI,
    ));
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn spawn_dialogue_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(200.0, 300.0)),
            ..default()
        },
        Transform::from_xyz(-250.0, 50.0, 1.0),
        CharacterSprite,
        CharacterSpriteImage(Handle::default()),
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/phoenix/talking/frame_0_delay-0.2s.png"),
            ..default()
        },
        Test,
        Transform::from_xyz(-150.0, 50.0, 1.0),
    ));

    // Character name
    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(400.0),
            left: Val::Px(50.0),
            ..default()
        },
        CharacterName,
    ));

    // Dialogue text
    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(450.0),
            left: Val::Px(50.0),
            right: Val::Px(50.0),
            ..default()
        },
        DialogueText,
    ));

    // Continue instruction
    commands.spawn((
        Text::new("Press SPACE to continue"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            right: Val::Px(50.0),
            ..default()
        },
        DialogueUI,
    ));
}

fn despawn_dialogue_ui(
    mut commands: Commands,
    query: Query<Entity, Or<(With<DialogueUI>, With<CharacterName>, With<CharacterSprite>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn handle_menu_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Playing);
    }
}

fn handle_input(
    mut next_dialogue_events: EventWriter<NextDialogue>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    typewriter: Res<TypewriterState>,
) {
    // Handle keyboard input
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!(
            "Space pressed, typewriter animating: {}",
            typewriter.is_animating
        );
        if !typewriter.is_animating {
            // Only advance to next line if current line is fully displayed
            println!("Advancing to next line");
            next_dialogue_events.write(NextDialogue);
        } else {
            println!("Still animating, can't advance yet");
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::MainMenu);
    }
}

fn update_dialogue(
    mut next_dialogue_events: EventReader<NextDialogue>,
    mut story_data: ResMut<StoryData>,
    mut queries: ParamSet<(
        Query<&mut Text, With<CharacterName>>,
        Query<&mut Text, With<DialogueText>>,
    )>,
    mut typewriter: ResMut<TypewriterState>,
    mut sprite_query: Query<&mut CharacterSpriteImage>,
    mut test_query: Query<&mut Sprite, With<Test>>,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    for _ in next_dialogue_events.read() {
        println!(
            "NextDialogue event received, current_line: {}",
            story_data.current_line
        );
        if story_data.current_line < story_data.story.lines.len() {
            let line = &story_data.story.lines[story_data.current_line];
            println!("Showing line: {} - {}", line.character, line.text);

            // Update character name
            if let Ok(mut name_text) = queries.p0().single_mut() {
                name_text.0 = line.character.clone();
            }

            // Update character sprite - temporarily disabled
            // TODO: Implement proper sprite rendering for Bevy 0.16.1
            // if let Ok(mut sprite) = sprite_query.single_mut() {
            //     sprite.0 =
            // }

            test_query.single_mut().unwrap().image = game_assets
                .character_sprites
                .get("trucy_talking")
                .cloned()
                .expect("REASON");
            // Initialize typewriter for this line
            typewriter.full_text = line.text.clone();
            typewriter.visible_chars = 0;
            typewriter.is_animating = true;
            typewriter.char_progress_accum = 0.0;
            // Clear the dialogue text shown for now
            if let Ok(mut text) = queries.p1().single_mut() {
                text.0.clear();
            }
        } else {
            println!("No more lines to show!");
        }
    }
}

fn start_first_dialogue(
    mut next_dialogue_events: EventWriter<NextDialogue>,
    mut story_data: ResMut<StoryData>,
) {
    // Ensure we start from the first line when entering Playing
    story_data.current_line = 0;
    next_dialogue_events.write(NextDialogue);
}

fn animate_typewriter(
    time: Res<Time>,
    mut typewriter: ResMut<TypewriterState>,
    mut dialogue_text_query: Query<&mut Text, With<DialogueText>>,
    mut story_data: ResMut<StoryData>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !typewriter.is_animating {
        return;
    }

    let delta = time.delta_secs();
    let total_chars = typewriter.full_text.chars().count();
    let speed = if keyboard_input.pressed(KeyCode::ShiftLeft)
        || keyboard_input.pressed(KeyCode::ShiftRight)
    {
        typewriter.chars_per_second * 3.0
    } else {
        typewriter.chars_per_second
    };
    typewriter.char_progress_accum += speed * delta;
    let add = typewriter.char_progress_accum.floor() as usize;
    typewriter.char_progress_accum -= add as f32;

    // Uncomment if you still need verbose debugging
    // println!("Animating: delta={:.3}s, total_chars={}, visible_chars={}, add={}, accum={:.2}",
    //          delta, total_chars, typewriter.visible_chars, add, typewriter.char_progress_accum);

    if add > 0 {
        typewriter.visible_chars = (typewriter.visible_chars + add).min(total_chars);
    }

    let partial: String = typewriter
        .full_text
        .chars()
        .take(typewriter.visible_chars)
        .collect();

    // println!("Partial text: '{}'", partial);

    if let Ok(mut text) = dialogue_text_query.single_mut() {
        text.0 = partial;
    }

    if typewriter.visible_chars >= total_chars {
        typewriter.is_animating = false;
        // println!("Line complete, advancing to next line. Current: {}", story_data.current_line);
        // Line is complete, advance to next line
        story_data.current_line += 1;
        // println!("Advanced to line: {}", story_data.current_line);
    }
}
