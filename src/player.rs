use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};
use bevy_third_person_camera::ThirdPersonCameraTarget;

use crate::speed::Speed;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_move);
    }
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::Cube::default().into()),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::new(0.5, 1.0, 0.5)),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        KinematicCharacterController::default(),
        Collider::cuboid(0.5, 1.0, 0.5),
        Player,
        Speed::new(5.0),
        ThirdPersonCameraTarget,
    ));
}

fn player_move(
    mut player_query: Query<(&mut KinematicCharacterController, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }

    let camera_transform = camera_query.single();

    let mut transform = Vec3::ZERO;
    if keyboard.pressed(KeyCode::A) {
        transform += camera_transform.left();
    }
    if keyboard.pressed(KeyCode::D) {
        transform += camera_transform.right();
    }
    if keyboard.pressed(KeyCode::W) {
        transform += camera_transform.forward();
    }
    if keyboard.pressed(KeyCode::S) {
        transform += camera_transform.back();
    }

    transform = transform.normalize_or_zero();
    transform.y = 0.0;
    println!("transform: {:?}", transform);

    let (mut controler, speed) = player_query.single_mut();

    transform = transform * time.delta_seconds() * speed.value;
    controler.translation = match controler.translation {
        Some(t) => Some(t + transform),
        None => Some(transform),
    }
}
