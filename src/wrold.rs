use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::Plane::default().into()),
            material: materials.add(Color::DARK_GREEN.into()),
            transform: Transform::default().with_scale(Vec3::new(20.0, 1.0, 20.0)),
            ..default()
        },
        RigidBody::Fixed,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 8000.0,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
