// systems/grid.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{GridLine, GridVisibilityState, AspectRatioState, GridState};
use crate::constants::{GRID_LINE_RADIUS, GRID_COLOR, GRID_BORDER_RADIUS, GRID_BORDER_COLOR};

pub fn update_grid_visibility(
    grid_visibility_state: Res<GridVisibilityState>,
    mut grid_line_query: Query<&mut Visibility, With<GridLine>>,
) {
    // Update visibility of all grid lines based on state
    for mut visibility in grid_line_query.iter_mut() {
        *visibility = if grid_visibility_state.is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn update_grid_dimensions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    aspect_ratio_state: Res<AspectRatioState>,
    grid_state: Res<GridState>,
    grid_line_query: Query<Entity, With<GridLine>>,
) {
    // Check if aspect ratio changed
    // Note: We don't update the previous value here - let the texture system handle that
    let aspect_ratio_changed = aspect_ratio_state.current != aspect_ratio_state.previous;
    
    if aspect_ratio_changed {
        // Despawn all existing grid lines
        for entity in grid_line_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Calculate grid dimensions based on aspect ratio (matching texture plane)
        let grid_z = grid_state.size_z as f32;
        let (size_x, size_z) = match aspect_ratio_state.current {
            crate::components::AspectRatio::Ratio16_9 => {
                // 16:9 aspect ratio: width = height * (16/9)
                let size_x = grid_z * (16.0 / 9.0);
                (size_x, grid_z)
            }
            crate::components::AspectRatio::Square => {
                // Square (1:1) aspect ratio
                (grid_z, grid_z)
            }
        };
        
        let half_size_x = size_x / 2.0;
        let half_size_z = size_z / 2.0;
        
        // Use the grid_state dimensions to determine number of grid cells
        // This ensures consistent grid cell count regardless of aspect ratio
        let num_cells_x = grid_state.size_x;
        let num_cells_z = grid_state.size_z;
        
        // Calculate spacing to fit exactly within the plane dimensions
        // Spacing = total dimension / number of cells
        let spacing_x = size_x / num_cells_x as f32;
        let spacing_z = size_z / num_cells_z as f32;
        
        // Number of lines = number of cells + 1 (lines at edges and between cells)
        let num_lines_x = num_cells_x + 1;
        let num_lines_z = num_cells_z + 1;
        
        // Create grid lines along X axis (parallel to Z) - these lines span the X direction
        // Lines are placed from -half_size_z to +half_size_z
        for i in 0..num_lines_z {
            let z = -half_size_z + (i as f32 * spacing_z);
            commands.spawn((
                Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_x))),
                MeshMaterial3d(materials.add(GRID_COLOR)),
                Transform::from_translation(Vec3::new(0.0, 0.02, z)) // Position at y=0.02 to appear in front of texture
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                Visibility::default(),
                GridLine,
            ));
        }
        
        // Create grid lines along Z axis (parallel to X) - these lines span the Z direction
        // Lines are placed from -half_size_x to +half_size_x
        for i in 0..num_lines_x {
            let x = -half_size_x + (i as f32 * spacing_x);
            commands.spawn((
                Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_z))),
                MeshMaterial3d(materials.add(GRID_COLOR)),
                Transform::from_translation(Vec3::new(x, 0.02, 0.0)) // Position at y=0.02 to appear in front of texture
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                Visibility::default(),
                GridLine,
            ));
        }
        
        // Create pink border around the perimeter
        // Top edge (at +half_size_z, spanning X direction)
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_BORDER_RADIUS, size_x))),
            MeshMaterial3d(materials.add(GRID_BORDER_COLOR)),
            Transform::from_translation(Vec3::new(0.0, 0.02, half_size_z))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            Visibility::default(),
            GridLine,
        ));
        
        // Bottom edge (at -half_size_z, spanning X direction)
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_BORDER_RADIUS, size_x))),
            MeshMaterial3d(materials.add(GRID_BORDER_COLOR)),
            Transform::from_translation(Vec3::new(0.0, 0.02, -half_size_z))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            Visibility::default(),
            GridLine,
        ));
        
        // Left edge (at -half_size_x, spanning Z direction)
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_BORDER_RADIUS, size_z))),
            MeshMaterial3d(materials.add(GRID_BORDER_COLOR)),
            Transform::from_translation(Vec3::new(-half_size_x, 0.02, 0.0))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            Visibility::default(),
            GridLine,
        ));
        
        // Right edge (at +half_size_x, spanning Z direction)
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_BORDER_RADIUS, size_z))),
            MeshMaterial3d(materials.add(GRID_BORDER_COLOR)),
            Transform::from_translation(Vec3::new(half_size_x, 0.02, 0.0))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            Visibility::default(),
            GridLine,
        ));
    }
}
