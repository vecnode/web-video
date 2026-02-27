// main.rs
// Copyright (C) 2026 vecnode

mod components;
mod constants;
mod setup;
mod systems;

use bevy::prelude::*;
use bevy::camera::Viewport;
use bevy_egui::{EguiPlugin, EguiGlobalSettings, PrimaryEguiContext, EguiPrimaryContextPass};

use setup::*;
use systems::*;
use components::EguiLayoutState;
use constants::WORLD_BACKGROUND_COLOR;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(EguiPlugin::default())
        .insert_resource(EguiGlobalSettings {
            auto_create_primary_context: false,
            ..default()
        })
        .insert_resource(ClearColor(WORLD_BACKGROUND_COLOR))
        .init_resource::<components::CameraProjectionState>()
        .init_resource::<components::EguiLayoutState>()
        .init_resource::<components::GridState>()
        .init_resource::<components::StreamsPanelState>()
        .init_resource::<components::LoadedTextures>()
        .init_resource::<components::AspectRatioState>()
        .init_resource::<components::TextureModeState>()
        .add_systems(
            Startup,
            (
                spawn_grid,
                spawn_textured_plane,
                setup_camera_and_lights,
                setup_split_screen_cameras,
            ),
        )
        .add_systems(
            Update,
            (
                update_grid_dimensions,
                update_texture_aspect_ratio,
            ),
        )
        .add_systems(
            Update,
            update_camera_viewports,
        )
        .add_systems(
            EguiPrimaryContextPass,
            egui_controls_ui,
        )
        .run();
}

fn setup_split_screen_cameras(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    // Disable auto-create primary context
    egui_global_settings.auto_create_primary_context = false;
    
    // Single camera for 3D world (will take remaining space on right)
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0_f32.to_radians(), // 60 degrees FOV
            ..default()
        }),
        Transform::from_translation(crate::constants::CAMERA_TOP_POSITION).looking_at(Vec3::ZERO, Vec3::Z),
        crate::components::RightCamera,
    ));
    
    // Primary Egui context camera (renders UI on top)
    commands.spawn((
        PrimaryEguiContext,
        Camera2d::default(),
        Camera {
            order: 10,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
    ));
}

fn update_camera_viewports(
    window: Query<&Window>,
    mut right_camera: Query<&mut Camera, With<crate::components::RightCamera>>,
    layout_state: Res<EguiLayoutState>,
) {
    let Ok(window) = window.single() else { return };
    let physical_size = window.physical_size();
    let scale_factor = window.scale_factor() as f32;
    
    // Use actual panel positions from Egui layout (in logical pixels, convert to physical)
    let left_panel_end_physical = (layout_state.left_panel_end_x * scale_factor) as u32;
    let top_bars_height_physical = (layout_state.top_bars_height * scale_factor) as u32;
    let bottom_bar_height_physical = (layout_state.bottom_bar_height * scale_factor) as u32;
    
    // Calculate viewport width: extend to right edge if inspector is collapsed, otherwise stop at inspector
    let viewport_right_edge = if layout_state.inspector_collapsed {
        physical_size.x // Extend to right edge of window when inspector is hidden
    } else {
        (layout_state.right_panel_start_x * scale_factor) as u32 // Stop at inspector when visible
    };
    
    // Calculate total available space: from left panel end to right edge (inspector or window edge)
    // Height: from below top bars to above bottom bar
    let total_viewport_width = viewport_right_edge.saturating_sub(left_panel_end_physical);
    let viewport_height = physical_size.y.saturating_sub(top_bars_height_physical).saturating_sub(bottom_bar_height_physical);
    
    // Camera viewport always visible - uses full available width
    if let Ok(mut camera) = right_camera.single_mut() {
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(left_panel_end_physical, top_bars_height_physical),
            physical_size: UVec2::new(total_viewport_width, viewport_height),
            ..default()
        });
    }
}