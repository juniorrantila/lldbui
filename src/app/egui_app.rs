use egui::{CentralPanel, SidePanel, TopBottomPanel};

use crate::app::components;
use crate::app::App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);

        components::close_confirmation(self, ctx);

        TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| components::bottom_bar(self, ui));
        SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                TopBottomPanel::top("process_info")
                    .resizable(true)
                    .show_inside(ui, |ui| components::process_info(self, ui));
                TopBottomPanel::top("threads")
                    .resizable(true)
                    .show_inside(ui, |ui| components::threads(self, ui));
                TopBottomPanel::top("frames")
                    .resizable(true)
                    .show_inside(ui, |ui| components::frames(self, ui));
                TopBottomPanel::top("variables")
                    .resizable(true)
                    .show_inside(ui, |ui| components::variables(self, ui));
                CentralPanel::default().show_inside(ui, |ui| components::breakpoints(self, ui));
            });
        TopBottomPanel::top("top_panel").show(ctx, |ui| components::top_bar(self, ui));
        TopBottomPanel::bottom("console_panel")
            .resizable(true)
            .show(ctx, |ui| {
                components::console_tabs(self, ui);
            });

        CentralPanel::default().show(ctx, |ui| components::source_view(self, ui));
    }
}
