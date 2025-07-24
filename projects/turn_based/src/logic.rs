use bevy::prelude::*;

use crate::unit::Stats;

#[derive(Resource)]
pub struct TurnQueue(Vec<Entity>);

#[derive(Resource)]
pub struct CurrentTurn(Option<Entity>);
#[derive(Event)]
pub struct EndTurnEvent;
pub fn setup_turn_queue(
    mut commands: Commands,
    units: Query<(Entity, &Stats)>,
) {
    let mut queue: Vec<(Entity, u32)> = units.iter().map(|(e, stats)| (e, stats.speed)).collect();

    // sort by speed descending
    queue.sort_by_key(|&(_, speed)| std::cmp::Reverse(speed));

    let ordered_entities = queue.into_iter().map(|(e, _)| e).collect();

    commands.insert_resource(TurnQueue(ordered_entities));
    commands.insert_resource(CurrentTurn(None));
}

pub fn begin_turn(
    mut current: ResMut<CurrentTurn>,
    mut queue: ResMut<TurnQueue>,
) {
    if current.0.is_none() {
        if let Some(next) = queue.0.first().copied() {
            current.0 = Some(next);
            println!("Turn begins for {:?}", next);
        }
    }
}

pub fn end_turn(
    mut current: ResMut<CurrentTurn>,
    mut queue: ResMut<TurnQueue>,
) {
    if let Some(done_unit) = current.0 {
        queue.0.remove(0);
        queue.0.push(done_unit); // rotate to back
        current.0 = None;
    }
}