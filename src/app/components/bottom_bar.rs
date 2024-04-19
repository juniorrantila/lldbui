use egui::{
    warn_if_debug_build, widgets::global_dark_light_mode_switch, Align, Layout, RichText, Ui,
};
use lldb::SBDebugger;

use crate::app::App;

pub fn add(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        warn_if_debug_build(ui);
        ui.separator();
        ui.label(RichText::new(format!("{:.2} fps", app.frame_history.fps())).small())
            .on_hover_text(format!(
                "Mean CPU usage: {:.2} ms / frame",
                1e3 * app.frame_history.mean_frame_time()
            ));
        ui.separator();
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            global_dark_light_mode_switch(ui);
            ui.separator();
            ui.label(RichText::new(SBDebugger::version()).small());
            ui.separator();
        });
    });
}
