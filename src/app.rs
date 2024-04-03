use egui::{widgets, Align, CentralPanel, Layout, SidePanel, TopBottomPanel};
use lldb::{LaunchFlags, SBDebugger, SBError, SBLaunchInfo};

#[derive(Debug, PartialEq)]
enum ConsoleTab {
    Console,
    Stdout,
    Stderr,
}

pub struct App {
    console_tab: ConsoleTab,
    debugger: SBDebugger,
}

impl App {
    pub fn new() -> Self {
        let app = Self {
            console_tab: ConsoleTab::Console,
            debugger: SBDebugger::create(false),
        };
        app.debugger.enable_log("lldb", &["default"]);
        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            console_tab,
            debugger: _,
        } = self;
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Target: ");
                ui.label("Process state: ");
            });
        SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("threads");
            });
        TopBottomPanel::bottom("console_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(console_tab, ConsoleTab::Console, "console");
                    ui.selectable_value(console_tab, ConsoleTab::Stdout, "stdout");
                    ui.selectable_value(console_tab, ConsoleTab::Stderr, "stderr");
                });
                ui.separator();
                ui.label("foo");
            });
        CentralPanel::default().show(ctx, |ui| {
            ui.label("code");
        });
    }
}
