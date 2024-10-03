use bevy::prelude::*;

#[derive(Resource)]
pub struct MousePos {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub world: Vec3,
}

impl Default for MousePos {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
            world: Vec3::splat(0.0),
        }
    }
}
