use macroquad::prelude::*;

use crate::{
    drawing::styles::{
        arrow_style::{arrow_draw, arrow_prew},
        brush_style::{brush_draw, brush_prew},
        circle_style::{circle_draw, circle_prew},
        curve_style::{curve_draw, curve_prew},
        ellipse_style::{ellipse_draw, ellipse_prew},
        line_style::{line_draw, line_prew},
        poly_style::{poly_draw, poly_prew},
        rectangle_style::{rect_draw, rect_prew},
    },
    ui::draw_settings::DrawSettings,
};

pub mod line_smoothing;
pub mod lyon_ops;
pub mod styles;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DrawStyle {
    Brush,
    Line,
    Curve,
    Arrow,
    Rect,
    Circle,
    Ellipse,
    Poly,
}

pub struct App {
    pub style: DrawStyle,
    pub lines: Vec<Vec<Mesh>>,
    pub redo_save: Vec<Vec<Mesh>>,
    pub current_line: Vec<Vec2>,
    pub current_line_raw: Vec<Vec2>,
    pub brush_color: Color,
    pub brush_size: f32,
    pub bg_color: Color,
    pub can_draw: bool,
    pub is_outline: bool,
    pub draw_settings: DrawSettings,
}

impl App {
    pub fn new() -> Self {
        App {
            style: DrawStyle::Brush,
            lines: vec![],
            redo_save: vec![],
            current_line: vec![],
            current_line_raw: vec![],
            brush_color: WHITE,
            brush_size: 5.0,
            bg_color: BLACK,
            can_draw: true,
            is_outline: false,
            draw_settings: DrawSettings::new(),
        }
    }

    pub fn inputs(self: &mut Self) {
        if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl) {
            self.undo();
        }

        if is_key_pressed(KeyCode::X) && is_key_down(KeyCode::LeftControl) {
            self.redo();
        }

        if is_key_pressed(KeyCode::C) {
            self.clear_canvas();
        }
    }

    fn undo(self: &mut Self) {
        if let Some(line) = self.lines.pop() {
            self.redo_save.push(line);
        }
    }

    fn redo(self: &mut Self) {
        if let Some(line) = self.redo_save.pop() {
            self.lines.push(line);
        }
    }

    pub fn clear_canvas(self: &mut Self) {
        self.lines.clear();
        self.redo_save.clear();
    }

    pub fn drawing(self: &mut Self, mouse_pos: Vec2) {
        match self.style {
            DrawStyle::Brush => brush_draw(mouse_pos, self),
            DrawStyle::Line => line_draw(mouse_pos, self),
            DrawStyle::Curve => curve_draw(mouse_pos, self),
            DrawStyle::Arrow => arrow_draw(mouse_pos, self),
            DrawStyle::Rect => rect_draw(mouse_pos, self),
            DrawStyle::Circle => circle_draw(mouse_pos, self),
            DrawStyle::Ellipse => ellipse_draw(mouse_pos, self),
            DrawStyle::Poly => poly_draw(mouse_pos, self),
        }
    }

    pub fn current_style_preview(self: &Self) {
        match self.style {
            DrawStyle::Brush => brush_prew(self),
            DrawStyle::Line => line_prew(self),
            DrawStyle::Curve => curve_prew(self),
            DrawStyle::Arrow => arrow_prew(self),
            DrawStyle::Rect => rect_prew(self),
            DrawStyle::Circle => circle_prew(self),
            DrawStyle::Ellipse => ellipse_prew(self),
            DrawStyle::Poly => poly_prew(self),
        }
    }

    pub fn line_render(self: &Self) {
        for lines in &self.lines {
            for mesh in lines {
                draw_mesh(&mesh);
            }
        }
    }
}
