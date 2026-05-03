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
        clear_background(app.bg_color);
        set_camera(&freedom_camera.camera);

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
