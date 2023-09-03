use bevy::{prelude::*, window::close_on_esc};
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use trdperson_rapier::{camera::CameraPlugin, player::PlayerPlugin, wrold::WorldPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            PlayerPlugin,
            WorldPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Update, close_on_esc)
        .run();
}
