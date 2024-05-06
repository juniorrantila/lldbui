use egui::{Color32, Pos2, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct IconArrow {
    color: Color32,
}

impl IconArrow {
    pub fn new(color: Color32) -> Self {
        Self { color }
    }
}

impl Widget for IconArrow {
    fn ui(self, ui: &mut Ui) -> Response {
        let row_height = ui.spacing().interact_size.y;
        let padding = row_height / 2.;
        let (rect, resp) = ui.allocate_at_least(Vec2::new(row_height, row_height), Sense::hover());
        ui.painter().arrow(
            Pos2::new(rect.min.x + padding, rect.min.y + (rect.height() / 2.)),
            Vec2::new(row_height - padding, 0.),
            Stroke::new(2., self.color),
        );
        resp
    }
}
