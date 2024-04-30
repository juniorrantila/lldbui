mod components;
mod egui_app;
mod frame_history;
mod widgets;

use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, thread::JoinHandle};

use eframe::CreationContext;
use egui::{style::ScrollStyle, Context};
use lldb::{SBEvent, SBListener, SBProcessEvent, SBTarget};

use crate::app::frame_history::FrameHistory;
use crate::debug_session::DebugSession;
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
    scroll_source_view: Arc<AtomicBool>,

    console_input: String,
    console_output: String,

    stdout: Arc<Mutex<String>>,
    stderr: Arc<Mutex<String>>,
}

impl App {
    pub fn new(cc: &CreationContext<'_>, debug_session: DebugSession) -> Self {
        cc.egui_ctx.set_fonts(resources::load_fonts());
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.scroll = ScrollStyle::solid();
        resources::register_fonts(&mut style);
        cc.egui_ctx.set_style(style);

        let scroll_source_view = Arc::new(AtomicBool::new(false));
        let stdout = Arc::new(Mutex::new(String::new()));
        let stderr = Arc::new(Mutex::new(String::new()));

        handle_lldb_events_thread(
            cc.egui_ctx.clone(),
            debug_session.target.as_ref().unwrap().clone(),
            debug_session.listener.clone(),
            scroll_source_view.clone(),
            stdout.clone(),
            stderr.clone(),
        );

        Self {
            debug_session,
            frame_history: FrameHistory::default(),

            console_tab: ConsoleTab::Console,
            variables_tab: VariablesTab::Locals,
            breakpoints_tab: BreakpointsTab::Breakpoints,

            show_confirmation_dialog: false,
            allowed_to_close: false,
            scroll_source_view,

            console_input: String::new(),
            console_output: String::from_str("\n\n").unwrap(),

            stdout,
            stderr,
        }
    }

    pub fn scroll_source_view(&self) {
        self.scroll_source_view.store(true, Ordering::Relaxed);
    }
}

// Used to force a repaint when the UI needs to update without user interaction.
// For example when new data from stdout of the debugged process is available.
pub fn handle_lldb_events_thread(
    egui_ctx: Context,
    target: SBTarget,
    listener: SBListener,
    scroll: Arc<AtomicBool>,
    stdout: Arc<Mutex<String>>,
    stderr: Arc<Mutex<String>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let event = SBEvent::new();
        loop {
            listener.wait_for_event(1, &event);
            if !event.is_valid() {
                continue;
            }
            tracing::debug!("LLDB event: {:?}", event);

            if event.event_type() == SBProcessEvent::BROADCAST_BIT_STATE_CHANGED {
                scroll.store(true, Ordering::Relaxed);
            } else if event.event_type() == SBProcessEvent::BROADCAST_BIT_STDOUT {
                if let Some(data) = target.process().get_stdout_all() {
                    stdout.lock().unwrap().push_str(&data);
                }
            } else if event.event_type() == SBProcessEvent::BROADCAST_BIT_STDERR {
                // TODO(ds): somehow stderr of the process ends up in stdout and this is never triggered?
                // https://github.com/llvm/llvm-project/issues/25350#issuecomment-980951241
                if let Some(data) = target.process().get_stderr_all() {
                    stderr.lock().unwrap().push_str(&data);
                }
            }

            egui_ctx.request_repaint_after(Duration::from_millis(100));
        }
    })
}
