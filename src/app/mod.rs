mod components;
mod egui_app;
mod frame_history;
mod widgets;

use std::str::FromStr;
use std::time::Duration;
use std::{
    sync::atomic::AtomicBool, sync::atomic::Ordering, sync::Arc, thread, thread::JoinHandle,
};

use eframe::CreationContext;
use egui::{style::ScrollStyle, Context};
use lldb::{SBEvent, SBListener};
use tracing::debug;

use crate::app::frame_history::FrameHistory;
use crate::debug_session::DebugSession;
use crate::resources;

#[derive(PartialEq)]
enum ConsoleTab {
    Console,
    Stdout,
    Stderr,
    Log,
}

#[derive(PartialEq)]
enum VariablesTab {
    Locals,
    Statics,
    Arguments,
    Registers,
}

#[derive(PartialEq)]
enum BreakpointsTab {
    Breakpoints,
    Watchpoints,
}

pub struct App {
    debug_session: DebugSession,
    frame_history: FrameHistory,

    console_tab: ConsoleTab,
    variables_tab: VariablesTab,
    breakpoints_tab: BreakpointsTab,

    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    debug_session_reset: Arc<AtomicBool>,

    console_input: String,
    console_output: String,

    stdout: String,
    stderr: String,
}

impl App {
    pub fn new(cc: &CreationContext<'_>, debug_session: DebugSession) -> Self {
        cc.egui_ctx.set_fonts(resources::load_fonts());
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.scroll = ScrollStyle::solid();
        resources::register_fonts(&mut style);
        cc.egui_ctx.set_style(style);

        let debug_session_reset = Arc::new(AtomicBool::new(false));
        handle_lldb_events_thread(
            cc.egui_ctx.clone(),
            debug_session.listener.clone(),
            debug_session_reset.clone(),
        );

        Self {
            debug_session,
            frame_history: FrameHistory::default(),

            console_tab: ConsoleTab::Console,
            variables_tab: VariablesTab::Locals,
            breakpoints_tab: BreakpointsTab::Breakpoints,

            show_confirmation_dialog: false,
            allowed_to_close: false,
            debug_session_reset,

            console_input: String::new(),
            console_output: String::from_str("\n\n").unwrap(),

            stdout: String::new(),
            stderr: String::new(),
        }
    }
}

// Used to force a repaint when the UI needs to update without user interaction.
// For example when new data from stdout of the debugged process is available.
pub fn handle_lldb_events_thread(
    egui_ctx: Context,
    listener: SBListener,
    reset: Arc<AtomicBool>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let event = SBEvent::new();
        loop {
            listener.wait_for_event(1, &event);
            if !event.is_valid() {
                continue;
            }
            debug!("{:?}", event);
            reset.store(true, Ordering::Relaxed);
            egui_ctx.request_repaint_after(Duration::from_millis(100));
        }
    })
}
