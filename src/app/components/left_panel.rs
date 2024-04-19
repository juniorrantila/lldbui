use egui::Ui;

use crate::app::App;

pub fn add(app: &mut App, ui: &mut Ui) {
    egui::Grid::new("Process")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Target:");
            ui.label(app.debug_session.process_name());
            ui.end_row();

            ui.label("State:");
            ui.label(app.debug_session.process_state());
            ui.end_row();

            ui.label("PID:");
            ui.label(format!("{}", app.debug_session.process_pid()));
            ui.end_row();
        });
}
