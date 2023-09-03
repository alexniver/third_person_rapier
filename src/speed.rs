use bevy::prelude::*;
#[derive(Component)]
pub struct Speed {
    pub value: f32,
}

impl Speed {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
