use egui::{ScrollArea, Ui};
use lldb::SBThread;

use crate::app::App;

pub fn add(app: &mut App, ui: &mut Ui) {
    let mut selected_thread_id = app.debug_session.selected_thread_id();
    ScrollArea::both()
        .id_source("threads")
        .auto_shrink(false)
        .show(ui, |ui| {
            egui::Grid::new("threads")
                .num_columns(1)
                .striped(true)
                .show(ui, |ui| {
                    for thread in app.debug_session.process_threads() {
                        if ui
                            .selectable_value(
                                &mut selected_thread_id,
                                thread.thread_id(),
                                thread_label(&thread),
                            )
                            .clicked()
                        {
                            app.debug_session.select_thread(&thread);
                            // (ds): lldb does not publish thread changed events
                            //       when the thread is changed via the API.
                            //       So we need to manually trigger a redraw.
                            app.scroll_source_view();
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
