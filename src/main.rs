// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cli;
mod debugger;
mod defines;
mod resources;

use anyhow::{bail, Result};
use clap::Parser;

use crate::cli::Cli;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .event_format(tracing_subscriber::fmt::format().pretty())
        .init();

    let cli = Cli::parse();

    debugger::initialize();

    let target = if let Some(executable) = cli.executable {
        debugger::run(&executable, !cli.no_lldbinit, cli.args)?
    } else if let Some(pid) = cli.attach_pid {
        debugger::attach_pid(pid, !cli.no_lldbinit)?
    } else if let Some(name) = cli.attach_name {
        debugger::attach_name(&name, !cli.no_lldbinit)?
    } else {
        // Should not happen because we require at least one parameter as cli option.
        bail!("required debug session parameter missing")
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id(format!(
                "{}-{}",
                crate::defines::APP_NAME,
                env!("VERGEN_GIT_SHA")
            ))
            .with_icon(resources::load_icon()),

        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native(
        crate::defines::APP_NAME,
        options,
        Box::new(|cc| Box::new(app::App::new(cc, target))),
    )
    .unwrap();

    debugger::terminate();

    Ok(())
}
