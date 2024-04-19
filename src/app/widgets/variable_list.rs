use egui::{CollapsingHeader, Response, Ui, Widget};
use lldb::SBValue;

pub struct VariableList<'a> {
    values: Box<dyn Iterator<Item = SBValue> + 'a>,
}

impl<'a> VariableList<'a> {
    pub fn new(values: impl Iterator<Item = SBValue> + 'a) -> Self {
        Self {
            values: Box::new(values),
        }
    }
}

impl<'a> Widget for VariableList<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Grid::new(ui.next_auto_id())
            .num_columns(3)
            .striped(true)
            .show(ui, |ui| {
                for v in self.values {
                    if v.children().count() > 0 {
                        CollapsingHeader::new(v.name().expect("name should be present"))
                            .id_source(ui.next_auto_id())
                            .show(ui, |ui| {
                                ui.add(VariableList::new(v.children()));
                            });
                    } else {
                        ui.label(v.name().unwrap_or_default());
                        ui.label(v.display_type_name().unwrap_or_default());
                        ui.label(v.value().unwrap_or_default());
                    }
                    ui.end_row();
                }
            })
            .response
    }
}
