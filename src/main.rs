#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use lldb::SBDebugger;

fn main() -> eframe::Result<()> {
    SBDebugger::initialize();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "lldb-gui",
        native_options,
        Box::new(|_| Box::new(lldb_gui::App::new())),
    )?;

    SBDebugger::terminate();

    Ok(())
}
