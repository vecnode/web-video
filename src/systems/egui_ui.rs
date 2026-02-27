// systems/egui_ui.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::{CameraProjectionState, EguiLayoutState, GridState, StreamsPanelState, LoadedTextures, AspectRatioState, AspectRatio, TextureModeState, TextureMode};
use crate::constants::{EGUI_TOP_BAR_HEIGHT, EGUI_SECOND_TOP_BAR_HEIGHT, EGUI_LEFT_PANEL_WIDTH};

pub fn egui_controls_ui(
    mut contexts: EguiContexts,
    mut projection_state: ResMut<CameraProjectionState>,
    mut layout_state: ResMut<EguiLayoutState>,
    mut grid_state: ResMut<GridState>,
    mut streams_panel_state: ResMut<StreamsPanelState>,
    loaded_textures: Res<LoadedTextures>,
    mut aspect_ratio_state: ResMut<AspectRatioState>,
    mut texture_mode_state: ResMut<TextureModeState>,
    mut _commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &mut Transform, &mut GlobalTransform, &mut Projection), (With<bevy::prelude::Camera3d>, With<crate::components::RightCamera>)>,
    )>,
) {
    if let Ok(ctx) = contexts.ctx_mut() {
        // Top bar
        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(EGUI_TOP_BAR_HEIGHT)
            .frame(egui::Frame::side_top_panel(&ctx.style())
                .corner_radius(0.0) // Squared corners
                .inner_margin(egui::Margin::ZERO) // Remove inner margin
                .outer_margin(egui::Margin::ZERO)) // Remove outer margin (including bottom)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Web-Video System");
                });
            });
        
        // Get the actual position where the first top bar ends
        // After TopBottomPanel is shown, available_rect() starts below it
        let available_after_top = ctx.available_rect();
        let first_top_bar_end_y = available_after_top.top(); // This is where the first bar actually ends
        
        // Controls panel on the left side
        egui::SidePanel::left("controls_panel")
            .resizable(false)
            .default_width(EGUI_LEFT_PANEL_WIDTH)
            .show(ctx, |ui| {
                // Measure actual content area width
                let left_panel_content_width = ui.available_width();
                layout_state.left_panel_content_width = left_panel_content_width;
                
                // Add scroll area for vertical overflow
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Controls");
                            ui.separator();
                    
                            // Camera controls section
                            ui.label("Camera Controls");
                            
                            // Display projection mode label
                            ui.label("Perspective Camera");
                            
                            ui.separator();
                            
                            // Camera projection info and controls
                            if let Ok((_, _, _, mut projection)) = queries.p0().single_mut() {
                                // Update stored FOV if currently in perspective mode
                                if let Projection::Perspective(ref persp) = *projection {
                                    projection_state.last_perspective_fov = persp.fov;
                                }
                                
                                
                                // FOV control for Perspective projection
                                if let Projection::Perspective(ref mut persp) = *projection {
                                    ui.label("Field of View (FOV)");
                                    
                                    // Convert to degrees for user-friendly display
                                    let mut fov_degrees = persp.fov.to_degrees();
                                    if ui.add(egui::Slider::new(&mut fov_degrees, 30.0..=120.0)
                                        .text("FOV (degrees)")
                                        .step_by(1.0)).changed() {
                                        persp.fov = fov_degrees.to_radians();
                                    }
                                    
                                }
                            }
                            
                            ui.separator();
                            
                            // Grid controls section
                            ui.label("Grid Size (meters)");
                            
                            // X dimension input
                            let mut size_x = grid_state.size_x;
                            if ui.add(egui::DragValue::new(&mut size_x)
                                .range(1..=100)
                                .speed(1)
                                .prefix("X: ")
                                .suffix(" m")).changed() {
                                grid_state.size_x = size_x;
                            }
                            
                            // Z dimension input
                            let mut size_z = grid_state.size_z;
                            if ui.add(egui::DragValue::new(&mut size_z)
                                .range(1..=100)
                                .speed(1)
                                .prefix("Z: ")
                                .suffix(" m")).changed() {
                                grid_state.size_z = size_z;
                            }
                            
                            ui.separator();
                            
                            // Texture Aspect Ratio controls
                            ui.label("Texture Aspect Ratio");
                            
                            ui.horizontal(|ui| {
                                if ui.selectable_label(
                                    aspect_ratio_state.current == AspectRatio::Ratio16_9,
                                    "16:9"
                                ).clicked() {
                                    aspect_ratio_state.current = AspectRatio::Ratio16_9;
                                }
                                
                                if ui.selectable_label(
                                    aspect_ratio_state.current == AspectRatio::Square,
                                    "Square"
                                ).clicked() {
                                    aspect_ratio_state.current = AspectRatio::Square;
                                }
                            });
                            
                            ui.separator();
                            
                            // Texture Mode controls
                            ui.label("Texture Mode");
                            
                            ui.horizontal(|ui| {
                                if ui.selectable_label(
                                    texture_mode_state.current == TextureMode::Normal,
                                    "Normal"
                                ).clicked() {
                                    texture_mode_state.current = TextureMode::Normal;
                                }
                                
                                if ui.selectable_label(
                                    texture_mode_state.current == TextureMode::Stretch,
                                    "Stretch"
                                ).clicked() {
                                    texture_mode_state.current = TextureMode::Stretch;
                                }
                            });
                        }); // Close vertical
                    }); // Close ScrollArea
            }); // Close SidePanel
        
        // Second top bar (starts at x=200, fills to right panel, right under first top bar)
        // SOLUTION: After SidePanels are shown, available_rect() gives the content area (excluding panels)
        // available_rect().left() gives us the ACTUAL position where the left panel ends (includes frame borders)
        // For the right panel, we mirror the left panel's total width (including borders) for symmetry
        let available_rect = ctx.available_rect(); // Content area after panels
        let viewport_rect = ctx.viewport_rect();
        
        // Get the actual left panel end position (includes frame borders, ~38px extra)
        let left_panel_end_x = available_rect.left(); // Actual position where left panel ends
        
        // Calculate right panel start position: mirror the left panel's total width
        // If left panel ends at 238.03 (200px content + 38.03px borders), 
        // right panel should start at viewport_right - 238.03 for symmetry
        let left_panel_total_width = left_panel_end_x; // Total width from 0 to left panel end
        let calculated_right_panel_start = viewport_rect.right() - left_panel_total_width;
        
        // Store actual panel positions for camera viewport calculation
        layout_state.left_panel_end_x = left_panel_end_x; // Actual position where left panel ends (includes frame borders)
        layout_state.right_panel_start_x = calculated_right_panel_start; // Right panel starts here (mirrors left panel width)
        layout_state.top_bars_height = EGUI_TOP_BAR_HEIGHT + EGUI_SECOND_TOP_BAR_HEIGHT;
        layout_state.bottom_bar_height = EGUI_SECOND_TOP_BAR_HEIGHT; // Bottom bar height
        
        // Calculate exact width: from left panel end to right edge of window (for testing)
        // Extended to the right side of the window, not stopping at inspector panel
        let second_bar_width = (viewport_rect.right() - left_panel_end_x).max(0.0);
        let second_bar_height = EGUI_SECOND_TOP_BAR_HEIGHT; // Match first top bar height
        
        // Position the second bar exactly where the first bar ends (no gap)
        // Use the actual panel positions from available_rect for accurate positioning
        let second_bar_rect = egui::Rect::from_min_size(
            egui::pos2(left_panel_end_x, first_top_bar_end_y),
            egui::vec2(second_bar_width, second_bar_height)
        );
        
        egui::Area::new(egui::Id::new("second_top_bar"))
            .fixed_pos(second_bar_rect.min)
            .constrain(true)
            .show(ctx, |ui| {
                // Allocate rect to intercept clicks and block 3D world input
                let _response = ui.allocate_rect(second_bar_rect, egui::Sense::click());
                
                // Paint the background directly to match panel fill (exact size, no Frame expansion)
                ui.painter().rect_filled(second_bar_rect, 0.0, ui.style().visuals.panel_fill);
                
                // Set clip rect to hard-constrain content to exactly 30px height (prevents overflow)
                ui.set_clip_rect(second_bar_rect);
                
                // Allocate UI at the exact rect position to constrain content within the bar
                #[allow(deprecated)]
                ui.allocate_ui_at_rect(second_bar_rect, |ui| {
                    // Constrain height to exactly 30px to match first top bar
                    ui.set_max_height(second_bar_height);
                    ui.set_min_height(second_bar_height);
                    
                    // Remove ALL margins, padding, and spacing inside the bar
                    ui.spacing_mut().button_padding = egui::vec2(4.0, 0.0); // Reduced vertical padding to make button 4px smaller
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0); // No spacing at all
                    ui.spacing_mut().window_margin = egui::Margin::ZERO; // No window margin
                    ui.spacing_mut().menu_margin = egui::Margin::ZERO; // No menu margin
                    
                    // Add 2px vertical offset to push button down
                    ui.add_space(2.0);
                    // Use the same layout approach as the first top bar - no margins
                    ui.horizontal(|ui| {
                        // Add 5px left margin for the button
                        ui.add_space(5.0);
                        // Button with normal frame to make it visible (not frame(false))
                        if ui.button("Workspace").clicked() {
                            streams_panel_state.is_visible = false;
                        }
                        // Add spacing between buttons
                        ui.add_space(5.0);
                        // Streams button with same style
                        if ui.button("Streams").clicked() {
                            streams_panel_state.is_visible = true;
                        }
                    });
                });
            });
        
        // Bottom bar - positioned at the bottom, between the two sidebars, under the 3D world
        let viewport_rect_for_bottom = ctx.viewport_rect();
        let bottom_bar_height = EGUI_SECOND_TOP_BAR_HEIGHT; // Same height as second top bar
        let bottom_bar_y = viewport_rect_for_bottom.bottom() - bottom_bar_height;
        
        let bottom_bar_rect = egui::Rect::from_min_size(
            egui::pos2(left_panel_end_x, bottom_bar_y),
            egui::vec2(second_bar_width, bottom_bar_height)
        );
        
        egui::Area::new(egui::Id::new("bottom_bar"))
            .fixed_pos(bottom_bar_rect.min)
            .constrain(true)
            .show(ctx, |ui| {
                // Allocate rect to intercept clicks and block 3D world input
                let _response = ui.allocate_rect(bottom_bar_rect, egui::Sense::click());
                
                // Paint the background directly to match panel fill (exact size, no Frame expansion)
                ui.painter().rect_filled(bottom_bar_rect, 0.0, ui.style().visuals.panel_fill);
                
                // Set clip rect to hard-constrain content to exactly the bar height
                ui.set_clip_rect(bottom_bar_rect);
                
                // Allocate UI at the exact rect position to constrain content within the bar
                #[allow(deprecated)]
                ui.allocate_ui_at_rect(bottom_bar_rect, |ui| {
                    // Constrain height to exactly match the bar height
                    ui.set_max_height(bottom_bar_height);
                    ui.set_min_height(bottom_bar_height);
                    
                    // Remove ALL margins, padding, and spacing inside the bar
                    ui.spacing_mut().button_padding = egui::vec2(4.0, 0.0); // Reduced vertical padding to make button 4px smaller
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0); // No spacing at all
                    ui.spacing_mut().window_margin = egui::Margin::ZERO; // No window margin
                    ui.spacing_mut().menu_margin = egui::Margin::ZERO; // No menu margin
                    
                    // Add Inspector toggle button
                    ui.add_space(2.0); // Add 2px vertical offset (couple pixels down)
                    // Use the same layout approach as the second top bar - no margins
                    ui.horizontal(|ui| {
                        // Add 5px left margin for the button
                        ui.add_space(5.0);
                        // Button with normal frame to make it visible (not frame(false))
                        if ui.button("Inspector").clicked() {
                            layout_state.inspector_collapsed = !layout_state.inspector_collapsed;
                        }
                    });
                });
            });
        
        // Inspector panel on the right side - rendered AFTER bars as Area to appear on top
        // Set width to match left panel's total width (including borders) for symmetry
        // Only show if not collapsed (toggled by button in bottom bar)
        if !layout_state.inspector_collapsed {
            let viewport_rect = ctx.viewport_rect();
            let inspector_width = left_panel_total_width;
            let inspector_x = viewport_rect.right() - inspector_width;
            let inspector_y = 22.0; // Start 22px from top (below top bars)
            let inspector_height = viewport_rect.height() - inspector_y;
            
            let inspector_rect = egui::Rect::from_min_size(
                egui::pos2(inspector_x, inspector_y),
                egui::vec2(inspector_width, inspector_height)
            );
            
            egui::Area::new(egui::Id::new("inspector_panel"))
                .fixed_pos(inspector_rect.min)
                .constrain(true)
                .order(egui::Order::Foreground) // Ensure it renders on top
                .show(ctx, |ui| {
                    // Allocate rect to intercept clicks
                    let _response = ui.allocate_rect(inspector_rect, egui::Sense::click());
                    
                    // Paint the background
                    ui.painter().rect_filled(inspector_rect, 0.0, ui.style().visuals.panel_fill);
                    
                    // Draw left border to match the left panel's border
                    let border_stroke = ui.style().visuals.widgets.noninteractive.bg_stroke;
                    let left_edge_start = egui::pos2(inspector_rect.left(), inspector_rect.top());
                    let left_edge_end = egui::pos2(inspector_rect.left(), inspector_rect.bottom());
                    ui.painter().line_segment([left_edge_start, left_edge_end], border_stroke);
                    
                    // Set clip rect to constrain content
                    ui.set_clip_rect(inspector_rect);
                    
                    // Allocate UI at the exact rect position
                    #[allow(deprecated)]
                    ui.allocate_ui_at_rect(inspector_rect, |ui| {
                        // Measure actual content area width (accounting for frame)
                        let right_panel_content_width = ui.available_width();
                        layout_state.right_panel_content_width = right_panel_content_width;
                        
                        // Calculate available height for the tracks section (1/3 of inspector height)
                        let inspector_height = inspector_rect.height();
                        let tracks_section_height = inspector_height / 3.0;
                        
                        ui.vertical(|ui| {
                            ui.heading("Inspector");
                            ui.separator();
                            
                            // Tracks section with collapsible header
                            egui::CollapsingHeader::new("Tracks")
                                .default_open(true)
                                .show(ui, |ui| {
                                    // Constrain the height to 1/3 of inspector
                                    ui.set_max_height(tracks_section_height);
                                    
                                    if loaded_textures.textures.is_empty() {
                                        ui.label("No textures loaded");
                                    } else {
                                        egui::ScrollArea::vertical()
                                            .max_height(tracks_section_height - 30.0) // Reserve space for header
                                            .show(ui, |ui| {
                                                for texture_path in &loaded_textures.textures {
                                                    ui.horizontal(|ui| {
                                                        ui.label("📄"); // Simple icon
                                                        ui.label(texture_path);
                                                    });
                                                }
                                            });
                                    }
                                });
                            
                            ui.separator();
                        });
                    });
                });
        }
        
        // Streams panel - covers the 3D viewport when visible
        if streams_panel_state.is_visible {
            let viewport_rect = ctx.viewport_rect();
            let viewport_x = layout_state.left_panel_end_x;
            let viewport_y = layout_state.top_bars_height;
            // Adjust width based on inspector visibility: extend to right edge if inspector is hidden
            let viewport_right_edge = if layout_state.inspector_collapsed {
                viewport_rect.right() // Extend to right edge of window when inspector is hidden
            } else {
                layout_state.right_panel_start_x // Stop at inspector when visible
            };
            let viewport_width = viewport_right_edge - layout_state.left_panel_end_x;
            let viewport_height = viewport_rect.height() - layout_state.top_bars_height;
            
            let streams_panel_rect = egui::Rect::from_min_size(
                egui::pos2(viewport_x, viewport_y),
                egui::vec2(viewport_width.max(0.0), viewport_height.max(0.0))
            );
            
            egui::Area::new(egui::Id::new("streams_panel"))
                .fixed_pos(streams_panel_rect.min)
                .constrain(true)
                .interactable(true)
                .order(egui::Order::Foreground) // Render on top instantly, no transitions
                .show(ctx, |ui| {
                    // Allocate rect to intercept clicks and block 3D world input
                    let _response = ui.allocate_rect(streams_panel_rect, egui::Sense::click());
                    
                    // Paint background to fully cover the 3D viewport - use fixed color for instant appearance
                    let panel_color = ui.style().visuals.panel_fill;
                    ui.painter().rect_filled(streams_panel_rect, 0.0, panel_color);
                    
                    // Set clip rect to constrain content
                    ui.set_clip_rect(streams_panel_rect);
                    
                    // Allocate UI at the exact rect position
                    #[allow(deprecated)]
                    ui.allocate_ui_at_rect(streams_panel_rect, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Streams Panel");
                            ui.separator();
                            ui.label("This panel covers the 3D viewport.");
                        });
                    });
                });
        }
    }
}
