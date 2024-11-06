use std::thread::spawn;
use bevy::{
    prelude::*,
};

#[derive(Debug, Component)]
struct Warrior;

#[derive(Debug, Component)]
struct Archer;


#[derive(Component)]
struct Map {
    // 8x8 grid where each square can either be empty or contain an Entity representing a piece
    squares: [[Option<Entity>; 8]; 8],
}

impl Map {
    pub fn new() -> Self {
        Self {
            squares: [[None; 8]; 8], // Initialize an empty board
        }
    }

    // Function to place a piece on the board
    pub fn place_piece(&mut self, entity: Entity, x: usize, y: usize) {
        self.squares[y][x] = Some(entity);
    }

    // Function to remove a piece from the board
    pub fn remove_piece(&mut self, x: usize, y: usize) -> Option<Entity> {
        let piece = self.squares[y][x];
        self.squares[y][x] = None;
        piece
    }

    // Function to get the entity at a specific position
    pub fn get_piece(&self, x: usize, y: usize) -> Option<Entity> {
        self.squares[y][x]
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_board))
        .add_systems(Update, system_voice)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands.spawn(Archer);
    commands.spawn(Warrior);
}

fn system_voice(mut query: Query<&mut Warrior>) {
    // for (warrior, archer) in &mut query.iter() {
    //     println!("Warrior: {:?}", warrior);
    //     println!("Archer: {:?}", archer);
    // }
    for warrior in &mut query.iter() {
        println!("Warrior: {:?}", warrior);
    }
}
fn system_make_turn(mut query: Query<&mut Archer>) {
    for archer in &mut query.iter() {
        println!("Archer: {:?}", archer);
    }
}

fn setup_board(mut commands: Commands, mut board_query: Query<&mut Map>) {
    let mut board = board_query.single_mut();

    // Example: Placing a white pawn at (0, 1)
    let pawn_entity = commands.spawn(Archer {
    }).id();

    board.place_piece(pawn_entity, 0, 1);
}