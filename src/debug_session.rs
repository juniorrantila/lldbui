use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;
use lldb::{
    LaunchFlags, RunMode, SBAttachInfo, SBDebugger, SBFrame, SBLaunchInfo, SBListener, SBProcess,
    SBTarget, SBThread, StateType,
};

pub fn initialize() {
    SBDebugger::initialize();
}
pub fn shutdown() {
    SBDebugger::terminate();
}

pub struct DebugSession {
    pub debugger: SBDebugger,
    pub listener: SBListener,
    pub target: Option<SBTarget>,

    source_cache: HashMap<String, String>,
}

impl DebugSession {
    pub fn new(debugger: SBDebugger) -> Self {
        let listener = debugger.listener();
        listener.start_listening_for_event_class(&debugger, SBTarget::broadcaster_class_name(), !0);
        listener.start_listening_for_event_class(
            &debugger,
            SBProcess::broadcaster_class_name(),
            !0,
        );
        // TODO(ds): why don't we get thread events?
        listener.start_listening_for_event_class(&debugger, SBThread::broadcaster_class_name(), !0);

        Self {
            debugger,
            listener,
            target: None,

            source_cache: HashMap::new(),
        }
    }

    pub fn run(&mut self, executable: &str, args: Option<Vec<String>>) -> Result<()> {
        let target = self.debugger.create_target(executable, None, None, false)?;
        let launch_info = SBLaunchInfo::new();
        launch_info.set_launch_flags(LaunchFlags::STOP_AT_ENTRY);
        if let Some(args) = args {
            launch_info.set_arguments(args.iter().map(AsRef::as_ref), false);
        }
        // (ds): The launch info isn't persisted in the target if we don't explicitly set it here.
        target.set_launch_info(launch_info.clone());
        target.launch(launch_info)?;

        self.target = Some(target);

        Ok(())
    }

    pub fn attach_pid(&mut self, pid: u64) -> Result<()> {
        let target = self.debugger.create_target("", None, None, false)?;
        let attach_info = SBAttachInfo::new_with_pid(pid);
        target.attach(attach_info)?;
        self.target = Some(target);
        Ok(())
    }

    pub fn attach_name(&mut self, name: &str) -> Result<()> {
        let target = self.debugger.create_target("", None, None, false)?;
        let attach_info = SBAttachInfo::new_with_path(name, true, false);
        target.attach(attach_info)?;
        self.target = Some(target);
        Ok(())
    }

    pub fn process_name(&self) -> String {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .process_info()
            .name()
            .to_string()
    }

    pub fn process_args(&self) -> Vec<String> {
        self.target
            .as_ref()
            .unwrap()
            .get_launch_info()
            .arguments()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn process_pid(&self) -> u64 {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .process_info()
            .process_id()
    }

    pub fn process_state(&self) -> String {
        format!("{:?}", self.target.as_ref().unwrap().process().state())
    }

    pub fn process_is_running(&self) -> bool {
        matches!(
            self.target.as_ref().unwrap().process().state(),
            StateType::Running | StateType::Stepping
        )
    }

    pub fn process_can_continue(&self) -> bool {
        matches!(
            self.target.as_ref().unwrap().process().state(),
            StateType::Stopped | StateType::Suspended
        )
    }

    pub fn process_stop(&self) {
        self.target.as_ref().unwrap().process().stop().unwrap();
    }

    pub fn process_continue(&self) {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .continue_execution()
            .unwrap();
    }

    pub fn step_into(&self) {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .selected_thread()
            .step_into(RunMode::OnlyDuringStepping);
    }

    pub fn step_over(&self) {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .selected_thread()
            .step_over(RunMode::OnlyDuringStepping)
            .unwrap();
    }

    pub fn step_out(&self) {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .selected_thread()
            .step_out()
            .unwrap();
    }

    pub fn get_stdout(&self) -> Option<String> {
        self.target.as_ref().unwrap().process().get_stdout_all()
    }

    pub fn get_stderr(&self) -> Option<String> {
        // TODO(ds): somehow stderr of the process ends up in stdout and this
        //           is always empty?
        self.target.as_ref().unwrap().process().get_stderr_all()
    }

    pub fn selected_thread_id(&self) -> u64 {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .selected_thread()
            .thread_id()
    }

    pub fn process_threads(&self) -> Vec<SBThread> {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .threads()
            .filter(|thread| thread.is_valid())
            .collect()
    }

    pub fn select_thread(&self, thread: &SBThread) {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .set_selected_thread(thread);
    }

    pub fn selected_frame(&self) -> SBFrame {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .selected_thread()
            .selected_frame()
    }

    pub fn selected_frame_id(&self) -> u32 {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .selected_thread()
            .selected_frame()
            .frame_id()
    }

    pub fn thread_frames(&self) -> Vec<SBFrame> {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .selected_thread()
            .frames()
            .filter(|frame| frame.is_valid())
            .collect()
    }

    pub fn select_frame(&self, frame: &SBFrame) {
        self.target
            .as_ref()
            .unwrap()
            .process()
            .selected_thread()
            .set_selected_frame(frame.frame_id());
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
