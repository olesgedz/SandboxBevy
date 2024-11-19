use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;

// Components
#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Component)]
struct Attack {
    range: i32,
    damage: i32,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct AIControlled;

#[derive(Component)]
struct PlayerUnit;

#[derive(Component)]
struct EnemyUnit;

// Resources
#[derive(Resource, Clone, Eq, PartialEq, Debug, Hash)]
enum TurnState {
    PlayerTurn,
    EnemyTurn,
    ProcessingTurn,
}

// Action Types
enum ActionType {
    Move(Position), // Move to a new position
    Attack(Entity), // Attack a specific target
}

// Bevy App
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TurnState::PlayerTurn)
        .add_systems(Startup, setup)
        .add_systems(Update, ai_turn_system)
        .run();
}

// Game Setup
fn setup(mut commands: Commands) {
    // Spawn player units
    commands.spawn(PlayerUnit)
        .insert(AIControlled)
        .insert(Health { current: 100, max: 100 })
        .insert(Attack { range: 1, damage: 20 })
        .insert(Position { x: 0, y: 0 });

    // Spawn enemy units
    commands.spawn(EnemyUnit)
        .insert(AIControlled)
        .insert(Health { current: 50, max: 50 })
        .insert(Attack { range: 1, damage: 10 })
        .insert(Position { x: 5, y: 5 });
}

// AI Decision Logic
fn decide_action(
    attacker_position: &Position,
    attack_range: i32,
    enemies: &[(Entity, &Position)],
) -> Option<ActionType> {
    for (enemy_entity, enemy_position) in enemies {
        let distance = (attacker_position.x - enemy_position.x).abs()
            + (attacker_position.y - enemy_position.y).abs();
        if distance <= attack_range {
            return Some(ActionType::Attack(*enemy_entity));
        }
    }
    // Default to move toward (1, 1) for simplicity
    Some(ActionType::Move(Position { x: attacker_position.x + 1, y: attacker_position.y }))
}

// AI Turn System
fn ai_turn_system(
    mut commands: Commands,
    mut turn_state: ResMut<TurnState>,
    mut player_query: Query<(Entity, &Position, &Attack, &mut Health), With<PlayerUnit>>, // `&mut Health` for players
    mut enemy_query: Query<(Entity, &Position, &Attack, &mut Health), With<EnemyUnit>>, // `&mut Health` for enemies
) {
    if *turn_state == TurnState::ProcessingTurn {
        return; // Skip if already processing actions
    }

    let (is_player_turn, mut current_units, mut enemies) = match *turn_state {
        TurnState::PlayerTurn => (
            true,
            player_query.iter_mut().collect::<Vec<_>>(), // Mutable access to player units
            enemy_query.iter_mut().collect::<Vec<_>>(), // Mutable access to enemy units
        ),
        TurnState::EnemyTurn => (
            false,
            enemy_query.iter_mut().collect::<Vec<_>>(), // Mutable access to enemy units
            player_query.iter_mut().collect::<Vec<_>>(), // Mutable access to player units
        ),
        TurnState::ProcessingTurn => {
            return; // Skip processing for this turn state
        }
    };

    for (entity, position, attack, ref mut health) in &mut current_units {
        // Map enemies to only include their position for decision-making
        let enemy_positions: Vec<(Entity, &Position)> = enemies
            .iter()
            .map(|(enemy_entity, enemy_position, _, _)| (*enemy_entity, *enemy_position))
            .collect();

        // Decide what to do (attack or move)
        let action = decide_action(position, attack.range, &enemy_positions);

        if let Some(action) = action {
            match action {
                ActionType::Attack(target) => {
                    for (enemy_entity, _, _, ref mut enemy_health) in &mut enemies {
                        if *enemy_entity == target {
                            enemy_health.current -= attack.damage;
                            println!(
                                "Entity {:?} attacked {:?} for {} damage!",
                                entity, target, attack.damage
                            );
                            if enemy_health.current <= 0 {
                                println!("Entity {:?} is defeated!", target);
                                commands.entity(*enemy_entity).despawn();
                            }
                        }
                    }
                }
                ActionType::Move(new_position) => {
                    println!(
                        "Entity {:?} moves from ({}, {}) to ({}, {})",
                        entity, position.x, position.y, new_position.x, new_position.y
                    );
                    commands.entity(*entity).insert(new_position);
                }
            }
        }
    }

    // Switch turn
    *turn_state = if is_player_turn {
        TurnState::EnemyTurn
    } else {
        TurnState::PlayerTurn
    };
}
