use lyon::geom::{Angle, Vector};
use lyon::math::point;
use lyon::path::{Path, Winding};
use macroquad::prelude::*;

use crate::drawing::{App, lyon_ops::*};

pub fn ellipse_draw(mouse_pos: Vec2, app: &mut App) {
    if is_mouse_button_pressed(MouseButton::Left) {
        app.current_line.push(mouse_pos);
        app.current_line.push(mouse_pos);
    };

    if is_mouse_button_down(MouseButton::Left) && !app.current_line.is_empty() {
        app.current_line[1] = mouse_pos;
    }

    if is_mouse_button_released(MouseButton::Left) {
        if app.current_line.len() > 1 {
            ellipse_mesh(app);
        }

        app.current_line.clear();
    }
}

pub fn ellipse_prew(app: &App) {
    if app.current_line.len() == 2 {
        let mut builder = Path::builder();

        let p1 = app.current_line[0];
        let p2 = app.current_line[1];

        let center = (p1 + p2) * 0.5;

        builder.add_ellipse(
            point(center.x, center.y),
            Vector::new((p2.x - p1.x) / 2.0, (p2.y - p1.y) / 2.0),
            Angle::zero(),
            Winding::Positive,
        );

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

fn ellipse_mesh(app: &mut App) {
    app.lines.push(vec![]);

    let mut builder = Path::builder();

    let p1 = app.current_line[0];
    let p2 = app.current_line[1];

    let center = (p1 + p2) * 0.5;

    builder.add_ellipse(
        point(center.x, center.y),
        Vector::new((p2.x - p1.x) / 2.0, (p2.y - p1.y) / 2.0),
        Angle::zero(),
        Winding::Positive,
    );

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
