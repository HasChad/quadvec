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

    pub fn ui(&mut self, ctx: &egui::Context, app: &mut App) {
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
                            Slider::new(&mut app.brush_size, 1.0..=30.0)
                                .trailing_fill(true)
                                .step_by(0.1)
                                .text_color(Color32::WHITE),
                        );
                        ui.end_row();

                        // Brush Color
                        let mut egui_color: [u8; 3] = [
                            255.min((app.brush_color.r * 255.0) as u8),
                            255.min((app.brush_color.g * 255.0) as u8),
                            255.min((app.brush_color.b * 255.0) as u8),
                        ];
                        ui.label(RichText::new("Brush Color:").color(Color32::WHITE));
                        if ui.color_edit_button_srgb(&mut egui_color).changed() {
                            app.brush_color.r = egui_color[0] as f32 / 255.0;
                            app.brush_color.g = egui_color[1] as f32 / 255.0;
                            app.brush_color.b = egui_color[2] as f32 / 255.0;
                        }
                        ui.end_row();

                        // Background Color
                        let mut egui_color: [u8; 3] = [
                            255.min((app.bg_color.r * 255.0) as u8),
                            255.min((app.bg_color.g * 255.0) as u8),
                            255.min((app.bg_color.b * 255.0) as u8),
                        ];
                        ui.label(RichText::new("BG Color:").color(Color32::WHITE));
                        if ui.color_edit_button_srgb(&mut egui_color).changed() {
                            app.bg_color.r = egui_color[0] as f32 / 255.0;
                            app.bg_color.g = egui_color[1] as f32 / 255.0;
                            app.bg_color.b = egui_color[2] as f32 / 255.0;
                        }
                        ui.end_row();

                        // Draw Style
                        egui::ComboBox::from_id_salt(egui::Id::new("style_picker"))
                            .selected_text(format!("{:?}", app.style))
                            .width(80.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut app.style, DrawStyle::Brush, "Brush");
                                ui.selectable_value(&mut app.style, DrawStyle::Line, "Line");
                                ui.selectable_value(&mut app.style, DrawStyle::Curve, "Curve");
                                ui.selectable_value(&mut app.style, DrawStyle::Arrow, "Arrow");
                                ui.selectable_value(&mut app.style, DrawStyle::Rect, "Rect");
                                ui.selectable_value(&mut app.style, DrawStyle::Circle, "Circle");
                                ui.selectable_value(&mut app.style, DrawStyle::Ellipse, "Ellipse");
                                ui.selectable_value(&mut app.style, DrawStyle::Poly, "Poly");
                            });

                        // Outline Toggle
                        if app.style == DrawStyle::Rect
                            || app.style == DrawStyle::Circle
                            || app.style == DrawStyle::Ellipse
                            || app.style == DrawStyle::Poly
                        {
                            ui.checkbox(&mut app.is_outline, "Outline");
                        }
                        ui.end_row();

                        // POLYGON SETTINGS
                        if app.style == DrawStyle::Poly {
                            // ui.heading("- Polygon Settings -");
                            ui.end_row();

                            ui.label(RichText::new("Edge Count:").color(Color32::WHITE));
                            ui.add(
                                Slider::new(&mut app.draw_settings.sides, 3..=20)
                                    .trailing_fill(true)
                                    .step_by(0.1),
                            );
                            ui.end_row();

                            ui.label(RichText::new("Rotation:").color(Color32::WHITE));
                            ui.add(
                                Slider::new(&mut app.draw_settings.rotation, 0.0..=360.0)
                                    .trailing_fill(true)
                                    .step_by(0.1),
                            );
                            ui.end_row();
                        }

                        // Outline Toggle
                        ui.end_row();
                        if ui.button("Clear Canvas").clicked() {
                            app.clear_canvas();
                        }
                        ui.end_row();

                        // ui.label(
                        //     RichText::new(format!("fps: {:.0}", get_fps())).color(Color32::WHITE),
                        // );
                    });
            });
    }
}
