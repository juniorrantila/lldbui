use egui::{Color32, Response, Sense, Ui, Vec2, Widget};

pub struct IconBreakpoint {
    color: Color32,
}

impl IconBreakpoint {
    pub fn new(color: Color32) -> Self {
        Self { color }
    }
}

impl Widget for IconBreakpoint {
    fn ui(self, ui: &mut Ui) -> Response {
        let row_height = ui.spacing().interact_size.y;
        let padding = row_height / 2.;
        let (rect, resp) = ui.allocate_at_least(Vec2::new(row_height, row_height), Sense::click());
        ui.painter()
            .circle_filled(rect.center(), (row_height - padding) / 2., self.color);
        resp
    }
}
