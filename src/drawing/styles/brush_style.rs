use lyon::math::point;
use lyon::path::Path;
use macroquad::prelude::*;

use crate::drawing::{App, line_smoothing::line_smoothing, lyon_ops::*};

pub fn brush_draw(mouse_pos: Vec2, app: &mut App) {
    if is_mouse_button_pressed(MouseButton::Left) {
        app.current_line_raw.push(mouse_pos);
        app.current_line.push(mouse_pos);
    };

    if is_mouse_button_down(MouseButton::Left) && !app.current_line_raw.is_empty() {
        let delta = mouse_delta_position();
        if delta.x != 0.0 || delta.y != 0.0 {
            let last = *app.current_line_raw.last().unwrap();
            if last.distance(mouse_pos) >= 2.0 {
                app.current_line_raw.push(mouse_pos);
                realtime_smooth(&app.current_line_raw, &mut app.current_line);
            }
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        if app.current_line.len() > 1 {
            if app.current_line.len() > 2 {
                line_smoothing(&mut app.current_line);
            }
            brush_mesh(app);
        }
        app.current_line.clear();
        app.current_line_raw.clear();
    }
}

pub fn brush_prew(app: &App) {
    let mut prev_last: Option<&Vec2> = None;

    for line_chunk in app.current_line.chunks(350) {
        let mut builder = Path::builder();
        let mut raw_points = vec![];

        if let Some(prev) = prev_last {
            raw_points.push(point(prev.x, prev.y));
        }

        for stroke in line_chunk.iter() {
            raw_points.push(point(stroke.x, stroke.y));
        }

        prev_last = line_chunk.last();

        for (i, point) in raw_points.iter().enumerate() {
            if i == 0 {
                builder.begin(*point);
            } else {
                builder.line_to(*point);
            }

            if i == raw_points.len() - 1 {
                builder.end(false);
            }
        }

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

pub fn brush_mesh(app: &mut App) {
    let mut prev_last: Option<&Vec2> = None;
    app.lines.push(vec![]);

    for line_chunk in app.current_line.chunks(350) {
        let mut builder = Path::builder();
        let mut raw_points = vec![];

        if let Some(prev) = prev_last {
            raw_points.push(point(prev.x, prev.y));
        }

        for stroke in line_chunk.iter() {
            raw_points.push(point(stroke.x, stroke.y));
        }

        prev_last = line_chunk.last();

        for (i, point) in raw_points.iter().enumerate() {
            if i == 0 {
                builder.begin(*point);
                continue;
            }

            builder.line_to(*point);

            if i == raw_points.len() - 1 {
                builder.end(false);
            }
        }

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
}

/// Rebuild smooth points from raw — only recomputes the tail
pub fn realtime_smooth(raw: &[Vec2], smooth: &mut Vec<Vec2>) {
    let n = raw.len();

    // Keep the settled head, only redo the last few points
    let tail_start = n.saturating_sub(6);
    smooth.truncate(tail_start);

    for i in tail_start..n {
        smooth.push(weighted_avg(raw, i));
    }
}

fn weighted_avg(points: &[Vec2], i: usize) -> Vec2 {
    // Gaussian-style [1, 2, 4, 2, 1] kernel
    const WEIGHTS: [(i32, f32); 5] = [(-2, 1.0), (-1, 2.0), (0, 4.0), (1, 2.0), (2, 1.0)];
    let mut sum = Vec2::ZERO;
    let mut total = 0.0f32;

    for (offset, w) in WEIGHTS {
        let j = i as i32 + offset;
        if j >= 0 && (j as usize) < points.len() {
            sum += points[j as usize] * w;
            total += w;
        }
    }

    sum / total
}
