use egui::{CentralPanel, SidePanel, TopBottomPanel};

use crate::app::components;
use crate::app::App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        components::close_confirmation(self, ctx);

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| components::bottom_bar(self, ui));
        SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                components::process_info(self, ui);
                ui.separator();
                components::threads(self, ui);
                ui.separator();
                components::frames(self, ui);
                ui.separator();
                components::variables(self, ui);
            });
        TopBottomPanel::top("top_panel").show(ctx, |ui| components::top_bar(self, ui));
        TopBottomPanel::bottom("console_panel")
            .resizable(true)
            .min_height(150.)
            .show(ctx, |ui| {
                components::console_tabs(self, ui);
            });
        CentralPanel::default().show(ctx, |ui| components::source_view(self, ui));
    }
}
