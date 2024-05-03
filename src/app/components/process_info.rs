use egui::Ui;

use crate::app::App;

pub fn add(app: &App, ui: &mut Ui) {
    egui::Grid::new("Process")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Target:");
            if let Some(executable) = app.target.executable() {
                ui.label(executable.filename());
            }
            ui.end_row();

            ui.label("Args:");
            ui.label(
                app.target
                    .get_launch_info()
                    .arguments()
                    .collect::<Vec<&str>>()
                    .join(" "),
            );
            ui.end_row();

            ui.label("State:");
            ui.label(format!("{:?}", app.target.process().state()));
            ui.end_row();

            ui.label("PID:");
            ui.label(format!("{}", app.target.process().process_id()));
            ui.end_row();
        });
}
