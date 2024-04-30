use egui::{Color32, ScrollArea, Ui};

use crate::app::{widgets::IconButton, App, BreakpointsTab};

pub fn add(app: &mut App, ui: &mut Ui) {
    let state = app.debug_session.state.lock().unwrap();

    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut app.breakpoints_tab,
            BreakpointsTab::Breakpoints,
            "breakpoints",
        );
        ui.selectable_value(
            &mut app.breakpoints_tab,
            BreakpointsTab::Watchpoints,
            "watchpoints",
        );
    });
    ScrollArea::both()
        .id_source("breakpoints")
        .show(ui, |ui| match app.breakpoints_tab {
            BreakpointsTab::Breakpoints => egui::Grid::new(ui.next_auto_id())
                .num_columns(4)
                .striped(true)
                .show(ui, |ui| {
                    for location in state.breakpoints.iter() {
                        let id = location.breakpoint().id();
                        let mut file = String::new();
                        let mut line = 0;
                        if let Some(address) = location.address() {
                            if let Some(line_entry) = address.line_entry() {
                                file = line_entry.filespec().filename().to_string();
                                line = line_entry.line();
                            }
                        }

                        ui.label(format!("{}", id));
                        ui.label(file);
                        ui.label(format!("{}", line));
                        if ui
                            .add(IconButton::new_with_color("❌", "remove", Color32::RED))
                            .clicked()
                        {
                            app.debug_session.delete_breakpoint(id);
                        }
                        ui.end_row();
                    }
                }),
            BreakpointsTab::Watchpoints => egui::Grid::new(ui.next_auto_id())
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    for watchpoint in state.watchpoints.iter() {
                        ui.label(format!("{}", watchpoint.id()));
                        ui.label(format!("{:#x}", watchpoint.watch_address()));
                        if ui
                            .add(IconButton::new_with_color("❌", "remove", Color32::RED))
                            .clicked()
                        {
                            app.debug_session.delete_watchpoint(watchpoint.id());
                        }
                        ui.end_row()
                    }
                }),
        });
}
