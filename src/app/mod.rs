mod components;
mod egui_app;
mod frame_history;
mod widgets;

use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use std::{thread, thread::JoinHandle};

use eframe::CreationContext;
use egui::{style::ScrollStyle, Context};
use lldb::{SBEvent, SBListener, SBTarget};

use crate::app::frame_history::FrameHistory;
use crate::resources;

#[derive(PartialEq)]
enum ConsoleTab {
    Console,
    Stdout,
    Stderr,
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
    target: SBTarget,
    frame_history: FrameHistory,

    console_tab: ConsoleTab,
    variables_tab: VariablesTab,
    breakpoints_tab: BreakpointsTab,

    show_confirmation_dialog: bool,
    allowed_to_close: bool,

    source_cache: HashMap<String, String>,
    source_file: String,
    source_line: u32,

    process_stdout: String,
    process_stderr: String,

    console_input: String,
    console_output: String,
}

impl App {
    pub fn new(cc: &CreationContext<'_>, target: SBTarget) -> Self {
        cc.egui_ctx.set_fonts(resources::load_fonts());
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.scroll = ScrollStyle::solid();
        resources::register_fonts(&mut style);
        cc.egui_ctx.set_style(style);

        handle_lldb_events_thread(cc.egui_ctx.clone(), target.debugger().listener().clone());

        Self {
            target,
            frame_history: FrameHistory::default(),

            console_tab: ConsoleTab::Console,
            variables_tab: VariablesTab::Locals,
            breakpoints_tab: BreakpointsTab::Breakpoints,

            show_confirmation_dialog: false,
            allowed_to_close: false,

            source_cache: HashMap::new(),
            source_file: String::new(),
            source_line: 0,

            process_stdout: String::new(),
            process_stderr: String::new(),

            console_input: String::new(),
            console_output: String::from_str("\n\n").unwrap(),
        }
    }
}

// Used to force a repaint when the UI needs to update without user interaction.
// For example when new data from stdout of the debugged process is available.
pub fn handle_lldb_events_thread(egui_ctx: Context, listener: SBListener) -> JoinHandle<()> {
    thread::spawn(move || {
        let event = SBEvent::new();
        loop {
            listener.wait_for_event(1, &event);
            if event.is_valid() {
                tracing::debug!("LLDB event: {:?}", event);
                egui_ctx.request_repaint_after(Duration::from_millis(100));
            }
        }
    })
}
