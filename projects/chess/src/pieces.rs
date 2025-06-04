use bevy::prelude::*;

pub fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn((Transform::from_translation(position), Visibility::Visible))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), {
                let transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            }));
            parent.spawn((
                Mesh3d(mesh_cross.clone()),
                MeshMaterial3d(material.clone()),
                {
                    let transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9))
                        .with_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
            ));
        });
}

pub fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh1: Handle<Mesh>,
    mesh2: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn((Transform::from_translation(position), Visibility::Visible))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((Mesh3d(mesh1.clone()), MeshMaterial3d(material.clone()), {
                let transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            }));
            parent.spawn((Mesh3d(mesh2.clone()), MeshMaterial3d(material.clone()), {
                let transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            }));
        });
}

pub fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn((
            Transform::from_translation(position - Vec3::new(0., 0., 2.0)),
            Visibility::Visible,
        ))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), {
                let transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.95))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            }));
        });
}

pub fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn((Transform::from_translation(position), Visibility::Visible))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), {
                let transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            }));
        });
}

pub fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn((Transform::from_translation(position), Visibility::Visible))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), {
                let transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            }));
        });
}

pub fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn((Transform::from_translation(position), Visibility::Visible))
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), {
                let transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            }));
        });
}
