use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DialogueState {
            full_text: "This is an example dialogue.".to_string(),
            visible_text: "".to_string(),
        })
        .insert_resource(LetterTimer(Timer::from_seconds(0.05, TimerMode::Repeating))) // Adjust speed here
        .add_systems(Startup, setup)
        .add_systems(Update, update_text)
        .run();
}

// A resource to hold the state of the dialogue
#[derive(Resource, Default)]
struct DialogueState {
    full_text: String,
    visible_text: String,
}

// A timer resource to control letter rendering speed
#[derive(Resource, Default)]
struct LetterTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Spawning the text entity
    commands.spawn((
        TextBundle::from_section(
            "", // Start with empty text
            TextStyle {
                font: asset_server.load("fonts/Montserrat-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        ),
            // .with_style(TextAlignment::Left),
        TextComponent,
    ));
}

// Marker component to find our text entity
#[derive(Component)]
struct TextComponent;

fn update_text(
    time: Res<Time>,
    mut timer: ResMut<LetterTimer>,
    mut state: ResMut<DialogueState>,
    mut query: Query<&mut Text, With<TextComponent>>,
) {
    // Update the timer
    if timer.0.tick(time.delta()).finished() {
        // Check if there are more letters to reveal
        if state.visible_text.len() < state.full_text.len() {
            // Append the next character to the visible text
            let next_char = state.full_text.chars().nth(state.visible_text.len()).unwrap();
            state.visible_text.push(next_char);
        }
    }

    // Update the Text component with the new visible text
    for mut text in query.iter_mut() {
        text.sections[0].value = state.visible_text.clone();
    }
}
