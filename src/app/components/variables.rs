use egui::{ScrollArea, Ui};

use crate::app::{widgets::VariableList, App, VariablesTab};

pub fn add(app: &mut App, ui: &mut Ui) {
    let frame = app.debug_session.selected_frame();

    ui.horizontal(|ui| {
        ui.selectable_value(&mut app.variables_tab, VariablesTab::Locals, "locals");
        ui.selectable_value(&mut app.variables_tab, VariablesTab::Statics, "statics");
        ui.selectable_value(&mut app.variables_tab, VariablesTab::Arguments, "arguments");
        ui.selectable_value(&mut app.variables_tab, VariablesTab::Registers, "registers");
    });
    ScrollArea::both()
        .id_source("variables")
        .show(ui, |ui| match app.variables_tab {
            VariablesTab::Locals => {
                ui.add(VariableList::new(frame.locals().iter()));
            }
            VariablesTab::Statics => {
                ui.add(VariableList::new(frame.statics().iter()));
            }
            VariablesTab::Arguments => {
                ui.add(VariableList::new(frame.arguments().iter()));
            }
            VariablesTab::Registers => {
                ui.add(VariableList::new(frame.registers().iter()));
            }
        });
}
