//! Escena de demostración (FR-005). Temporal: se reemplaza cuando existan escenas reales
//! (feature 003 de la hoja de ruta).

use bevy::prelude::*;

/// Rotación continua alrededor del eje Y, en radianes por segundo.
#[derive(Component, Debug, Clone, Copy)]
pub struct Spin(pub f32);

pub struct DemoScenePlugin;

impl Plugin for DemoScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_demo_scene)
            .add_systems(Update, spin);
    }
}

fn spawn_demo_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Suelo"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.28, 0.30, 0.33))),
        Transform::default(),
    ));

    commands.spawn((
        Name::new("Cubo"),
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.85, 0.35, 0.25))),
        Transform::from_xyz(-3.0, 0.75, 1.0).with_rotation(Quat::from_rotation_y(0.4)),
    ));

    commands.spawn((
        Name::new("Torre"),
        Mesh3d(meshes.add(Cylinder::new(0.6, 4.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.75, 0.72, 0.65))),
        Transform::from_xyz(3.5, 2.0, -2.0),
    ));

    commands.spawn((
        Name::new("Planeta"),
        Spin(0.8),
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().uv(48, 24))),
        MeshMaterial3d(materials.add(Color::srgb(0.25, 0.55, 0.85))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        children![(
            Name::new("Luna"),
            Mesh3d(meshes.add(Sphere::new(0.3).mesh().uv(24, 12))),
            MeshMaterial3d(materials.add(Color::srgb(0.85, 0.85, 0.80))),
            Transform::from_xyz(2.2, 0.0, 0.0),
        )],
    ));

    commands.spawn((
        Name::new("Sol"),
        DirectionalLight {
            illuminance: 8_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spin(time: Res<Time>, mut objects: Query<(&Spin, &mut Transform)>) {
    for (spin, mut transform) in &mut objects {
        transform.rotate_y(spin.0 * time.delta_secs());
    }
}
