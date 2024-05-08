use std::collections::HashMap;

use egui::{CollapsingHeader, CursorIcon, Label, Response, Sense, Ui, Widget};
use lldb::{SBTarget, SBValue, SBWatchpoint};

/// `VariableList` renders a nested list of debugger values.
pub struct VariableList<'a> {
    values: Box<dyn Iterator<Item = SBValue> + 'a>,
    target: &'a SBTarget,
}

impl<'a> VariableList<'a> {
    pub fn new(values: impl Iterator<Item = SBValue> + 'a, target: &'a SBTarget) -> Self {
        Self {
            values: Box::new(values),
            target,
        }
    }
}

impl<'a> Widget for VariableList<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let watchpoints: HashMap<u64, SBWatchpoint> = self
            .target
            .watchpoints()
            .map(|wp| (wp.watch_address(), wp))
            .collect();

        egui::Grid::new(ui.next_auto_id())
            .num_columns(3)
            .striped(true)
            .show(ui, |ui| {
                for v in self.values {
                    if v.children().count() > 0 {
                        CollapsingHeader::new(v.name().expect("name should be present"))
                            .id_source(ui.next_auto_id())
                            .show(ui, |ui| {
                                ui.add(VariableList::new(v.children(), self.target));
                            });
                    } else {
                        if let Some(load_address) = v.load_address() {
                            if let Some(wp) = watchpoints.get(&load_address) {
                                if ui
                                    .add(
                                        Label::new(v.name().unwrap_or_default())
                                            .sense(Sense::click()),
                                    )
                                    .on_hover_cursor(CursorIcon::Default)
                                    .on_hover_text_at_pointer(format!(
                                        "unwatch {:#x}",
                                        wp.watch_address()
                                    ))
                                    .clicked()
                                {
                                    self.target.delete_watchpoint(wp.id());
                                }
                            } else if ui
                                .add(Label::new(v.name().unwrap_or_default()).sense(Sense::click()))
                                .on_hover_cursor(CursorIcon::Default)
                                .on_hover_text_at_pointer(format!("watch {:#x}", load_address))
                                .clicked()
                            {
                                match v.watch(true, false, true) {
                                    Ok(wp) => {
                                        tracing::debug!("Watchpoint created: {:?}", wp);
                                    }
                                    Err(err) => {
                                        tracing::error!("Failed to create watchpoint: {}", err)
                                    }
                                }
                            }
                        } else {
                            ui.label(v.name().unwrap_or_default());
                        }
                        ui.label(v.display_type_name().unwrap_or_default());
                        ui.label(v.value().unwrap_or_default());
                    }
                    ui.end_row();
                }
            })
            .response
    }
}
