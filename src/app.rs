use egui::{
    widgets, Align, CentralPanel, Layout, RichText, ScrollArea, SidePanel, TopBottomPanel, Ui,
};
use egui_extras::syntax_highlighting::{code_view_ui, CodeTheme};
use lldb::{SBDebugger, SBEvent, SBListener, SBProcess, SBTarget, SBThread, SBValue, StateType};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
enum ConsoleTab {
    Console,
    Stdout,
    Stderr,
}

#[derive(Debug, PartialEq)]
enum VariableTab {
    Locals,
    Statics,
    Arguments,
    Registers,
}

pub struct App {
    console_tab: ConsoleTab,
    variables_tab: VariableTab,
    target: SBTarget,
    listener: SBListener,
    selected_thread_id: u64,
    selected_frame_id: u32,
    sources: HashMap<PathBuf, String>,
}

impl App {
    pub fn new(target: SBTarget) -> Self {
        let listener = target.debugger().listener();
        Self {
            console_tab: ConsoleTab::Console,
            variables_tab: VariableTab::Locals,
            target,
            listener,
            selected_thread_id: 0,
            selected_frame_id: 0,
            sources: HashMap::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            console_tab,
            variables_tab,
            target,
            listener,
            selected_thread_id,
            selected_frame_id,
            sources,
        } = self;

        let process = target.process();
        let thread = process.selected_thread();
        let frame = thread.selected_frame();
        *selected_thread_id = thread.thread_id();
        *selected_frame_id = thread.selected_frame().frame_id();

        // without polling the events process.state() never changes???
        let mut event = SBEvent::new();
        while listener.get_next_event(&mut event) {
            println!("{:?}", event);
        }

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.label(RichText::new(SBDebugger::version()).small());
                ui.separator();
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                egui::Grid::new("target")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        if let Some(exec) = target.executable() {
                            ui.label("Target:");
                            ui.label(format!("{}", exec.filename()));
                            ui.end_row();
                        }

                        ui.label("Process state:");
                        ui.label(format!("{:?}", process.state()));
                        ui.end_row();

                        ui.label("PID:");
                        ui.label(format!("{:?}", process.process_id()));
                        ui.end_row();
                    });
                if process.is_running() {
                    if ui.button("Stop").clicked() {
                        process.stop().unwrap();
                    }
                } else if process.is_stopped() {
                    if ui.button("Run").clicked() {
                        process.continue_execution().unwrap();
                    }
                }
            });
        SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("threads");
                ScrollArea::vertical().id_source("threads").show(ui, |ui| {
                    egui::Grid::new("frames")
                        .num_columns(1)
                        .striped(true)
                        .show(ui, |ui| {
                            for thread in process.threads() {
                                if !thread.is_valid() {
                                    continue;
                                }
                                let mut label = format!(
                                    "{} {}",
                                    thread.thread_id(),
                                    thread.name().unwrap_or("")
                                );
                                if let Some(queue) = thread.queue() {
                                    label.push_str(&format!(" queue={}", queue.name()));
                                }
                                if thread.is_stopped() {
                                    label.push_str(&format!(
                                        " stop_reason={:?}",
                                        thread.stop_reason()
                                    ));
                                }
                                if ui
                                    .selectable_value(selected_thread_id, thread.thread_id(), label)
                                    .clicked()
                                {
                                    process.set_selected_thread(&thread);
                                }
                                ui.end_row();
                            }
                        });
                });
                ui.separator();
                ui.label("frames");
                egui::Grid::new("frames")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        for frame in thread.frames() {
                            if !frame.is_valid() {
                                continue;
                            }
                            let function = frame.function();
                            if function.is_valid() {
                                if ui
                                    .selectable_value(
                                        selected_frame_id,
                                        frame.frame_id(),
                                        function.display_name(),
                                    )
                                    .clicked()
                                {
                                    thread.set_selected_frame(frame.frame_id());
                                }
                            } else {
                                ui.label(frame.display_function_name().unwrap_or(""));
                            }
                            if let Some(line_entry) = frame.line_entry() {
                                let path: PathBuf = [
                                    line_entry.filespec().directory(),
                                    line_entry.filespec().filename(),
                                ]
                                .iter()
                                .collect();
                                ui.label(format!("{}:{}", path.display(), line_entry.line(),));
                            } else {
                                ui.label("");
                            }
                            ui.end_row();
                        }
                    });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.selectable_value(variables_tab, VariableTab::Locals, "locals");
                    ui.selectable_value(variables_tab, VariableTab::Statics, "statics");
                    ui.selectable_value(variables_tab, VariableTab::Arguments, "arguments");
                    ui.selectable_value(variables_tab, VariableTab::Registers, "registers");
                });
                ScrollArea::vertical()
                    .id_source("variables")
                    .show(ui, |ui| match variables_tab {
                        VariableTab::Locals => {
                            render_values(ui, frame.locals().iter());
                        }
                        VariableTab::Statics => {
                            render_values(ui, frame.statics().iter());
                        }
                        VariableTab::Arguments => {
                            render_values(ui, frame.arguments().iter());
                        }
                        VariableTab::Registers => {
                            render_values(ui, frame.registers().iter());
                        }
                    });
            });
        TopBottomPanel::bottom("console_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(console_tab, ConsoleTab::Console, "console");
                    ui.selectable_value(console_tab, ConsoleTab::Stdout, "stdout");
                    ui.selectable_value(console_tab, ConsoleTab::Stderr, "stderr");
                });
                ui.label("foo");
            });
        CentralPanel::default().show(ctx, |ui| {
            if let Some(line_entry) = frame.line_entry() {
                let path: PathBuf = [
                    line_entry.filespec().directory(),
                    line_entry.filespec().filename(),
                ]
                .iter()
                .collect();
                ui.label(format!("path: {}", path.display()));

                let fs = line_entry.filespec();
                ui.label(format!("exists: {}", fs.exists()));

                let compile_unit = frame.compile_unit();
                ui.label(format!("Language: {:?}", compile_unit.language()));

                let code = sources
                    .entry(path.clone())
                    .or_insert_with(|| read_to_string(&path).unwrap());

                ScrollArea::vertical().show(ui, |ui| {
                    ScrollArea::horizontal().show(ui, |ui| {
                        code_view_ui(ui, &CodeTheme::from_style(ui.style()), &code, "C");
                    });
                });
            }
        });
    }
}

fn render_values(ui: &mut Ui, values: impl Iterator<Item = SBValue>) {
    egui::Grid::new(ui.next_auto_id())
        .num_columns(3)
        .striped(true)
        .show(ui, |ui| {
            for v in values {
                if v.children().count() > 0 {
                    ui.collapsing(v.name().expect("name should be present"), |ui| {
                        render_values(ui, v.children());
                    });
                } else {
                    render_value(ui, &v);
                }
                ui.end_row();
            }
        });
}

fn render_value(ui: &mut Ui, value: &SBValue) {
    if let Some(name) = value.name() {
        ui.label(name);
    }
    if let Some(type_name) = value.display_type_name() {
        ui.label(type_name);
    }
    if let Some(value) = value.value() {
        ui.label(value);
    }
}
