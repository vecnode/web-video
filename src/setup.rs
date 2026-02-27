// setup.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::constants::*;

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_state: Option<Res<crate::components::GridState>>,
) {
    // Use grid_state if available, otherwise use default values (10x10)
    let size_x = grid_state.as_ref().map(|gs| gs.size_x).unwrap_or(10) as f32;
    let size_z = grid_state.as_ref().map(|gs| gs.size_z).unwrap_or(10) as f32;
    let half_size_x = size_x / 2.0;
    let half_size_z = size_z / 2.0;
    let num_lines_x = grid_state.as_ref().map(|gs| gs.size_x).unwrap_or(10) + 1;
    let num_lines_z = grid_state.as_ref().map(|gs| gs.size_z).unwrap_or(10) + 1;
    
    // Create grid lines along X axis (parallel to Z) - these lines span the X direction
    for i in 0..num_lines_z {
        let z = -half_size_z + (i as f32 * GRID_SPACING);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_x))),
            MeshMaterial3d(materials.add(GRID_COLOR)),
            Transform::from_translation(Vec3::new(0.0, 0.0, z))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            crate::components::GridLine,
        ));
    }
    
    // Create grid lines along Z axis (parallel to X) - these lines span the Z direction
    for i in 0..num_lines_x {
        let x = -half_size_x + (i as f32 * GRID_SPACING);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_z))),
            MeshMaterial3d(materials.add(GRID_COLOR)),
            Transform::from_translation(Vec3::new(x, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            crate::components::GridLine,
        ));
    }
}

pub fn spawn_textured_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    grid_state: Option<Res<crate::components::GridState>>,
    mut loaded_textures: ResMut<crate::components::LoadedTextures>,
) {
    // Use grid_state if available, otherwise use default values (10x10)
    let size_x = grid_state.as_ref().map(|gs| gs.size_x).unwrap_or(10) as f32;
    let size_z = grid_state.as_ref().map(|gs| gs.size_z).unwrap_or(10) as f32;
    
    // Load the texture
    let texture_path = "tree.png";
    let texture_handle = asset_server.load(texture_path);
    
    // Register the texture in loaded textures
    if !loaded_textures.textures.contains(&texture_path.to_string()) {
        loaded_textures.textures.push(texture_path.to_string());
    }
    
    // Create a plane mesh that matches the grid size using Rectangle
    // Rectangle is 1x1 by default, so we scale it to match grid dimensions
    let plane_mesh = meshes.add(Rectangle::new(size_x, size_z));
    
    // Create material with the texture
    // Use unlit material so the texture appears exactly as in the image file, without lighting effects
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        unlit: true, // Disable lighting to show texture exactly as it appears in the image
        ..default()
    });
    
    // Spawn the plane slightly above the grid (y = 0.01) to ensure it's visible above grid lines
    // Rotate -90 degrees around X axis to make it horizontal (floor plane)
    // Then rotate 180 degrees around Z axis to flip the texture right-side up when viewed from above
    commands.spawn((
        Mesh3d(plane_mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(0.0, 0.01, 0.0))
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * Quat::from_rotation_z(std::f32::consts::PI)),
        crate::components::TexturedPlane,
    ));
}

pub fn setup_camera_and_lights(mut commands: Commands) {
    // Front light
    commands.spawn(DirectionalLight {
        illuminance: FRONT_LIGHT_ILLUMINANCE,
        ..default()
    });
    
    // Back light (from behind)
    commands.spawn((
        DirectionalLight {
            illuminance: BACK_LIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
    ));
}
