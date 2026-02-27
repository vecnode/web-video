// systems/texture.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{TexturedPlane, AspectRatioState, GridState};

pub fn update_texture_aspect_ratio(
    mut meshes: ResMut<Assets<Mesh>>,
    mut aspect_ratio_state: ResMut<AspectRatioState>,
    grid_state: Res<GridState>,
    mut plane_query: Query<&mut Mesh3d, With<TexturedPlane>>,
) {
    // Check if aspect ratio changed or grid size changed
    let aspect_ratio_changed = aspect_ratio_state.current != aspect_ratio_state.previous;
    let grid_size_changed = grid_state.size_z != grid_state.previous_size_z;
    
    if aspect_ratio_changed || grid_size_changed {
        if aspect_ratio_changed {
            aspect_ratio_state.previous = aspect_ratio_state.current;
        }
        
        // Get grid size
        let grid_z = grid_state.size_z as f32;
        
        // Calculate plane dimensions based on aspect ratio
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
        
        // Update the plane mesh for each textured plane
        for mut mesh_3d in plane_query.iter_mut() {
            // Create new mesh with updated dimensions
            let new_mesh = meshes.add(Rectangle::new(size_x, size_z));
            mesh_3d.0 = new_mesh;
        }
    }
}
