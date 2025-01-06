use bevy::asset::io::memory::Data;
use bevy::{
    input::mouse::*,
    prelude::*,
    text::{LineBreak, TextBounds},
    ui::RelativeCursorPosition,
};
use std::fmt::Debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse_motion, relative_cursor_position_system))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Montserrat-Bold.ttf");

    // 2d camera
    commands.spawn(Camera2d::default());
    // Demonstrate changing translation

    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), box_position),
            Transform::from_translation(box_position.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn(Text2d("Data".parse().unwrap()));
        });

    commands
        .spawn((
            Sprite::from_image(asset_server.load("characters/alice/Alice_Default.png")),
            Transform::from_translation(Vec3::new(500.0, -10.0, 1.0)).with_scale(Vec3::splat(0.25)),
        ))
        .observe(recolor_on::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
        .observe(recolor_on::<Pointer<Out>>(Color::BLACK))
        .observe(recolor_on::<Pointer<Down>>(Color::srgb(1.0, 1.0, 0.0)))
        .observe(recolor_on::<Pointer<Up>>(Color::srgb(0.0, 1.0, 1.0)))
        .observe(
            |out: Trigger<Pointer<Down>>, mut texts: Query<&mut Text2d>| {
                let mut text = texts.get_single_mut().unwrap();
                text.0 = "Down".parse().unwrap();
                println!("Down {:?}", out);
            },
        )
        .observe(|out: Trigger<Pointer<Up>>, mut texts: Query<&mut Text2d>| {
            let mut text = texts.get_single_mut().unwrap();
            text.0 = "Up".parse().unwrap();
            println!("Up {:?}", out);
        });

    let other_box_size = Vec2::new(300.0, 100.0);
    let other_box_position = Vec2::new(50.0, -150.0);
    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 15.0,
        ..default()
    };
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.20, 0.3, 0.70), other_box_size),
            Transform::from_translation(other_box_position.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("this text wraps in the box\n(AnyCharacter linebreaks)"),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::AnyCharacter),
                // Wrap text in the rectangle
                TextBounds::from(other_box_size),
                // ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
            ));
        });

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(250.),
                        height: Val::Px(250.),
                        margin: UiRect::bottom(Val::Px(15.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(235., 35., 12.)),
                ))
                .insert(RelativeCursorPosition::default());

            parent.spawn((
                Text::new("(0.0, 0.0)"),
                TextFont {
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn recolor_on<E: Debug + Clone + Reflect>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.entity()) else {
            return;
        };
        sprite.color = color;
    }
}

fn mouse_motion(mut evr_motion: EventReader<MouseMotion>) {
    for ev in evr_motion.read() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}
fn relative_cursor_position_system(
    relative_cursor_position: Single<&RelativeCursorPosition>,
    output_query: Single<(&mut Text, &mut TextColor)>,
) {
    let (mut output, mut text_color) = output_query.into_inner();

    **output = if let Some(relative_cursor_position) = relative_cursor_position.normalized {
        format!(
            "({:.1}, {:.1})",
            relative_cursor_position.x, relative_cursor_position.y
        )
    } else {
        "unknown".to_string()
    };

    text_color.0 = if relative_cursor_position.mouse_over() {
        Color::srgb(0.1, 0.9, 0.1)
    } else {
        Color::srgb(0.9, 0.1, 0.1)
    };
}
