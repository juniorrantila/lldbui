#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// hide console window on Windows in release
use clap::{ArgGroup, Parser};
use lldb::{LaunchFlags, SBAttachInfo, SBDebugger, SBLaunchInfo};
use std::os::fd::AsRawFd;
use tempfile::NamedTempFile;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("target").required(true).args(&["executable", "attach_pid", "attach_name"]),
))]
struct Cli {
    executable: Option<String>,

    /// Tells the debugger to attach to a process with the given pid.
    #[arg(short = 'p', long)]
    attach_pid: Option<u64>,

    /// Tells the debugger to attach to a process with the given name.
    #[arg(short = 'n', long)]
    attach_name: Option<String>,
}

fn main() -> eframe::Result<()> {
    let cli = Cli::parse();

    SBDebugger::initialize();
    let debugger = SBDebugger::create(false);
    debugger.enable_log("lldb", &["default"]);
    debugger.set_asynchronous(true);

    let target = if let Some(executable) = cli.executable {
        let target = debugger
            .create_target(&executable, None, None, false)
            .unwrap();
        let launch_info = SBLaunchInfo::new();
        launch_info.set_launch_flags(LaunchFlags::STOP_AT_ENTRY);
        target.launch(launch_info).unwrap();
        target
    } else {
        let target = debugger.create_target_simple("").unwrap();
        let attach_info = if let Some(pid) = cli.attach_pid {
            SBAttachInfo::new_with_pid(pid)
        } else if let Some(name) = cli.attach_name {
            SBAttachInfo::new_with_path(&name, true, false)
        } else {
            panic!("should not happen!")
        };
        target.attach(attach_info).unwrap();
        target
    };

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
