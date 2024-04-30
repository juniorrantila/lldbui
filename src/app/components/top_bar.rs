use egui::{Align, Color32, Layout, Ui};

use crate::app::{widgets::IconButton, App};

pub fn add(app: &mut App, ui: &mut Ui) {
    let debug_session = &app.debug_session;
    let state = debug_session.state.lock().unwrap();

    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                state.process_is_running,
                IconButton::new_with_color("⏸", "Stop", Color32::RED),
            )
            .clicked()
        {
            debug_session.stop_process();
        }
        if ui
            .add_enabled(
                state.process_can_continue,
                IconButton::new_with_color("⏵", "Continue", Color32::GREEN),
            )
            .clicked()
        {
            debug_session.continue_process();
        }
        if ui
            .add_enabled(
                state.process_can_continue,
                IconButton::new("⬇", "Step Into"),
            )
            .clicked()
        {
            debug_session.step_into();
        }
        if ui
            .add_enabled(
                state.process_can_continue,
                IconButton::new("⬈", "Step Over"),
            )
            .clicked()
        {
            debug_session.step_over();
        }
        if ui
            .add_enabled(state.can_step_out, IconButton::new("⬆", "Step Out"))
            .clicked()
        {
            debug_session.step_out();
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui.button("Quit").clicked() {
                app.show_confirmation_dialog = true;
            }
            ui.separator();
        })
    });
}
