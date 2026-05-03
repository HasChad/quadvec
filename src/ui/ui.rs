use crate::{
    drawing::DrawState,
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

    pub fn render_ui(self: &mut Self, draw_state: &mut DrawState) {
        egui_macroquad::ui(|ctx| {
            draw_state.can_draw = !ctx.wants_pointer_input();

            self.draw_settings.ui(ctx, draw_state);
            self.quit_ui.ui(ctx);
        })
    }
}
