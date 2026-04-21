use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Orbit {
    pub radius: f32,
    pub speed: f32,
    pub phase: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Bob {
    pub base_height: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Spin {
    pub axis: Vec3,
    pub speed: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CameraRig {
    pub active_shot: usize,
    pub shot_count: usize,
    pub transition_speed: f32,
    pub current_focus: Vec3,
    pub current_zoom: f32,
    pub target_zoom: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct LightPulse {
    pub base_intensity: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}
