use egui::Ui;

use crate::app::App;

pub fn add(app: &App, ui: &mut Ui) {
    let state = app.debug_session.state.lock().unwrap();

    egui::Grid::new("Process")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Target:");
            ui.label(state.executable);
            ui.end_row();

            ui.label("Args:");
            ui.label(state.process_args.join(" "));
            ui.end_row();

            ui.label("State:");
            ui.label(format!("{:?}", state.process_state));
            ui.end_row();

            ui.label("PID:");
            ui.label(format!("{}", state.process_pid));
            ui.end_row();
        });
}
