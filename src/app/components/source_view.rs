use std::{path::PathBuf, sync::atomic::Ordering};

use egui::{Align, RichText, ScrollArea, Ui};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};

use crate::app::App;

pub fn add(app: &mut App, ui: &mut Ui) {
    let frame = app.debug_session.selected_frame();
    if let Some(line_entry) = frame.line_entry() {
        let path: PathBuf = [
            line_entry.filespec().directory(),
            line_entry.filespec().filename(),
        ]
        .iter()
        .collect();

        let key = path.clone().into_os_string().into_string().unwrap();
        ui.label(&key);
        ui.separator();

        if let Some(source) = app.debug_session.get_source(&path) {
            // TODO(ds): use proper mapping
            let compile_unit = frame.compile_unit();
            let mut language = format!("{:?}", compile_unit.language());
            if language == "C99" || language == "C11" {
                language = "C".to_string();
            }

            let theme = &CodeTheme::from_style(ui.style());
            let line_entry_color = ui.style().visuals.warn_fg_color;

            ScrollArea::both()
                .auto_shrink(false)
                .animated(false)
                .show(ui, |ui| {
                    egui::Grid::new("source").num_columns(3).show(ui, |ui| {
                        let mut i = 0;
                        for line in source.lines() {
                            i += 1;
                            if i == line_entry.line() {
                                ui.label(RichText::new("→").size(18.0).color(line_entry_color));
                            } else {
                                ui.label("");
                            }
                            let mut line_number = RichText::new(format!("{}", i));
                            if i == line_entry.line() {
                                line_number = line_number.color(line_entry_color);
                            }
                            ui.label(line_number);
                            let layout_job = highlight(ui.ctx(), theme, line, &language);
                            let response = ui.add(egui::Label::new(layout_job).selectable(true));
                            if i == line_entry.line()
                                && app.debug_session_reset.load(Ordering::Relaxed)
                            {
                                response.scroll_to_me(Some(Align::Center));
                                app.debug_session_reset.store(false, Ordering::Relaxed);
                            }
                            ui.end_row();
                        }
                    })
                });
        }
    }
}
