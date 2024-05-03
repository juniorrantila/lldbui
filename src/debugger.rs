use anyhow::Result;
use lldb::{LaunchFlags, SBAttachInfo, SBDebugger, SBProcess, SBTarget, SBThread, StateType};

pub fn run(
    executable: &str,
    source_init_files: bool,
    args: Option<Vec<String>>,
) -> Result<SBTarget> {
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

    Ok(target)
}

pub fn attach_pid(pid: u64, source_init_files: bool) -> Result<SBTarget> {
    let debugger = create_debugger(source_init_files);
    let target = debugger.create_target("", None, None, false)?;
    let attach_info = SBAttachInfo::new_with_pid(pid);

    // Disable async events so the attach will be successful when we return from
    // the attach call and the attach will happen synchronously
    debugger.set_asynchronous(false);
    target.attach(attach_info)?;
    debugger.set_asynchronous(true);

    Ok(target)
}

pub fn attach_name(name: &str, source_init_files: bool) -> Result<SBTarget> {
    let debugger = create_debugger(source_init_files);
    let target = debugger.create_target("", None, None, false)?;
    let attach_info = SBAttachInfo::new_with_path(name, true, false);

    // Disable async events so the attach will be successful when we return from
    // the attach call and the attach will happen synchronously
    debugger.set_asynchronous(false);
    target.attach(attach_info)?;
    debugger.set_asynchronous(true);

    Ok(target)
}

pub fn initialize() {
    SBDebugger::initialize();
}

pub fn terminate() {
    SBDebugger::terminate();
}

pub fn breakpoint_locations(target: &SBTarget) -> Vec<(i32, String, u32)> {
    let mut locations = Vec::new();
    for breakpoint in target.breakpoints() {
        for location in breakpoint.locations() {
            let Some(address) = location.address() else {
                continue;
            };
            let Some(line_entry) = address.line_entry() else {
                continue;
            };
            locations.push((
                breakpoint.id(),
                line_entry.filespec().filename().to_string(),
                line_entry.line(),
            ))
        }
    }
    locations
}

pub fn process_can_stop(process: &SBProcess) -> bool {
    matches!(process.state(), StateType::Running | StateType::Stepping)
}

pub fn process_can_continue(process: &SBProcess) -> bool {
    matches!(process.state(), StateType::Stopped | StateType::Suspended)
}

pub fn process_frame_has_parent(process: &SBProcess) -> bool {
    let frame = process.selected_thread().selected_frame();
    frame.is_valid() && frame.parent_frame().is_some()
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
