use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Result;
use lldb::{
    lldb_pid_t, lldb_tid_t, LaunchFlags, RunMode, SBAttachInfo, SBBreakpointLocation, SBDebugger,
    SBFrame, SBListener, SBProcess, SBTarget, SBThread, SBValue, SBWatchpoint, StateType,
};

pub struct DebugSessionState {
    pub executable: String,

    pub process_state: StateType,
    pub process_pid: lldb_pid_t,
    pub process_args: Vec<String>,
    pub process_is_running: bool,
    pub process_can_continue: bool,

    pub can_step_out: bool,

    pub threads: Vec<SBThread>,
    pub selected_thread_id: lldb_tid_t,

    pub frames: Vec<SBFrame>,
    pub selected_frame_id: u32,

    pub locals: Vec<SBValue>,
    pub statics: Vec<SBValue>,
    pub arguments: Vec<SBValue>,
    pub registers: Vec<SBValue>,

    pub breakpoints: Vec<SBBreakpointLocation>,
    pub watchpoints: Vec<SBWatchpoint>,

    pub stdout: String,
    pub stderr: String,
}

impl DebugSessionState {
    pub fn new(target: &SBTarget) -> Self {
        let executable = match target.executable() {
            Some(executable) => executable.filename().to_string(),
            None => {
                tracing::error!("executable not available");
                String::new()
            }
        };

        let process_args = target
            .get_launch_info()
            .arguments()
            .map(|s| s.to_string())
            .collect();

        DebugSessionState {
            executable,

            process_state: target.process().state(),
            process_pid: target.process().process_id(),
            process_args,
            process_is_running: false,
            process_can_continue: false,

            can_step_out: false,

            threads: Vec::new(),
            selected_thread_id: 0,

            frames: Vec::new(),
            selected_frame_id: 0,

            locals: Vec::new(),
            statics: Vec::new(),
            arguments: Vec::new(),
            registers: Vec::new(),

            breakpoints: Vec::new(),
            watchpoints: Vec::new(),

            stdout: String::new(),
            stderr: String::new(),
        }
    }

    pub fn selected_frame(&self) -> Option<&SBFrame> {
        for frame in self.frames.iter() {
            if frame.frame_id() == self.selected_frame_id {
                return Some(frame);
            }
        }
        return None;
    }
}

pub struct DebugSession {
    pub debugger: SBDebugger,
    pub listener: SBListener,
    pub target: SBTarget,

    pub state: Arc<Mutex<DebugSessionState>>,
}

impl DebugSession {
    pub fn new(debugger: SBDebugger, target: SBTarget) -> Self {
        let state = Arc::new(Mutex::new(DebugSessionState::new(&target)));
        let listener = debugger.listener();
        DebugSession {
            debugger,
            target,
            listener,

            state,
        }
    }

    pub fn run(
        executable: &str,
        source_init_files: bool,
        args: Option<Vec<String>>,
    ) -> Result<DebugSession> {
        let debugger = create_debugger(source_init_files);
        let target = debugger.create_target(executable, None, None, false)?;
        let launch_info = target.get_launch_info();
        launch_info.set_launch_flags(LaunchFlags::STOP_AT_ENTRY);
        if let Some(args) = args {
            launch_info.set_arguments(args.iter().map(AsRef::as_ref), false);
        }

        // (ds): The launch info isn't persisted in the target if we don't
        //       explicitly set it here.
        target.set_launch_info(launch_info.clone());

        // Disable async events so the launch will be successful when we return from
        // the launch call and the launch will happen synchronously
        debugger.set_asynchronous(false);
        target.launch(launch_info)?;
        debugger.set_asynchronous(true);

        Ok(DebugSession::new(debugger, target))
    }

    pub fn attach_pid(pid: u64, source_init_files: bool) -> Result<DebugSession> {
        let debugger = create_debugger(source_init_files);
        let target = debugger.create_target("", None, None, false)?;
        let attach_info = SBAttachInfo::new_with_pid(pid);

        // Disable async events so the attach will be successful when we return from
        // the attach call and the attach will happen synchronously
        debugger.set_asynchronous(false);
        target.attach(attach_info)?;
        debugger.set_asynchronous(true);

        Ok(DebugSession::new(debugger, target))
    }

