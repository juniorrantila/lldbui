use egui::{ScrollArea, Ui};

use crate::app::{App, ConsoleTab};

pub fn add(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut app.console_tab, ConsoleTab::Console, "console");
        ui.selectable_value(&mut app.console_tab, ConsoleTab::Stdout, "stdout");
        ui.selectable_value(&mut app.console_tab, ConsoleTab::Stderr, "stderr");
    });
    ScrollArea::both()
        .auto_shrink(false)
        .stick_to_bottom(true)
        .show(ui, |ui| match app.console_tab {
            ConsoleTab::Console => {
                ui.label("console");
            }
            ConsoleTab::Stdout => {
                if let Some(output) = app.debug_session.get_stdout() {
                    app.stdout.push_str(&output);
                }
                ui.label(&app.stdout);
            }
            ConsoleTab::Stderr => {
                if let Some(output) = app.debug_session.get_stderr() {
                    app.stderr.push_str(&output);
                }
                ui.label(&app.stderr);
            }
        });
}
