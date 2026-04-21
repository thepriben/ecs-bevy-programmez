use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::components::{Bob, CameraRig, LightPulse, Orbit, Spin};

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let initial_focus = Vec3::new(0.0, 1.6, 0.0);

    commands.spawn((
        Camera3d::default(),
        CameraRig {
            active_shot: 0,
            shot_count: 4,
            transition_speed: 4.0,
            current_focus: initial_focus,
            current_zoom: 1.0,
            target_zoom: 1.0,
            min_zoom: 0.55,
            max_zoom: 1.75,
        },
        Transform::from_xyz(-11.0, 7.5, 11.0).looking_at(initial_focus, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.05, -0.85, 0.0)),
    ));

    commands.spawn((
        PointLight {
            intensity: 650_000.0,
            range: 28.0,
            radius: 0.35,
            shadows_enabled: true,
            color: Color::srgb(0.97, 0.61, 0.34),
            ..default()
        },
        LightPulse {
            base_intensity: 650_000.0,
            amplitude: 130_000.0,
            speed: 1.7,
            phase: 0.0,
        },
        Transform::from_xyz(4.5, 6.0, 4.5),
    ));

    commands.spawn((
        PointLight {
            intensity: 520_000.0,
            range: 26.0,
            radius: 0.30,
            shadows_enabled: true,
            color: Color::srgb(0.36, 0.76, 0.97),
            ..default()
        },
        LightPulse {
            base_intensity: 520_000.0,
            amplitude: 110_000.0,
            speed: 1.1,
            phase: 1.9,
        },
        Transform::from_xyz(-5.0, 5.0, -3.5),
    ));

    let platform_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.055, 0.06, 0.075),
        perceptual_roughness: 0.96,
        metallic: 0.04,
        ..default()
    });

    let core_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.88, 0.46, 0.24),
        metallic: 0.18,
        perceptual_roughness: 0.38,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(11.5, 0.18).mesh().resolution(72))),
        MeshMaterial3d(platform_material.clone()),
        Transform::from_xyz(0.0, -0.09, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(2.7, 0.72).mesh().resolution(72))),
        MeshMaterial3d(platform_material),
        Transform::from_xyz(0.0, 0.35, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.45).mesh().ico(5).expect("valid ico sphere"))),
        MeshMaterial3d(core_material),
        Spin {
            axis: Vec3::new(0.0, 1.0, 0.25),
            speed: 0.85,
        },
        Bob {
            base_height: 1.95,
            amplitude: 0.18,
            speed: 0.9,
            phase: 0.3,
        },
        Transform::from_xyz(0.0, 1.95, 0.0),
    ));

    let cube_mesh = meshes.add(Cuboid::new(0.95, 0.95, 0.95));
    let satellite_mesh = meshes.add(Sphere::new(0.34).mesh().ico(4).expect("valid ico sphere"));

    let cube_colors = [
        Color::srgb(0.92, 0.36, 0.28),
        Color::srgb(0.95, 0.74, 0.28),
        Color::srgb(0.25, 0.76, 0.58),
        Color::srgb(0.28, 0.64, 0.95),
        Color::srgb(0.77, 0.45, 0.96),
        Color::srgb(0.96, 0.47, 0.71),
    ];

    for index in 0..6 {
        let phase = index as f32 * TAU / 6.0;
        let material = materials.add(StandardMaterial {
            base_color: cube_colors[index % cube_colors.len()],
            metallic: 0.08,
            perceptual_roughness: 0.32,
            ..default()
        });

        commands.spawn((
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(material),
            Orbit {
                radius: 5.0,
                speed: 0.82,
                phase,
            },
            Bob {
                base_height: 1.85,
                amplitude: 0.65,
                speed: 1.45,
                phase,
            },
            Spin {
                axis: Vec3::new(1.0, 1.0, 0.2).normalize(),
                speed: 1.25 + index as f32 * 0.08,
            },
            Transform::default(),
        ));
    }

    for index in 0..14 {
        let phase = index as f32 * TAU / 14.0;
        let hue_blend = index as f32 / 14.0;
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(
                0.30 + hue_blend * 0.45,
                0.55 + hue_blend * 0.22,
                0.92 - hue_blend * 0.34,
            ),
            metallic: 0.02,
            perceptual_roughness: 0.26,
            ..default()
        });

        commands.spawn((
            Mesh3d(satellite_mesh.clone()),
            MeshMaterial3d(material),
            Orbit {
                radius: 8.0,
                speed: -0.36,
                phase,
            },
            Bob {
                base_height: 0.95,
                amplitude: 0.38,
                speed: 2.1,
                phase: phase * 1.6,
            },
            Spin {
                axis: Vec3::Y,
                speed: 1.8,
            },
            Transform::default(),
        ));
    }
}
