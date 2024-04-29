// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cli;
mod debug_session;
mod defines;
mod resources;

use anyhow::Result;
use clap::Parser;
use lldb::SBDebugger;

use crate::cli::Cli;
use debug_session::DebugSession;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .event_format(tracing_subscriber::fmt::format().pretty())
        .init();

    let cli = Cli::parse();

    debug_session::initialize();
    let debugger = SBDebugger::create(!cli.no_lldbinit);
    let mut session = DebugSession::new(debugger);

    if let Some(executable) = cli.executable {
        session.run(&executable, cli.args)?
    } else if let Some(pid) = cli.attach_pid {
        session.attach_pid(pid)?
    } else if let Some(name) = cli.attach_name {
        session.attach_name(&name)?
    } else {
        // Should not happen because we require at least one parameter as cli option.
        panic!("required debug session parameters missing")
    };

    let optins = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_icon(resources::load_icon()),
        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native(
        crate::defines::APP_NAME,
        optins,
        Box::new(|cc| Box::new(app::App::new(cc, session))),
    )
    .unwrap();

    debug_session::shutdown();

    Ok(())
}
