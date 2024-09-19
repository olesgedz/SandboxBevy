use std::slice::Windows;
use bevy::prelude::*;
use bevy::sprite::Anchor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LetterTimer(Timer::from_seconds(0.05, TimerMode::Repeating))) // Adjust speed here
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, update_text)
        .run();
}

// A resource to hold the state of the dialogue
struct DialogueState {
    full_text: String,
    visible_text: String,
}

// A timer resource to control letter rendering speed
#[derive(Resource, Default)]
struct LetterTimer(Timer);

// Marker component to find our text entity
#[derive(Component)]
struct TextComponent;

#[derive(Resource, Default)]
struct Dialogue {
    text: Vec<DialogueState>,
    current: usize,
}

impl Dialogue {
    fn next(&mut self) -> Option<&DialogueState> {
        if self.current < self.text.len() {
            let current_text = &self.text[self.current];
            self.current += 1;
            Some(current_text)
        } else {
            None
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(Dialogue {
        text: vec![
            DialogueState {
                full_text: "Hello, World!".to_string(),
                visible_text: "".to_string(),
            },
            DialogueState {
                full_text: "This is a test.".to_string(),
                visible_text: "".to_string(),
            },
        ],
        current: 0,
    });
}


fn setup_ui(mut commands: Commands, asset_serve: Res<AssetServer>, window: Query<&Window>, asset_server: Res<AssetServer>) {
    let window_display = window.single();

    let resolution = window_display.resolution.size();

    commands.spawn(SpriteBundle {
        texture: asset_serve.load("textures/clippy.png"),
        sprite: Sprite {
            // Flip the logo to the left
            ..default()
        },
        ..default()
    });

    let background =  SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.5, 0.5, 1.0, 0.02),
            custom_size: Some(Vec2::new(resolution.x - 20.0, resolution.y / 3.0 - 10.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -resolution.y / 3.0, 0.0)),
        ..default()
    };

    let text_field = TextBundle::from_section(
            "", // Start with empty text
            TextStyle {
                font: asset_server.load("fonts/Montserrat-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        );


    commands.spawn(NodeBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    }
    ).with_children(|parent| {

        parent.spawn(background);
        parent.spawn(text_field);
            // Spawning the text entity
            // Anchor::Center,
            // TextComponent,

    });

    // commands.spawn(background);
}



fn update_text(
    time: Res<Time>,
    mut timer: ResMut<LetterTimer>,
    mut state: ResMut<Dialogue>,
    mut query: Query<&mut Text, With<TextComponent>>,
) {
    // // Update the timer
    // if timer.0.tick(time.delta()).finished() {
    //     // Check if there are more letters to reveal
    //     if state.visible_text.len() < state.full_text.len() {
    //         // Append the next character to the visible text
    //         let next_char = state.full_text.chars().nth(state.visible_text.len()).unwrap();
    //         state.visible_text.push(next_char);
    //     }
    // }
    //
    // Update the Text component with the new visible text
    for mut text in query.iter_mut() {
        text.sections[0].value = "AAAAA".parse().unwrap(); //state.visible_text.clone();
    }
}
