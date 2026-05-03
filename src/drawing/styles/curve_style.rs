use lyon::math::point;
use lyon::path::Path;
use macroquad::prelude::*;

use crate::drawing::{App, lyon_ops::*};

pub fn curve_draw(mouse_pos: Vec2, app: &mut App) {
    if is_mouse_button_pressed(MouseButton::Left) {
        if app.current_line.len() == 3 {
            curve_mesh(app);

            app.current_line.clear();
        } else {
            app.current_line.push(mouse_pos);
        }
    };

    if is_mouse_button_down(MouseButton::Left) && !app.current_line.is_empty() {
        if app.current_line.len() == 2 {
            app.current_line[1] = mouse_pos;
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        if app.current_line.len() == 2 {
            app.current_line[1] = mouse_pos;
        }
    }

    if app.current_line.len() == 3 {
        app.current_line[1] = mouse_pos;
    }
}

pub fn curve_prew(app: &App) {
    if app.current_line.len() > 1 {
        let p1 = app.current_line[0];
        let p2 = app.current_line[1];
        let p3 = if app.current_line.len() == 3 {
            app.current_line[2]
        } else {
            app.current_line[1]
        };

        let mut builder = Path::builder();

        builder.begin(point(p1.x, p1.y));
        builder.quadratic_bezier_to(point(p3.x, p3.y), point(p2.x, p2.y));
        builder.line_to(point(p2.x, p2.y));
        builder.end(false);

        let path = builder.build();

        let (geometry, vertices) = LyonOpsLine::new(&path, app.brush_color, app.brush_size);

        let mesh = Mesh {
            vertices: vertices,
            indices: geometry.indices,
            texture: None,
        };

        draw_mesh(&mesh);
    }
}

fn curve_mesh(app: &mut App) {
    app.lines.push(vec![]);

    let p1 = app.current_line[0];
    let p2 = app.current_line[1];
    let p3 = if app.current_line.len() == 3 {
        app.current_line[2]
    } else {
        app.current_line[1]
    };

    let mut builder = Path::builder();

    builder.begin(point(p1.x, p1.y));
    builder.quadratic_bezier_to(point(p3.x, p3.y), point(p2.x, p2.y));
    builder.line_to(point(p2.x, p2.y));
    builder.end(false);

    let path = builder.build();

    let (geometry, vertices) = LyonOpsLine::new(&path, app.brush_color, app.brush_size);

    let mesh = Mesh {
        vertices: vertices,
        indices: geometry.indices,
        texture: None,
    };

    let last = app.lines.len() - 1;
    app.lines[last].push(mesh);
}
