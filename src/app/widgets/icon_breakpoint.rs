use egui::{Response, Sense, Ui, Vec2, Widget};

pub struct IconBreakpoint {}

impl IconBreakpoint {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for IconBreakpoint {
    fn ui(self, ui: &mut Ui) -> Response {
        let breakpoint_color = ui.style().visuals.error_fg_color;
        let row_height = ui.spacing().interact_size.y;
        let (rect, resp) = ui.allocate_at_least(Vec2::new(row_height, row_height), Sense::hover());
        ui.painter().circle_filled(
            rect.center(),
            (row_height - (row_height / 2.)) / 2.,
            breakpoint_color,
        );
        resp
    }
}
