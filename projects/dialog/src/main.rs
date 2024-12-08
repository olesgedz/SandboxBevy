use bevy::asset::io::memory::Data;
use bevy::{
    input::mouse::*,
    prelude::*,
    ui::RelativeCursorPosition,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(Update, relative_cursor_position_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Montserrat-Bold.ttf");
    let text_style = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    // 2d camera
    commands.spawn(Camera2d::default());
    // Demonstrate changing translation

    // // Demonstrate text wrapping
    let slightly_smaller_text_style = TextFont {
        font,
        font_size: 42.0,
        ..default()
    };
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn((
                Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), box_position),
                Transform::from_translation(box_position.extend(0.0)),
            ))
        .with_children(|builder| {
            builder.spawn((Text2d("hjhjhjh".parse().unwrap())));
        });
    // 
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("characters/alice/Alice_Default.png"),
    //     transform: Transform::from_translation(Vec3::new(500.0, -10.0, 1.0))
    //         .with_scale(Vec3::splat(0.25)),
    //     ..default()
    // });
    // 
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             width: Val::Percent(100.),
    //             height: Val::Percent(100.0),
    //             align_items: AlignItems::Center,
    //             justify_content: JustifyContent::Center,
    //             flex_direction: FlexDirection::Column,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent
    //             .spawn(NodeBundle {
    //                 style: Style {
    //                     width: Val::Px(250.),
    //                     height: Val::Px(250.),
    //                     margin: UiRect::bottom(Val::Px(15.)),
    //                     ..default()
    //                 },
    //                 background_color: Color::srgb(235., 35., 12.).into(),
    //                 ..default()
    //             })
    //             .insert(RelativeCursorPosition::default());
    // 
    //         parent.spawn(TextBundle {
    //             text: Text::from_section(
    //                 "(0.0, 0.0)",
    //                 TextStyle {
    //                     font: asset_server.load("fonts/Montserrat-Bold.ttf"),
    //                     font_size: 40.0,
    //                     color: Color::srgb(0.9, 0.9, 0.9),
    //                 },
    //             ),
    //             ..default()
    //         });
    //     });
}

fn relative_cursor_position_system(
    relative_cursor_position_query: Query<&RelativeCursorPosition>,
    mut output_query: Query<&mut Text>,
) {
    let relative_cursor_position = relative_cursor_position_query.single();
}
