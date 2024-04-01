use eframe::egui;
use lldb::{LaunchFlags, SBDebugger, SBError, SBLaunchInfo};

fn main() -> Result<(), eframe::Error> {
    SBDebugger::initialize();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    let mut app = <App>::default();
    let debugger = SBDebugger::create(false);
    debugger.enable_log("lldb", &["default"]);
    app.debugger = Some(debugger);
    let ret = eframe::run_native("lldb-gui", options, Box::new(|_cc| Box::new(app)));
    SBDebugger::terminate();
    ret
}

#[derive(Default)]
struct App {
    target_path: Option<String>,
    debugger: Option<lldb::SBDebugger>,
    target: Option<lldb::SBTarget>,
    show_error: bool,
    error: Option<SBError>,
}

impl App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Target", |ui| {
                    if ui.button("New").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.target_path = Some(path.display().to_string());
                            match self
                                .debugger
                                .as_ref()
                                .expect("debugger should be set up")
                                .create_target(&path.display().to_string(), None, None, false)
                            {
                                Ok(target) => self.target = Some(target),
                                Err(e) => {
                                    self.show_error = true;
                                    self.error = Some(e);
                                }
                            };
                        }
                        ui.close_menu();
                    }
                    if ui.button("Connect").clicked() {
                        ui.close_menu();
                    }
                });
                if let Some(target) = &self.target {
                    if ui.button("Run").clicked() {
                        let launch_info = SBLaunchInfo::new();
                        // launch_info.set_launch_flags(LaunchFlags::LAUNCH_IN_SEPARATE_PROCESS_GROUP);
                        match target.launch(launch_info) {
                            Ok(_process) => (),
                            Err(e) => {
                                self.show_error = true;
                                self.error = Some(e);
                                ()
                            }
                        };
                    }
                }
            });

            if let Some(target_path) = &self.target_path {
                ui.horizontal(|ui| {
                    ui.label("Target:");
                    ui.monospace(target_path);
                });
            }

            if self.show_error {
                egui::Window::new("Error").show(ctx, |ui| {
                    if ui.button("Close").clicked() {
                        self.show_error = false;
                    }
                    ui.label(
                        self.error
                            .as_ref()
                            .expect("error show be present")
                            .to_string(),
                    );
                });
            }
        });
    }
}
