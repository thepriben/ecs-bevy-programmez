use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::components::{Bob, CameraRig, LightPulse, Orbit, Spin};

fn camera_shot(index: usize) -> (Vec3, Vec3) {
    match index % 4 {
        0 => (Vec3::new(-11.0, 7.5, 11.0), Vec3::new(0.0, 1.6, 0.0)),
        1 => (Vec3::new(0.0, 3.3, 6.8), Vec3::new(0.0, 1.8, 0.0)),
        2 => (Vec3::new(8.5, 10.5, 0.0), Vec3::new(0.0, 1.4, 0.0)),
        _ => (Vec3::new(-3.5, 4.0, -8.8), Vec3::new(0.0, 1.7, 0.0)),
    }
}

pub fn animate_orbits(time: Res<Time>, mut query: Query<(&Orbit, &mut Transform)>) {
    let t = time.elapsed_secs();

    for (orbit, mut transform) in &mut query {
        let angle = t * orbit.speed + orbit.phase;
        transform.translation.x = orbit.radius * angle.cos();
        transform.translation.z = orbit.radius * angle.sin();
    }
}

pub fn animate_bob(time: Res<Time>, mut query: Query<(&Bob, &mut Transform)>) {
    let t = time.elapsed_secs();

    for (bob, mut transform) in &mut query {
        transform.translation.y =
            bob.base_height + bob.amplitude * (t * bob.speed + bob.phase).sin();
    }
}

pub fn animate_spins(time: Res<Time>, mut query: Query<(&Spin, &mut Transform)>) {
    for (spin, mut transform) in &mut query {
        let axis = spin.axis.normalize_or_zero();
        if axis != Vec3::ZERO {
            transform.rotate(Quat::from_axis_angle(axis, spin.speed * time.delta_secs()));
        }
    }
}

pub fn cycle_camera_shot(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut CameraRig>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Right) {
        return;
    }

    for mut camera_rig in &mut query {
        camera_rig.active_shot = (camera_rig.active_shot + 1) % camera_rig.shot_count;
    }
}

pub fn zoom_camera(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut query: Query<&mut CameraRig>,
) {
    let mut zoom_delta = 0.0;

    for event in mouse_wheel_reader.read() {
        let step = match event.unit {
            MouseScrollUnit::Line => event.y * 0.10,
            MouseScrollUnit::Pixel => event.y * 0.0015,
        };
        zoom_delta += step;
    }

    if zoom_delta == 0.0 {
        return;
    }

    for mut camera_rig in &mut query {
        camera_rig.target_zoom =
            (camera_rig.target_zoom - zoom_delta).clamp(camera_rig.min_zoom, camera_rig.max_zoom);
    }
}

pub fn move_camera(time: Res<Time>, mut query: Query<(&mut CameraRig, &mut Transform)>) {
    for (mut camera_rig, mut transform) in &mut query {
        let blend = 1.0 - (-time.delta_secs() * camera_rig.transition_speed).exp();
        let (base_position, target_focus) = camera_shot(camera_rig.active_shot);
        let offset = base_position - target_focus;

        camera_rig.current_zoom += (camera_rig.target_zoom - camera_rig.current_zoom) * blend;
        let target_position = target_focus + offset * camera_rig.current_zoom;

        transform.translation = transform.translation.lerp(target_position, blend);
        camera_rig.current_focus = camera_rig.current_focus.lerp(target_focus, blend);
        transform.look_at(camera_rig.current_focus, Vec3::Y);
    }
}

pub fn pulse_lights(time: Res<Time>, mut query: Query<(&LightPulse, &mut PointLight)>) {
    let t = time.elapsed_secs();

    for (pulse, mut light) in &mut query {
        light.intensity =
            pulse.base_intensity + pulse.amplitude * (t * pulse.speed + pulse.phase).sin();
    }
}
