use egui::{Color32, ScrollArea, Ui};

use crate::{
    app::{widgets::IconButton, App, BreakpointsTab},
    debugger,
};

pub fn add(app: &mut App, ui: &mut Ui) {
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
                    for (id, file, line) in debugger::breakpoint_locations(&app.target).iter() {
                        ui.label(format!("{}", id));
                        ui.label(file);
                        ui.label(format!("{}", line));
                        if ui
                            .add(IconButton::new_with_color("❌", "remove", Color32::RED))
                            .clicked()
                        {
                            app.target.delete_breakpoint(*id)
                        }
                        ui.end_row();
                    }
                }),
            BreakpointsTab::Watchpoints => egui::Grid::new(ui.next_auto_id())
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    for watchpoint in app.target.watchpoints() {
                        ui.label(format!("{}", watchpoint.id()));
                        ui.label(format!("{:#x}", watchpoint.watch_address()));
                        if ui
                            .add(IconButton::new_with_color("❌", "remove", Color32::RED))
                            .clicked()
                        {
                            app.target.delete_watchpoint(watchpoint.id())
                        }
                        ui.end_row()
                    }
                }),
        });
}
