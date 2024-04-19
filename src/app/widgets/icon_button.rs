use egui::{Button, Color32, Response, RichText, Ui, Widget};

pub struct IconButton<'a> {
    label: &'a str,
    hover: &'a str,
    color: Option<Color32>,
}

impl<'a> IconButton<'a> {
    pub fn new(label: &'a str, hover: &'a str) -> Self {
        Self {
            label,
            hover,
            color: None,
        }
    }

    pub fn new_with_color(label: &'a str, hover: &'a str, color: Color32) -> Self {
        Self {
            label,
            hover,
            color: Some(color),
        }
    }
}

impl<'a> Widget for IconButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut label = RichText::new(self.label).monospace();
        if let Some(color) = self.color {
            label = label.color(color)
        }
        ui.add(Button::new(label))
            .on_hover_text(RichText::new(self.hover).small())
    }
}
