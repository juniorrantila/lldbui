use egui::{
    widgets, Align, CentralPanel, CollapsingHeader, Layout, RichText, ScrollArea, SidePanel,
    TopBottomPanel, Ui,
};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};
use lldb::{RunMode, SBDebugger, SBEvent, SBExpressionOptions, SBTarget, SBValue};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::{Read, Seek};
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
enum ConsoleTab {
    Console,
    Stdout,
    Stderr,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
enum VariableTab {
    Locals,
    Statics,
    Arguments,
    Registers,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct App {
    console_tab: ConsoleTab,
    variables_tab: VariableTab,
    #[serde(skip)]
    target: Option<SBTarget>,
    #[serde(skip)]
    source_cache: HashMap<String, String>,
    #[serde(skip)]
    scrolled: bool,
    #[serde(skip)]
    stdout: Option<File>,
    #[serde(skip)]
    stdout_buffer: Vec<u8>,
    #[serde(skip)]
    stdout_position: usize,
    #[serde(skip)]
    stderr: Option<File>,
    #[serde(skip)]
    stderr_buffer: Vec<u8>,
    #[serde(skip)]
    stderr_position: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            target: None,
            console_tab: ConsoleTab::Console,
            variables_tab: VariableTab::Locals,
            source_cache: HashMap::new(),
            scrolled: false,
            stdout: None,
            stdout_buffer: Vec::new(),
            stdout_position: 0,
            stderr: None,
            stderr_buffer: Vec::new(),
            stderr_position: 0,
        }
    }
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        target: SBTarget,
        stdout: PathBuf,
        stderr: PathBuf,
    ) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            App::default()
        };

        app.target = Some(target);
        app.stdout = Some(File::open(stdout).unwrap());
        app.stderr = Some(File::open(stderr).unwrap());
        app
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let debugger = self
            .target
            .as_ref()
            .expect("target should be set")
            .debugger();
        let process = self
            .target
            .as_ref()
            .expect("target should be set")
            .process();
        let executable = self
            .target
            .as_ref()
            .expect("target should be set")
            .executable();

        // without polling the events process.state() never changes???
        let event = SBEvent::new();
        while debugger.listener().get_next_event(&event) {
            // println!("{:?}", event);
        }

        if let Some(stdout) = &mut self.stdout {
            // TODO(ds): debugee's stdout is probably unbuffered, find a way
            //           to flush it
            stdout
                .seek(std::io::SeekFrom::Start(
                    self.stdout_position.try_into().unwrap(),
                ))
                .unwrap();
            self.stdout_position += stdout.read_to_end(&mut self.stdout_buffer).unwrap();
        }

        if let Some(stderr) = &mut self.stderr {
            stderr
                .seek(std::io::SeekFrom::Start(
                    self.stderr_position.try_into().unwrap(),
                ))
                .unwrap();
            self.stderr_position += stderr.read_to_end(&mut self.stderr_buffer).unwrap();
        }

        TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
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
                        if let Some(executable) = executable {
                            ui.label("Target:");
                            ui.label(executable.filename().to_string());
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
                    let thread = process.selected_thread();
                    if ui.button("Run").clicked() {
                        process.continue_execution().unwrap();
                        self.scrolled = false;
                    } else if ui.button("Step into").clicked() {
                        thread.step_into(RunMode::OnlyDuringStepping).unwrap();
                        self.scrolled = false;
                    } else if ui.button("Step over").clicked() {
                        thread.step_over(RunMode::OnlyDuringStepping).unwrap();
                        self.scrolled = false;
                    } else if ui.button("Step out").clicked() {
                        thread.step_out().unwrap();
                        self.scrolled = false;
                    }
                }
            });
        SidePanel::right("right_panel")
            .resizable(true)
            .min_width(250.0)
            .show(ctx, |ui| {
                TopBottomPanel::top("threads")
                    .resizable(true)
                    .min_height(150.0)
                    .show_inside(ui, |ui| {
                        ui.label("threads");
                        ScrollArea::vertical()
                            .id_source("threads")
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                egui::Grid::new("threads")
                                    .num_columns(1)
                                    .striped(true)
                                    .show(ui, |ui| {
                                        let mut selected_thread_id =
                                            process.selected_thread().thread_id();
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
                                                .selectable_value(
                                                    &mut selected_thread_id,
                                                    thread.thread_id(),
                                                    label,
                                                )
                                                .clicked()
                                            {
                                                process.set_selected_thread(&thread);
                                            }
                                            ui.end_row();
                                        }
                                    });
                            });
                    });
                TopBottomPanel::top("frames")
                    .resizable(true)
                    .min_height(150.0)
                    .show_inside(ui, |ui| {
                        ui.label("frames");
                        ScrollArea::vertical()
                            .id_source("frames")
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                ScrollArea::horizontal().auto_shrink(false).show(ui, |ui| {
                                    egui::Grid::new("frames").num_columns(2).striped(true).show(
                                        ui,
                                        |ui| {
                                            let thread = process.selected_thread();
                                            let mut selected_frame_id =
                                                thread.selected_frame().frame_id();
                                            for frame in thread.frames() {
                                                if !frame.is_valid() {
                                                    continue;
                                                }

                                                if ui
                                                    .selectable_value(
                                                        &mut selected_frame_id,
                                                        frame.frame_id(),
                                                        frame.display_function_name().unwrap(),
                                                    )
                                                    .clicked()
                                                {
                                                    thread.set_selected_frame(frame.frame_id());
                                                    self.scrolled = false;
                                                }
                                                if let Some(line_entry) = frame.line_entry() {
                                                    let path: PathBuf = [
                                                        line_entry.filespec().directory(),
                                                        line_entry.filespec().filename(),
                                                    ]
                                                    .iter()
                                                    .collect();
                                                    ui.label(format!(
                                                        "{}:{}",
                                                        path.display(),
                                                        line_entry.line(),
                                                    ));
                                                }
                                                ui.end_row();
                                            }
                                        },
                                    );
                                });
                            });
                    });
                CentralPanel::default().show_inside(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.variables_tab, VariableTab::Locals, "locals");
                        ui.selectable_value(
                            &mut self.variables_tab,
                            VariableTab::Statics,
                            "statics",
                        );
                        ui.selectable_value(
                            &mut self.variables_tab,
                            VariableTab::Arguments,
                            "arguments",
                        );
                        ui.selectable_value(
                            &mut self.variables_tab,
                            VariableTab::Registers,
                            "registers",
                        );
                    });
                    ScrollArea::vertical()
                        .id_source("variables")
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            let frame = process.selected_thread().selected_frame();
                            match self.variables_tab {
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
                            }
                        });
                });
            });
        TopBottomPanel::bottom("console_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.console_tab, ConsoleTab::Console, "console");
                    ui.selectable_value(&mut self.console_tab, ConsoleTab::Stdout, "stdout");
                    ui.selectable_value(&mut self.console_tab, ConsoleTab::Stderr, "stderr");
                });
                match self.console_tab {
                    ConsoleTab::Console => {
                        ui.label("TODO");
                    }
                    ConsoleTab::Stdout => {
                        ScrollArea::horizontal()
                            .id_source(ui.next_auto_id())
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                ScrollArea::vertical()
                                    .id_source(ui.next_auto_id())
                                    .auto_shrink(false)
                                    .show(ui, |ui| {
                                        ui.label(std::str::from_utf8(&self.stdout_buffer).unwrap());
                                    });
                                ui.scroll_to_cursor(Some(Align::BOTTOM));
                            });
                    }
                    ConsoleTab::Stderr => {
                        ScrollArea::horizontal()
                            .id_source(ui.next_auto_id())
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                ScrollArea::vertical()
                                    .id_source(ui.next_auto_id())
                                    .auto_shrink(false)
                                    .show(ui, |ui| {
                                        ui.label(std::str::from_utf8(&self.stderr_buffer).unwrap());
                                    });
                                ui.scroll_to_cursor(Some(Align::BOTTOM));
                            });
                    }
                }
            });
        CentralPanel::default().show(ctx, |ui| {
            let frame = process.selected_thread().selected_frame();
            if let Some(line_entry) = frame.line_entry() {
                let path: PathBuf = [
                    line_entry.filespec().directory(),
                    line_entry.filespec().filename(),
                ]
                .iter()
                .collect();

                let key = path.clone().into_os_string().into_string().unwrap();

                let fs = line_entry.filespec();
                let source = if fs.exists() {
                    self.source_cache
                        .entry(key.clone())
                        .or_insert(read_to_string(&path).unwrap())
                } else {
                    ""
                };

                ui.label(&key);
                ui.separator();

                // TODO: use proper mapping
                let compile_unit = frame.compile_unit();
                let mut language = format!("{:?}", compile_unit.language());
                if language == "C99" || language == "C11" {
                    language = "C".to_string();
                }

                let highlighted_line = if let Some(line_entry) = frame.line_entry() {
                    line_entry.line()
                } else {
                    0
                };

                let theme = &CodeTheme::from_style(ui.style());

                ScrollArea::horizontal()
                    .id_source(&key)
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ScrollArea::vertical()
                            .id_source(path)
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                let mut i = 0;
                                egui::Grid::new("source").num_columns(3).show(ui, |ui| {
                                    for line in source.lines() {
                                        i += 1;
                                        if i == highlighted_line {
                                            ui.label(
                                                RichText::new("→")
                                                    .monospace()
                                                    .color(egui::Color32::YELLOW),
                                            );
                                        } else {
                                            ui.label("");
                                        }
                                        let mut line_number =
                                            RichText::new(format!("{}", i)).monospace();
                                        if i == highlighted_line {
                                            line_number = line_number.color(egui::Color32::YELLOW);
                                        }
                                        ui.label(line_number);
                                        let layout_job =
                                            highlight(ui.ctx(), theme, &line, &language);
                                        let response =
                                            ui.add(egui::Label::new(layout_job).selectable(true));
                                        if !self.scrolled && i == highlighted_line {
                                            response.scroll_to_me(Some(egui::Align::Center));
                                            self.scrolled = true;
                                        }
                                        ui.end_row();
                                    }
                                });
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
                    CollapsingHeader::new(v.name().expect("name should be present"))
                        .id_source(ui.next_auto_id())
                        .show(ui, |ui| {
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
