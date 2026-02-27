// constants.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;

// Grid constants
pub const GRID_SPACING: f32 = 1.0;
pub const GRID_LINE_RADIUS: f32 = 0.005;
pub const GRID_COLOR: Color = Color::srgb(0.5, 0.5, 0.5); // Mid gray

// Camera constants
pub const CAMERA_TOP_POSITION: Vec3 = Vec3::new(0.0, 15.0, 0.0);

// Lighting constants
pub const FRONT_LIGHT_ILLUMINANCE: f32 = 2000.0;
pub const BACK_LIGHT_ILLUMINANCE: f32 = 1500.0;

// World background color
pub const WORLD_BACKGROUND_COLOR: Color = Color::srgb(0.08, 0.08, 0.08); // Very dark, almost pure black

// UI layout constants
pub const EGUI_TOP_BAR_HEIGHT: f32 = 20.0;
pub const EGUI_SECOND_TOP_BAR_HEIGHT: f32 = 22.0; // 2px for the buttons
pub const EGUI_LEFT_PANEL_WIDTH: f32 = 200.0;