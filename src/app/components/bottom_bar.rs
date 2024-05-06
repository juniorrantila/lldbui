use egui::{widgets::global_dark_light_mode_switch, Align, Layout, RichText, Ui};
use lldb::SBDebugger;

use crate::app::App;

pub fn add(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        if cfg!(debug_assertions) {
            ui.hyperlink_to(
                RichText::new("lldbui").small(),
                "https://git.sr.ht/~dennis/lldbui",
            );
            ui.label(RichText::new(format!("({})", env!("VERGEN_GIT_SHA"))).small());
            ui.separator();
            ui.label(
                RichText::new("⚠ Debug build ⚠")
                    .small()
                    .color(ui.visuals().warn_fg_color),
            )
            .on_hover_text("egui was compiled with debug assertions enabled.");
            ui.separator();

            ui.label(RichText::new(format!("{:.2} fps", app.frame_history.fps())).small())
                .on_hover_text(format!(
                    "Mean CPU usage: {:.2} ms / frame",
                    1e3 * app.frame_history.mean_frame_time()
                ));
            ui.separator();
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            global_dark_light_mode_switch(ui);
            ui.separator();
            ui.label(RichText::new(SBDebugger::version()).small());
        });
    });
}
