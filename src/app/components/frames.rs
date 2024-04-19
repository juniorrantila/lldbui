use std::sync::atomic::Ordering;

use egui::{ScrollArea, Ui};

use crate::app::App;

pub fn add(app: &mut App, ui: &mut Ui) {
    let mut selected_frame_id = app.debug_session.selected_frame_id();

    ui.label("frames");
    ScrollArea::both().id_source("frames").show(ui, |ui| {
        egui::Grid::new("frames")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                for frame in app.debug_session.thread_frames() {
                    if ui
                        .selectable_value(
                            &mut selected_frame_id,
                            frame.frame_id(),
                            frame.display_function_name().unwrap(),
                        )
                        .clicked()
                    {
                        app.debug_session.select_frame(&frame);
                        // TODO(ds): remove once we fix the receiving of thread events
                        app.debug_session_reset.store(true, Ordering::Relaxed);
                    }
                    if let Some(line_entry) = frame.line_entry() {
                        ui.label(format!(
                            "{}:{}",
                            line_entry.filespec().filename(),
                            line_entry.line(),
                        ));
                    }
                    ui.end_row();
                }
            });
    });
}
