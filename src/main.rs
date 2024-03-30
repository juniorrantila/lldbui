use autocxx::prelude::*;

include_cpp! {
    #include "lldb/API/LLDB.h"
    safety!(unsafe_ffi)
    generate!("lldb::SBDebugger")
}

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let lldb_error = ffi::lldb::SBDebugger::InitializeWithErrorHandling().within_unique_ptr();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native("lldb-gui", options, Box::new(|_cc| Box::<MyApp>::default()))
}

#[derive(Default)]
struct MyApp {
    picked_path: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }
        });
    }
}
