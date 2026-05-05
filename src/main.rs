#![windows_subsystem = "windows"]

use macroquad::prelude::*;

mod app_settings;
mod drawing;
mod ui;

use crate::{drawing::App, ui::ui::UI};
use app_settings::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut freedom_camera = FreedomCamera2D::new();
    let mut app = App::new();
    let mut ui = UI::new();

    'app: loop {
        freedom_camera.update();
        let world_mpos = freedom_camera.world_mpos();

        if ui.quit_ui.quit_app {
            break 'app;
        }

        if app.can_draw {
            app.drawing(world_mpos);
        }

        app.inputs();

        // ! draw
        set_camera(&freedom_camera.camera);
        clear_background(app.bg_color);

        if app.draw_settings.enable_grid {
            draw_grid(freedom_camera.camera.target);
        }

        ui.render_ui(&mut app);

        app.line_render();
        app.current_style_preview();

        draw_circle_lines(
            world_mpos.x,
            world_mpos.y,
            app.brush_size / 2.0,
            1.0,
            app.brush_color,
        );

        egui_macroquad::draw();

        next_frame().await
    }
}

const CELL_SIZE: i32 = 50;
const LINE_COUNT: i32 = 300;

fn draw_grid(center: Vec2) {
    let grid_size = (LINE_COUNT * CELL_SIZE) as f32;

    let offset_x = center.x - grid_size / 2.0;
    let offset_y = center.y - grid_size / 2.0;

    for i in 0..=LINE_COUNT {
        let x = offset_x + (i * CELL_SIZE) as f32 - center.x % CELL_SIZE as f32;
        let y = offset_y + (i * CELL_SIZE) as f32 - center.y % CELL_SIZE as f32;
        // vertical lines
        draw_line(x, offset_y, x, offset_y + grid_size, 1.0, DARKGRAY);
        // horizontal lines
        draw_line(offset_x, y, offset_x + grid_size, y, 1.0, DARKGRAY);
    }
}
