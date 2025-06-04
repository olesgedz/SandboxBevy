use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    picking::pointer::PointerInteraction,
    prelude::*,
    window::WindowTheme,
};

#[derive(Component, Debug)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Resource)]
struct SelectedSquare {
    entity: Option<Entity>,
}

// impl Default for SelectedSquare {
//     fn default() -> Self {
//         None
//     }
// }

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.5)));
    let white_material = materials.add(Color::srgb(1., 0.9, 0.9));
    let black_material = materials.add(Color::srgb(0., 0.1, 0.1));

    commands.insert_resource(SelectedSquare { entity: None });

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            let material;
            if (i + j + 1) % 2 == 0 {
                material = white_material.clone();
            } else {
                material = black_material.clone()
            }
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                Square { x: i, y: j },
                RayCastPickable,
            ));
        }
    }
}

pub fn color_squares(
    mut interaction_query: Query<(&Interaction), Changed<Interaction>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (interaction) in interaction_query.iter_mut() {
        println!("Interaction: {:?} ", interaction);
        // match *interaction {
        //     Interaction::Pressed => {
        //         // Change the material color to red when clicked
        //         if let Some(mat) = materials.get_mut(&*material) {
        //             mat.base_color = Color::rgb(1.0, 0.0, 0.0);
        //         }
        //     }
        //     Interaction::Hovered => {
        //         // Optional: Change the material color to yellow when hovered
        //         if let Some(mat) = materials.get_mut(&*material) {
        //             mat.base_color = Color::rgb(1.0, 1.0, 0.0);
        //         }
        //     }
        //     Interaction::None => {
        //         // Optional: Reset the material color to green when not hovered
        //         if let Some(mat) = materials.get_mut(&*material) {
        //             mat.base_color = Color::rgb(0.3, 0.5, 0.3);
        //         }
        //     }
        // }
    }
}

pub fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, Color::srgb(1.0, 0.0, 0.0));
        gizmos.arrow(
            point,
            point + normal.normalize() * 0.5,
            Color::srgb(0.0, 1.0, 0.0),
        );
    }

    for (entity, hitData) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
    {}

    // let (target_entity, hit) = pointers.iter().next().unwrap();
}
