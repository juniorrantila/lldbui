use egui::{widgets, Align, CentralPanel, Layout, RichText, ScrollArea, SidePanel, TopBottomPanel};
use lldb::{SBDebugger, SBEvent, SBListener, SBTarget};

#[derive(Debug, PartialEq)]
enum ConsoleTab {
    Console,
    Stdout,
    Stderr,
}

pub struct App {
    console_tab: ConsoleTab,
    target: SBTarget,
    listener: SBListener,
    selected_thread_id: u64,
    selected_frame_id: u32,
}

impl App {
    pub fn new(target: SBTarget) -> Self {
        let listener = target.debugger().listener();
        Self {
            console_tab: ConsoleTab::Console,
            target,
            listener,
            selected_thread_id: 0,
            selected_frame_id: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            console_tab,
            target,
            listener,
            selected_thread_id,
            selected_frame_id,
        } = self;

        let process = target.process();
        *selected_thread_id = process.selected_thread().thread_id();

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
                egui::Grid::new("my_grid")
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
                ScrollArea::vertical().id_source("threads").show(ui, |ui| {
                    ui.label("threads");
                    for thread in process.threads() {
                        if ui
                            .selectable_value(
                                selected_thread_id,
                                thread.thread_id(),
                                format!("{}", thread.thread_id()),
                            )
                            .clicked()
                        {
                            process.set_selected_thread(&thread);
                        }
                    }
                });
                ui.separator();
                ScrollArea::vertical().id_source("frames").show(ui, |ui| {
                    ui.label("frames");
                    ScrollArea::horizontal().show(ui, |ui| {
                        egui::Grid::new("my_grid")
                            .num_columns(2)
                            .striped(true)
                            .show(ui, |ui| {
                                let thread = process.selected_thread();
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
                                    } else if let Some(name) = frame.display_function_name() {
                                        ui.label(name);
                                    }
                                    if let Some(line_entry) = frame.line_entry() {
                                        ui.label(format!(
                                            "{}:{}",
                                            line_entry.filespec().filename(),
                                            line_entry.line(),
                                        ));
                                    } else {
                                        ui.label("");
                                    }
                                    ui.end_row();
                                }
                            });
                        ui.separator();
                        ScrollArea::vertical()
                            .id_source("variables")
                            .show(ui, |ui| {
                                ui.label("variables");
                                let thread = process.selected_thread();
                                let frame = thread.selected_frame();
                                for v in frame.all_variables().iter() {
                                    ui.label(v.name());
                                }
                            });
                    });
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
                ui.separator();
                ui.label("foo");
            });
        CentralPanel::default().show(ctx, |ui| {
            ui.label("code");
        });
    }
}
