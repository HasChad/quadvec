use lyon::math::point;
use lyon::path::{Path, Polygon};
use macroquad::prelude::*;

use crate::drawing::{App, lyon_ops::*};

pub fn poly_draw(mouse_pos: Vec2, app: &mut App) {
    if is_mouse_button_pressed(MouseButton::Left) {
        app.current_line.push(mouse_pos);
        app.current_line.push(mouse_pos);
    };

    if is_mouse_button_down(MouseButton::Left) && !app.current_line.is_empty() {
        app.current_line[1] = mouse_pos;
    }

    if is_mouse_button_released(MouseButton::Left) {
        if app.current_line.len() > 1 {
            poly_mesh(app);
        }

        app.current_line.clear();
    }
}

pub fn poly_prew(app: &App) {
    if app.current_line.len() == 2 {
        let mut builder = Path::builder();

        let p1 = app.current_line[0];
        let p2 = app.current_line[1];

        let center = (p1 + p2) * 0.5;

        let sides = app.draw_settings.sides;
        let rot = app.draw_settings.rotation.to_radians();

        let radius = if (p2.x - p1.x).abs() < (p2.y - p1.y).abs() {
            (p2.x - p1.x).abs() * 0.5
        } else {
            (p2.y - p1.y).abs() * 0.5
        };
        let mut points = vec![];

        for i in 0..=sides {
            let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();
            let ry = -(i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();

            let poly_p = point(center.x + radius * rx, center.y + radius * ry);

            points.push(poly_p);
        }

        builder.add_polygon(Polygon {
            points: &points,
            closed: true,
        });

        let path = builder.build();

        let (geometry, vertices) = if app.is_outline {
            LyonOpsFill::new(&path, app.brush_color)
        } else {
            LyonOpsLine::new(&path, app.brush_color, app.brush_size)
        };

        let mesh = Mesh {
            vertices: vertices,
            indices: geometry.indices,
            texture: None,
        };

        draw_mesh(&mesh);
    }
}

fn poly_mesh(app: &mut App) {
    app.lines.push(vec![]);

    let mut builder = Path::builder();

    let p1 = app.current_line[0];
    let p2 = app.current_line[1];

    let center = (p1 + p2) * 0.5;

    let sides = app.draw_settings.sides;
    let rot = app.draw_settings.rotation.to_radians();

    let radius = if (p2.x - p1.x).abs() < (p2.y - p1.y).abs() {
        (p2.x - p1.x).abs() * 0.5
    } else {
        (p2.y - p1.y).abs() * 0.5
    };
    let mut points = vec![];

    for i in 0..=sides {
        let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();
        let ry = -(i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();

        let poly_p = point(center.x + radius * rx, center.y + radius * ry);

        points.push(poly_p);
    }

    builder.add_polygon(Polygon {
        points: &points,
        closed: true,
    });

    let path = builder.build();

    let (geometry, vertices) = if app.is_outline {
        LyonOpsFill::new(&path, app.brush_color)
    } else {
        LyonOpsLine::new(&path, app.brush_color, app.brush_size)
    };

    let mesh = Mesh {
        vertices: vertices,
        indices: geometry.indices,
        texture: None,
    };

    let last = app.lines.len() - 1;
    app.lines[last].push(mesh);
}
