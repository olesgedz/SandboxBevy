use bevy::prelude::*;
use bevy::ui::prelude::*;
use bevy::ui::*;
use rand::Rng;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D Vampire Survivors Clone".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(LastShotTimer(Timer::from_seconds(0.2, TimerMode::Once)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                spawn_enemy,
                enemy_chase_player,
                update_healthbar,
                handle_collisions,
                player_shooting,
                bullet_movement,
                bullet_enemy_collision,
                bullet_lifetime_system,
                update_hit_flash
            ),
        )
        .run();
}

static YELLOW_COLOR: Color = Color::srgb(0.9, 0.3, 0.9);
static WHITE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
static RED_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
static BLUE_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct HealthBarBackground;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct LastShotTimer(Timer);


#[derive(Component)]
struct Bullet {
    lifetime: Timer,
}

#[derive(Component)]
struct HitFlash {
    timer: Timer,
}

#[derive(Component)]
struct Velocity(Vec2);

fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d);

    // Player square (white)
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
        Health {
            current: 100.0,
            max: 100.0,
        },
    ));

    // Health bar background node
    commands.spawn((
        Node {
            width: Val::Px(400.0),
            height: Val::Px(10.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
        HealthBar,
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let speed = 200.0;
    transform.translation +=
        (direction.normalize_or_zero() * speed * time.delta_secs()).extend(0.0);
}

fn spawn_enemy(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-400.0..400.0);
        let y = rng.gen_range(-300.0..300.0);

        commands.spawn((
            Sprite {
                color: RED_COLOR,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy,
            Health {
                current: 20.0,
                max: 20.0,
            }
        ));
    }
}

fn update_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitFlash, &mut Sprite), With<Enemy>>,
) {
    for (entity, mut hit_flash, mut sprite) in query.iter_mut() {
        hit_flash.timer.tick(time.delta());

        if hit_flash.timer.finished() {
            // Reset color and remove component
            sprite.color = RED_COLOR;
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}

fn enemy_chase_player(
    time: Res<Time>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player.single();

    for mut enemy_transform in enemies.iter_mut() {
        let direction = (player_transform.translation - enemy_transform.translation).truncate();
        let speed = 100.0;
        let movement = direction.normalize_or_zero() * speed * time.delta_secs();
        enemy_transform.translation += movement.extend(0.0);
    }
}
fn update_healthbar(
    player_health: Query<&Health, With<Player>>,
    mut bar_query: Query<&mut Node, With<HealthBar>>,
) {
    let health = player_health.single();
    let mut bar_style = bar_query.single_mut();

    let percent = health.current / health.max;
    bar_style.width = Val::Percent((percent * 100.0).clamp(0.0, 100.0));
}

fn handle_collisions(
    mut player_query: Query<(&mut Health, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let (mut player_health, player_transform) = player_query.single_mut();

    let player_size = Vec2::splat(32.0); // matches your player size
    let player_pos = player_transform.translation.truncate();

    for enemy_transform in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation.truncate();
        let enemy_size = Vec2::new(30.0, 30.0); // matches your enemy size

        let collision = aabb_collision(player_pos, player_size, enemy_pos, enemy_size);

        if collision {
            // Damage player
            player_health.current -= 10.0;
            println!("Player hit! HP: {}", player_health.current);
            // commands.entity(entity).despawn();
        }
    }
}

fn bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health, Option<&mut Sprite>), With<Enemy>>,
) {
    let bullet_radius = 4.0;
    let enemy_radius = 15.0;

    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();
        for (enemy_entity, enemy_transform, mut health, mut sprite) in enemy_query.iter_mut() {
            let enemy_pos = enemy_transform.translation.truncate();

            let distance = bullet_pos.distance(enemy_pos);
            if distance < bullet_radius + enemy_radius {
                // Apply damage
                health.current -= 10.0;
                commands.entity(enemy_entity)
                    .insert(HitFlash {
                        timer: Timer::from_seconds(0.2, TimerMode::Once),
                    });

                // Change the enemy’s color to flash color
                if let Some(mut sprite_data) = sprite {
                    sprite_data.color = YELLOW_COLOR; // or Color::YELLOW
                }
                // Despawn enemy if health reaches zero
                if health.current <= 0.0 {
                    commands.entity(enemy_entity).despawn();
                }

                // Despawn the bullet either way
                commands.entity(bullet_entity).despawn();
                break;
            }
        }
    }
}


fn aabb_collision(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    let half1 = size1 / 2.0;
    let half2 = size2 / 2.0;

    let delta = pos1 - pos2;
    delta.x.abs() < (half1.x + half2.x) && delta.y.abs() < (half1.y + half2.y)
}


fn player_shooting(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut last_shot_timer: ResMut<LastShotTimer>,
    time: Res<Time>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<&Transform, With<Player>>,
) {
    last_shot_timer.0.tick(time.delta());
    if !last_shot_timer.0.finished() {
        return;
    }

    if !buttons.pressed(MouseButton::Left) {
        return;
    }
    last_shot_timer.0.reset();


    let Ok(window) = windows.get_single() else { return; };
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return; };
    let Ok(player_transform) = player_query.get_single() else { return; };

    if let Some(screen_pos) = window.cursor_position() {
        // Convert screen position to world coordinates
        let world_pos = camera.viewport_to_world_2d(camera_transform, screen_pos);
        if let Ok(target_pos) = world_pos {
            let direction = (target_pos - player_transform.translation.truncate()).normalize_or_zero();

            commands.spawn((
                Sprite {
                    color: BLUE_COLOR,
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                Transform::from_translation(player_transform.translation),
                Bullet {
                    lifetime: Timer::from_seconds(1.5, TimerMode::Once)
                },
                Velocity(direction * 400.0), // pixels per second
            ));
        }
    }
}

fn bullet_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in query.iter_mut() {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_movement(
    mut query: Query<(&mut Transform, &Velocity), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}