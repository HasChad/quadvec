use macroquad::input::{KeyCode, is_key_pressed};

use crate::{
    drawing::App,
    ui::{draw_settings::DrawSettings, quit_ui::QuitUI},
};

pub struct UI {
    draw_settings: DrawSettings,
    pub quit_ui: QuitUI,
}

impl UI {
    pub fn new() -> Self {
        let draw_settings = DrawSettings::new();
        let quit_ui = QuitUI::new();

        Self {
            draw_settings,
            quit_ui,
        }
    }

    pub fn render_ui(self: &mut Self, app: &mut App) {
        egui_macroquad::ui(|ctx| {
            app.can_draw = !ctx.wants_pointer_input();

            if is_key_pressed(KeyCode::Escape) {
                self.quit_ui.visible = true;
            }

            self.draw_settings.ui(ctx, app);
            self.quit_ui.ui(ctx);
        })
    }
}
