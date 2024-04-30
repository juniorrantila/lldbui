mod components;
mod egui_app;
mod frame_history;
mod widgets;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, thread::JoinHandle};

use eframe::CreationContext;
use egui::{style::ScrollStyle, Context};
use lldb::{SBEvent, SBListener, SBProcess, SBProcessEvent, SBTarget};

use crate::app::frame_history::FrameHistory;
use crate::debug_session::{DebugSession, DebugSessionState};
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
    debug_session: DebugSession,
    frame_history: FrameHistory,

    console_tab: ConsoleTab,
    variables_tab: VariablesTab,
    breakpoints_tab: BreakpointsTab,

    show_confirmation_dialog: bool,
    allowed_to_close: bool,

    source_cache: HashMap<String, String>,
    source_view_changed: Arc<AtomicBool>,

    console_input: String,
    console_output: String,
}

impl App {
    pub fn new(cc: &CreationContext<'_>, debug_session: DebugSession) -> Self {
        cc.egui_ctx.set_fonts(resources::load_fonts());
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.scroll = ScrollStyle::solid();
        resources::register_fonts(&mut style);
        cc.egui_ctx.set_style(style);

        let source_view_changed = Arc::new(AtomicBool::new(false));

        handle_lldb_events_thread(
            cc.egui_ctx.clone(),
            debug_session.listener.clone(),
            debug_session.target.clone(),
            debug_session.state.clone(),
            source_view_changed.clone(),
        );

        Self {
            debug_session,
            frame_history: FrameHistory::default(),

            console_tab: ConsoleTab::Console,
            variables_tab: VariablesTab::Locals,
            breakpoints_tab: BreakpointsTab::Breakpoints,

            show_confirmation_dialog: false,
            allowed_to_close: false,

            source_cache: HashMap::new(),
            source_view_changed,

            console_input: String::new(),
            console_output: String::from_str("\n\n").unwrap(),
        }
    }

    pub fn get_source(&mut self, path: &Path) -> Option<&String> {
        match path.exists() {
            true => Some(
                self.source_cache
                    .entry(path.to_str().unwrap().to_string())
                    .or_insert(read_to_string(path).unwrap()),
            ),
            false => None,
        }
    }
}

// Used to force a repaint when the UI needs to update without user interaction.
// For example when new data from stdout of the debugged process is available.
pub fn handle_lldb_events_thread(
    egui_ctx: Context,
    listener: SBListener,
    target: SBTarget,
    state: Arc<Mutex<DebugSessionState>>,
    source_view_changed: Arc<AtomicBool>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let event = SBEvent::new();
        loop {
            listener.wait_for_event(1, &event);
            if !event.is_valid() {
                continue;
            }
            tracing::debug!("LLDB event: {:?}", event);

            if let Some(process_event) = SBProcess::event_as_process_event(&event) {
                let event_type = event.event_type();
                if event_type == SBProcessEvent::BROADCAST_BIT_STATE_CHANGED {
                    let mut state = state.lock().unwrap();
                    state.process_state = process_event.process_state();
                    state.process_pid = process_event.process().process_id();
                    source_view_changed.store(true, Ordering::Relaxed);
                } else if event_type == SBProcessEvent::BROADCAST_BIT_STDOUT {
                    if let Some(data) = target.process().get_stdout_all() {
                        state.lock().unwrap().stdout.push_str(&data);
                    }
                } else if event_type == SBProcessEvent::BROADCAST_BIT_STDERR {
                    // TODO(ds): somehow stderr of the process ends up in stdout and this is never triggered?
                    // https://github.com/llvm/llvm-project/issues/25350#issuecomment-980951241
                    if let Some(data) = target.process().get_stderr_all() {
                        state.lock().unwrap().stderr.push_str(&data);
                    }
                }
            }

            egui_ctx.request_repaint_after(Duration::from_millis(100));
        }
    })
}
