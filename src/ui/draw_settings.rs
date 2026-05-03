use egui_macroquad::egui::{self, Color32, Pos2, RichText, Slider};

use crate::drawing::{App, DrawStyle};

pub struct DrawSettings {
    pub sides: u32,
    pub rotation: f32,
}

impl DrawSettings {
    pub fn new() -> Self {
        Self {
            sides: 6,
            rotation: 0.0,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, state: &mut App) {
        egui::Window::new("Toolbar")
            .fixed_pos(Pos2::new(5.0, 5.0))
            .resizable(false)
            .movable(false)
            .frame(egui::Frame::window(&ctx.style()).shadow(egui::Shadow::NONE))
            .show(ctx, |ui| {
                egui::Grid::new("color_grid")
                    .num_columns(2)
                    .spacing([5.0, 5.0])
                    .striped(true)
                    .show(ui, |ui| {
                        // Brush Size
                        ui.label(RichText::new("Brush Size:").color(Color32::WHITE));
                        ui.add(
                            Slider::new(&mut state.brush_size, 1.0..=30.0)
                                .trailing_fill(true)
                                .step_by(0.1)
                                .text_color(Color32::WHITE),
                        );
                        ui.end_row();

                        // Brush Color
                        let mut egui_color: [u8; 3] = [
                            255.min((state.brush_color.r * 255.0) as u8),
                            255.min((state.brush_color.g * 255.0) as u8),
                            255.min((state.brush_color.b * 255.0) as u8),
                        ];
                        ui.label(RichText::new("Brush Color:").color(Color32::WHITE));
                        if ui.color_edit_button_srgb(&mut egui_color).changed() {
                            state.brush_color.r = egui_color[0] as f32 / 255.0;
                            state.brush_color.g = egui_color[1] as f32 / 255.0;
                            state.brush_color.b = egui_color[2] as f32 / 255.0;
                        }
                        ui.end_row();

                        // Background Color
                        let mut egui_color: [u8; 3] = [
                            255.min((state.bg_color.r * 255.0) as u8),
                            255.min((state.bg_color.g * 255.0) as u8),
                            255.min((state.bg_color.b * 255.0) as u8),
                        ];
                        ui.label(RichText::new("BG Color:").color(Color32::WHITE));
                        if ui.color_edit_button_srgb(&mut egui_color).changed() {
                            state.bg_color.r = egui_color[0] as f32 / 255.0;
                            state.bg_color.g = egui_color[1] as f32 / 255.0;
                            state.bg_color.b = egui_color[2] as f32 / 255.0;
                        }
                        ui.end_row();

                        // Draw Style
                        egui::ComboBox::from_id_salt(egui::Id::new("style_picker"))
                            .selected_text(format!("{:?}", state.style))
                            .width(80.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut state.style, DrawStyle::Brush, "Brush");
                                ui.selectable_value(&mut state.style, DrawStyle::Line, "Line");
                                ui.selectable_value(&mut state.style, DrawStyle::Curve, "Curve");
                                ui.selectable_value(&mut state.style, DrawStyle::Arrow, "Arrow");
                                ui.selectable_value(&mut state.style, DrawStyle::Rect, "Rect");
                                ui.selectable_value(&mut state.style, DrawStyle::Circle, "Circle");
                                ui.selectable_value(
                                    &mut state.style,
                                    DrawStyle::Ellipse,
                                    "Ellipse",
                                );
                                ui.selectable_value(&mut state.style, DrawStyle::Poly, "Poly");
                            });

                        // Outline Toggle
                        if state.style == DrawStyle::Rect
                            || state.style == DrawStyle::Circle
                            || state.style == DrawStyle::Ellipse
                            || state.style == DrawStyle::Poly
                        {
                            ui.checkbox(&mut state.is_outline, "Outline");
                        }
                        ui.end_row();

                        // POLYGON SETTINGS
                        if state.style == DrawStyle::Poly {
                            // ui.heading("- Polygon Settings -");
                            ui.end_row();

                            ui.label(RichText::new("Edge Count:").color(Color32::WHITE));
                            ui.add(
                                Slider::new(&mut state.draw_settings.sides, 3..=20)
                                    .trailing_fill(true)
                                    .step_by(0.1),
                            );
                            ui.end_row();

                            ui.label(RichText::new("Rotation:").color(Color32::WHITE));
                            ui.add(
                                Slider::new(&mut state.draw_settings.rotation, 0.0..=360.0)
                                    .trailing_fill(true)
                                    .step_by(0.1),
                            );
                            ui.end_row();
                        }

                        // ui.label(
                        //     RichText::new(format!("fps: {:.0}", get_fps())).color(Color32::WHITE),
                        // );
                    });
            });
    }
}
