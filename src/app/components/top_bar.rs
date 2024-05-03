use egui::{Align, Color32, Layout, Ui};
use lldb::RunMode;

use crate::{
    app::{widgets::IconButton, App},
    debugger,
};

pub fn add(app: &mut App, ui: &mut Ui) {
    let process = app.target.process();

    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                debugger::process_can_stop(&process),
                IconButton::new_with_color("⏸", "Stop", Color32::RED),
            )
            .clicked()
        {
            process.stop().unwrap();
        }
        if ui
            .add_enabled(
                debugger::process_can_continue(&process),
                IconButton::new_with_color("⏵", "Continue", Color32::GREEN),
            )
            .clicked()
        {
            process.continue_execution().unwrap();
        }
        if ui
            .add_enabled(
                debugger::process_can_continue(&process),
                IconButton::new("⬇", "Step Into"),
            )
            .clicked()
        {
            process
                .selected_thread()
                .step_into(RunMode::OnlyDuringStepping);
        }
        if ui
            .add_enabled(
                debugger::process_can_continue(&process),
                IconButton::new("⬈", "Step Over"),
            )
            .clicked()
        {
            process
                .selected_thread()
                .step_over(RunMode::OnlyDuringStepping)
                .unwrap();
        }
        if ui
            .add_enabled(
                debugger::process_frame_has_parent(&process),
                IconButton::new("⬆", "Step Out"),
            )
            .clicked()
        {
            process.selected_thread().step_out().unwrap();
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.button("Quit").clicked() {
                app.show_confirmation_dialog = true;
            }
            ui.separator();
        })
    });
}
