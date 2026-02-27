// systems/texture.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy::math::Affine2;
use crate::components::{TexturedPlane, AspectRatioState, GridState, TextureModeState, TextureMode};

pub fn update_texture_aspect_ratio(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut aspect_ratio_state: ResMut<AspectRatioState>,
    mut texture_mode_state: ResMut<TextureModeState>,
    grid_state: Res<GridState>,
    images: Res<Assets<Image>>,
    mut plane_query: Query<(&mut Mesh3d, &MeshMaterial3d<StandardMaterial>), With<TexturedPlane>>,
) {
    // Check if aspect ratio changed or grid size changed
    let aspect_ratio_changed = aspect_ratio_state.current != aspect_ratio_state.previous;
    let grid_size_changed = grid_state.size_z != grid_state.previous_size_z;
    let texture_mode_changed = texture_mode_state.current != texture_mode_state.previous;
    
    if aspect_ratio_changed || grid_size_changed || texture_mode_changed {
        if aspect_ratio_changed {
            aspect_ratio_state.previous = aspect_ratio_state.current;
        }
        if texture_mode_changed {
            texture_mode_state.previous = texture_mode_state.current;
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
        
        // Update the plane mesh and material for each textured plane
        for (mut mesh_3d, material_3d) in plane_query.iter_mut() {
            // Update mesh dimensions
            let new_mesh = meshes.add(Rectangle::new(size_x, size_z));
            mesh_3d.0 = new_mesh;
            
            // Update material based on texture mode
            if let Some(material) = materials.get_mut(&material_3d.0) {
                // Ensure material remains unlit to display texture exactly as in the image
                material.unlit = true;
                
                if let Some(texture_handle) = &material.base_color_texture {
                    if let Some(image) = images.get(texture_handle) {
                        let texture_width = image.width() as f32;
                        let texture_height = image.height() as f32;
                        let texture_aspect = texture_width / texture_height;
                        let plane_aspect = size_x / size_z;
                        
                        match texture_mode_state.current {
                            TextureMode::Normal => {
                                // Maintain texture's original aspect ratio and center it
                                let (scale_x, scale_z, offset_x, offset_z) = if texture_aspect > plane_aspect {
                                    // Texture is wider relative to its height than the plane
                                    // Fit texture to plane's width, scale height proportionally, center vertically
                                    let scale = size_x / texture_width; // Scale to fit width
                                    let scaled_height = texture_height * scale;
                                    let scale_z = scaled_height / size_z; // UV scale in Z direction
                                    let offset_z = (1.0 - scale_z) * 0.5; // Center vertically in UV space
                                    (1.0, scale_z, 0.0, offset_z)
                                } else {
                                    // Texture is taller relative to its width than the plane
                                    // Fit texture to plane's height, scale width proportionally, center horizontally
                                    let scale = size_z / texture_height; // Scale to fit height
                                    let scaled_width = texture_width * scale;
                                    let scale_x = scaled_width / size_x; // UV scale in X direction
                                    let offset_x = (1.0 - scale_x) * 0.5; // Center horizontally in UV space
                                    (scale_x, 1.0, offset_x, 0.0)
                                };
                                
                                // Create UV transform using Affine2 to scale and offset texture
                                // Affine2 represents a 2D affine transformation (scale + translation)
                                let scale = Vec2::new(scale_x, scale_z);
                                let translation = Vec2::new(offset_x, offset_z);
                                let uv_transform = Affine2::from_scale_angle_translation(scale, 0.0, translation);
                                
                                // Update material with UV transform
                                material.uv_transform = uv_transform;
                            }
                            TextureMode::Stretch => {
                                // Stretch texture to fill the plane (default behavior)
                                material.uv_transform = Affine2::IDENTITY;
                            }
                        }
                    }
                }
            }
        }
    }
}
