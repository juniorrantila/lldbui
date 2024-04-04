#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use clap::Parser;
use lldb::{LaunchFlags, SBDebugger, SBLaunchInfo};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    target: String,
}

fn main() -> eframe::Result<()> {
    let cli = Cli::parse();

    SBDebugger::initialize();

    let debugger = SBDebugger::create(false);
    debugger.set_asynchronous(true);

    let target = debugger
        .create_target(&cli.target, None, None, false)
        .unwrap();

    let launch_info = SBLaunchInfo::new();
    launch_info.set_launch_flags(LaunchFlags::STOP_AT_ENTRY);

    target.launch(launch_info).unwrap();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "lldbui",
        native_options,
        Box::new(|cc| Box::new(lldbui::App::new(cc, target))),
    )?;

    SBDebugger::terminate();

    Ok(())
}
