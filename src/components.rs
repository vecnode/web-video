// components.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;

#[derive(Component)]
pub struct RightCamera;

#[derive(Resource)]
pub struct CameraProjectionState {
    pub last_perspective_fov: f32, // Store FOV for camera projection state
}

impl Default for CameraProjectionState {
    fn default() -> Self {
        Self {
            last_perspective_fov: 1.047, // Default ~60 degrees, will be updated from actual camera
        }
    }
}

#[derive(Resource)]
pub struct EguiLayoutState {
    pub left_panel_end_x: f32, // Actual x position where left panel ends (in logical pixels)
    pub right_panel_start_x: f32, // Actual x position where right panel starts (in logical pixels)
    pub top_bars_height: f32, // Combined height of both top bars (in logical pixels)
    pub bottom_bar_height: f32, // Height of the bottom bar (in logical pixels)
    pub left_panel_content_width: f32, // Actual content width inside left panel (in logical pixels)
    pub right_panel_content_width: f32, // Actual content width inside right panel (in logical pixels)
    pub inspector_collapsed: bool, // Whether the inspector panel is collapsed
}

impl Default for EguiLayoutState {
    fn default() -> Self {
        Self {
            left_panel_end_x: 0.0,
            right_panel_start_x: 0.0,
            top_bars_height: 0.0,
            bottom_bar_height: 0.0,
            left_panel_content_width: 0.0,
            right_panel_content_width: 0.0,
            inspector_collapsed: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct StreamsPanelState {
    pub is_visible: bool,
}

#[derive(Resource, Default)]
pub struct LoadedTextures {
    pub textures: Vec<String>, // Store texture paths/names
}

#[derive(Component)]
pub struct GridLine;

#[derive(Component)]
pub struct TexturedPlane;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AspectRatio {
    Ratio16_9,
    Square,
}

#[derive(Resource)]
pub struct AspectRatioState {
    pub current: AspectRatio,
    pub previous: AspectRatio,
}

impl Default for AspectRatioState {
    fn default() -> Self {
        Self {
            current: AspectRatio::Square,
            previous: AspectRatio::Square,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureMode {
    Normal,
    Stretch,
}

#[derive(Resource)]
pub struct TextureModeState {
    pub current: TextureMode,
    pub previous: TextureMode,
}

impl Default for TextureModeState {
    fn default() -> Self {
        Self {
            current: TextureMode::Stretch,
            previous: TextureMode::Stretch,
        }
    }
}

#[derive(Resource)]
pub struct GridState {
    pub size_x: i32, // Grid size in X direction (meters)
    pub size_z: i32, // Grid size in Z direction (meters)
    pub previous_size_x: i32,
    pub previous_size_z: i32,
}

impl Default for GridState {
    fn default() -> Self {
        Self {
            size_x: 10,
            size_z: 10,
            previous_size_x: 10,
            previous_size_z: 10,
        }
    }
}