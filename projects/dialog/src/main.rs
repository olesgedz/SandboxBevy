use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
    input::mouse::*,
    ui::RelativeCursorPosition,
};
use bevy::asset::io::memory::Data;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, relative_cursor_position_system)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Montserrat-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        ..default()
    };
    // 2d camera
    commands.spawn(Camera2dBundle::default());
    // Demonstrate changing translation

    // Demonstrate text wrapping
    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 42.0,
        ..default()
    };
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(Unicode linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    justify: JustifyText::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("characters/alice/Alice_Default.png"),
        transform: Transform::from_translation(Vec3::new(500.0, -10.0, 1.0))
            .with_scale(Vec3::splat(0.25)),
        ..default()
    });


    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(250.),
                        height: Val::Px(250.),
                        margin: UiRect::bottom(Val::Px(15.)),
                        ..default()
                    },
                    background_color: Color::srgb(235., 35., 12.).into(),
                    ..default()
                })
                .insert(RelativeCursorPosition::default());

            parent.spawn(TextBundle {
                text: Text::from_section(
                    "(0.0, 0.0)",
                    TextStyle {
                        font: asset_server.load("fonts/Montserrat-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                    },
                ),
                ..default()
            });
        });
}

fn relative_cursor_position_system(
    relative_cursor_position_query: Query<&RelativeCursorPosition>,
    mut output_query: Query<&mut Text>,
) {
    let relative_cursor_position = relative_cursor_position_query.single();
}