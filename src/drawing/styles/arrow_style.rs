use lyon::math::point;
use lyon::path::Path;
use macroquad::prelude::*;

use crate::drawing::{App, lyon_ops::*};

pub fn arrow_draw(mouse_pos: Vec2, app: &mut App) {
    if is_mouse_button_pressed(MouseButton::Left) {
        app.current_line.push(mouse_pos);
        app.current_line.push(mouse_pos);
    };

    if is_mouse_button_down(MouseButton::Left) && !app.current_line.is_empty() {
        app.current_line[1] = mouse_pos;
    }

    if is_mouse_button_released(MouseButton::Left) {
        if app.current_line.len() > 1 {
            arrow_mesh(app);
        }

        app.current_line.clear();
    }
}

pub fn arrow_prew(app: &App) {
    if app.current_line.len() == 2 {
        let p1 = app.current_line[0];
        let p2 = app.current_line[1];
        let size = app.brush_size;

        let dir = if p2 == p1 { p1 } else { p2 - p1 };

        let norm = dir.normalize();
        let mid = p2 - (norm * 5.0 * size);

        let normal = Vec2::new(-dir.y, dir.x).normalize();

        let end1 = mid + normal * 2.0 * size;
        let end2 = mid - normal * 2.0 * size;

        let mut builder = Path::builder();

        builder.begin(point(p1.x, p1.y));
        builder.line_to(point(mid.x, mid.y));
        builder.end(false);

        let path = builder.build();

        let (geometry, vertices) = LyonOpsLine::new(&path, app.brush_color, app.brush_size);

        let mesh = Mesh {
            vertices: vertices,
            indices: geometry.indices,
            texture: None,
        };

        draw_mesh(&mesh);

        let mut builder = Path::builder();

        builder.begin(point(p2.x, p2.y));
        builder.line_to(point(end1.x, end1.y));
        builder.line_to(point(end2.x, end2.y));
        builder.end(true);

        let path = builder.build();

        let (geometry, vertices) = LyonOpsFill::new(&path, app.brush_color);

        let mesh = Mesh {
            vertices: vertices,
            indices: geometry.indices,
            texture: None,
        };

        draw_mesh(&mesh);
    }
}

fn arrow_mesh(app: &mut App) {
    app.lines.push(vec![]);

    let p1 = app.current_line[0];
    let p2 = app.current_line[1];
    let size = app.brush_size;

    let dir = if p2 == p1 { p1 } else { p2 - p1 };

    let norm = dir.normalize();
    let mid = p2 - (norm * 5.0 * size);

    let normal = Vec2::new(-dir.y, dir.x).normalize();

    let end1 = mid + normal * 2.0 * size;
    let end2 = mid - normal * 2.0 * size;

    let mut builder = Path::builder();

    builder.begin(point(p1.x, p1.y));
    builder.line_to(point(mid.x, mid.y));
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

    let mut builder = Path::builder();

    builder.begin(point(p2.x, p2.y));
    builder.line_to(point(end1.x, end1.y));
    builder.line_to(point(end2.x, end2.y));
    builder.end(true);

    let path = builder.build();

    let (geometry, vertices) = LyonOpsFill::new(&path, app.brush_color);

    let mesh = Mesh {
        vertices: vertices,
        indices: geometry.indices,
        texture: None,
    };

    let last = app.lines.len() - 1;
    app.lines[last].push(mesh);
}
