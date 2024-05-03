use egui::{Align, ScrollArea, Ui};

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
        .animated(false)
        .show(ui, |ui| match app.console_tab {
            ConsoleTab::Console => {
                ui.label(&app.console_output);
                let response = ui.add(
                    egui::TextEdit::singleline(&mut app.console_input)
                        .hint_text("lldb command")
                        .desired_width(f32::INFINITY),
                );
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    app.console_output
                        .push_str(&format!("(lldb) {}\n", app.console_input));
                    match app.target.debugger().execute_command(&app.console_input) {
                        Ok(result) => app.console_output.push_str(result),
                        Err(err) => app.console_output.push_str(&err),
                    }
                    app.console_output.push('\n');
                    app.console_input.clear();
                    response.scroll_to_me(Some(Align::Center));
                    response.request_focus();
                }
            }
            ConsoleTab::Stdout => {
                if let Some(output) = app.target.process().get_stdout_all() {
                    app.process_stdout.push_str(&output);
                }
                ui.label(app.process_stdout.as_str());
            }
            ConsoleTab::Stderr => {
                // TODO(ds): somehow stderr of the process ends up in stdout and this is always empty?
                // https://github.com/llvm/llvm-project/issues/25350#issuecomment-980951241
                if let Some(output) = app.target.process().get_stderr() {
                    app.process_stderr.push_str(&output);
                }
                ui.label(app.process_stderr.as_str());
            }
        });
}