    pub fn attach_name(name: &str, source_init_files: bool) -> Result<DebugSession> {
        let debugger = create_debugger(source_init_files);
        let target = debugger.create_target("", None, None, false)?;
        let attach_info = SBAttachInfo::new_with_path(name, true, false);

        // Disable async events so the attach will be successful when we return from
        // the attach call and the attach will happen synchronously
        debugger.set_asynchronous(false);
        target.attach(attach_info)?;
        debugger.set_asynchronous(true);

        Ok(DebugSession::new(debugger, target))
    }

    pub fn process_can_continue(&self) -> bool {
        matches!(
            self.target.process().state(),
            StateType::Stopped | StateType::Suspended
        )
    }

    pub fn has_parent_frame(&self) -> bool {
        let frame = self.target.process().selected_thread().selected_frame();
        frame.is_valid() && frame.parent_frame().is_some()
    }

    pub fn stop_process(&self) {
        let _ = self
            .target
            .process()
            .stop()
            .map_err(|err| tracing::error!("{}", err));
    }

    pub fn continue_process(&self) {
        let _ = self
            .target
            .process()
            .continue_execution()
            .map_err(|err| tracing::error!("{}", err));
    }

    pub fn step_into(&self) {
        self.target
            .process()
            .selected_thread()
            .step_into(RunMode::OnlyDuringStepping);
    }

    pub fn step_over(&self) {
        let _ = self
            .target
            .process()
            .selected_thread()
            .step_over(RunMode::OnlyDuringStepping)
            .map_err(|err| tracing::error!("{}", err));
    }

    pub fn step_out(&self) {
        let _ = self
            .target
            .process()
            .selected_thread()
            .step_out()
            .map_err(|err| tracing::error!("{}", err));
    }

    pub fn selected_thread_id(&self) -> u64 {
        self.target.process().selected_thread().thread_id()
    }

    pub fn process_threads(&self) -> Vec<SBThread> {
        self.target
            .process()
            .threads()
            .filter(|thread| thread.is_valid())
            .collect()
    }

    pub fn select_thread(&self, thread_id: lldb_tid_t) {
        self.target.process().set_selected_thread_by_id(thread_id);
    }

    pub fn selected_frame(&self) -> SBFrame {
        self.target.process().selected_thread().selected_frame()
    }

    pub fn selected_frame_id(&self) -> u32 {
        self.target
            .process()
            .selected_thread()
            .selected_frame()
            .frame_id()
    }

    pub fn thread_frames(&self) -> Vec<SBFrame> {
        self.target
            .process()
            .selected_thread()
            .frames()
            .filter(|frame| frame.is_valid())
            .collect()
    }

    pub fn select_frame(&self, frame_id: u32) {
        self.target
            .process()
            .selected_thread()
            .set_selected_frame(frame_id);
    }

    pub fn execute_command(&self, cmd: &str) -> Result<&str, String> {
        self.debugger.execute_command(cmd)
    }

    pub fn delete_breakpoint(&self, id: i32) {
        self.target.delete_breakpoint(id);
    }

    pub fn delete_watchpoint(&self, id: i32) {
        self.target.delete_watchpoint(id);
    }
}

impl Drop for DebugSession {
    fn drop(&mut self) {
        SBDebugger::terminate();
    }
}

fn create_debugger(source_init_files: bool) -> SBDebugger {
    SBDebugger::initialize();

    let debugger = SBDebugger::create(source_init_files);
    // debugger.enable_log("lldb", &["process", "target"]);

    let listener = debugger.listener();
    listener.start_listening_for_event_class(&debugger, SBTarget::broadcaster_class_name(), !0);
    listener.start_listening_for_event_class(&debugger, SBProcess::broadcaster_class_name(), !0);
    listener.start_listening_for_event_class(&debugger, SBThread::broadcaster_class_name(), !0);

    debugger
}
