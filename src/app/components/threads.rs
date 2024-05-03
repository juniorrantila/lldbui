use egui::{ScrollArea, Ui};
use lldb::SBThread;

use crate::app::App;

pub fn add(app: &mut App, ui: &mut Ui) {
    ScrollArea::both()
        .id_source("threads")
        .auto_shrink(false)
        .show(ui, |ui| {
            egui::Grid::new("threads")
                .num_columns(1)
                .striped(true)
                .show(ui, |ui| {
                    let mut selected_thread_id = app.target.process().selected_thread().thread_id();
                    for thread in app.target.process().threads() {
                        if ui
                            .selectable_value(
                                &mut selected_thread_id,
                                thread.thread_id(),
                                thread_label(&thread),
                            )
                            .clicked()
                        {
                            app.target.process().set_selected_thread(&thread);
                            app.source_file.clear(); // reset to make the source view scroll
                        }
                        ui.end_row();
                    }
                });
        });
}

fn thread_label(thread: &SBThread) -> String {
    let mut label = format!("{} {}", thread.thread_id(), thread.name().unwrap_or(""));
    if let Some(queue) = thread.queue() {
        label.push_str(&format!(" queue={}", queue.name()));
    }
    if thread.is_stopped() {
        label.push_str(&format!(" stop_reason={:?}", thread.stop_reason()));
    }
    label
}
