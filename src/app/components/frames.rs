use egui::{ScrollArea, Ui};

use crate::app::App;

pub fn add(app: &mut App, ui: &mut Ui) {
    ScrollArea::both()
        .id_source("frames")
        .auto_shrink(false)
        .show(ui, |ui| {
            egui::Grid::new("frames")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    let mut selected_frame_id = app
                        .target
                        .process()
                        .selected_thread()
                        .selected_frame()
                        .frame_id();
                    for frame in app.target.process().selected_thread().frames() {
                        if let Some(line_entry) = frame.line_entry() {
                            ui.label(format!(
                                "{}:{}",
                                line_entry.filespec().filename(),
                                line_entry.line(),
                            ));
                        } else {
                            ui.label("");
                        }

                        if ui
                            .selectable_value(
                                &mut selected_frame_id,
                                frame.frame_id(),
                                frame.display_function_name().unwrap(),
                            )
                            .clicked()
                        {
                            app.target
                                .process()
                                .selected_thread()
                                .set_selected_frame(frame.frame_id());
                        }
                        ui.end_row();
                    }
                });
        });
}
