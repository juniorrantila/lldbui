use egui::{Color32, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct IconBreakpoint {
    enabled: bool,
}

impl IconBreakpoint {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Widget for IconBreakpoint {
    fn ui(self, ui: &mut Ui) -> Response {
        let row_height = ui.spacing().interact_size.y;
        let padding = row_height / 2.;
        let (rect, resp) = ui.allocate_at_least(Vec2::new(row_height, row_height), Sense::click());
        let radius = (row_height - padding) / 2.;
        if self.enabled {
            ui.painter()
                .circle_filled(rect.center(), radius, ui.visuals().error_fg_color);
        } else if resp.hovered() {
            ui.painter().circle(
                rect.center(),
                radius,
                Color32::TRANSPARENT,
                Stroke::new(1., ui.visuals().error_fg_color),
            );
        }
        resp
    }
}
