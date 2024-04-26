use egui::{Align, Color32, Layout, Ui};

use crate::app::{widgets::IconButton, App};

pub fn add(app: &mut App, ui: &mut Ui) {
    let debug_session = &mut app.debug_session;

    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                debug_session.process_is_running(),
                IconButton::new_with_color("⏸", "Stop", Color32::RED),
            )
            .clicked()
        {
            debug_session.process_stop();
        }
        if ui
            .add_enabled(
                debug_session.process_can_continue(),
                IconButton::new_with_color("⏵", "Continue", Color32::GREEN),
            )
            .clicked()
        {
            debug_session.process_continue();
        }
        if ui
            .add_enabled(
                debug_session.process_can_continue(),
                IconButton::new("⬇", "Step Into"),
            )
            .clicked()
        {
            debug_session.step_into();
        }
        if ui
            .add_enabled(
                debug_session.process_can_continue(),
                IconButton::new("⬈", "Step Over"),
            )
            .clicked()
        {
            debug_session.step_over();
        }
        if ui
            .add_enabled(
                debug_session.has_parent_frame(),
                IconButton::new("⬆", "Step Out"),
            )
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
