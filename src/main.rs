mod components;
mod scene;
mod systems;

use bevy::prelude::*;
use bevy::window::PresentMode;

use scene::setup_scene;
use systems::{
    animate_bob, animate_orbits, animate_spins, cycle_camera_shot, move_camera, pulse_lights,
    zoom_camera,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.012, 0.014, 0.02)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ECS / Bevy / Programmez - clic droit: vue, molette: zoom".into(),
                resolution: (1280, 720).into(),
                present_mode: PresentMode::AutoVsync,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                cycle_camera_shot,
                zoom_camera,
                animate_orbits,
                animate_bob,
                animate_spins,
                move_camera,
                pulse_lights,
            ),
        )
        .run();
}
