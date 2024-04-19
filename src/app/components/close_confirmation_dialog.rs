use egui::Context;

use crate::app::App;

pub fn add(app: &mut App, ctx: &Context) {
    // Handle close requests
    if ctx.input(|i| i.viewport().close_requested()) {
        if app.allowed_to_close {
            // do nothing - we will close
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            app.show_confirmation_dialog = true;
        }
    }
    if app.show_confirmation_dialog {
        egui::Window::new("Really close?")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("No").clicked() {
                        app.show_confirmation_dialog = false;
                        app.allowed_to_close = false;
                    }

                    if ui.button("Yes").clicked() {
                        app.show_confirmation_dialog = false;
                        app.allowed_to_close = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
    }
}
