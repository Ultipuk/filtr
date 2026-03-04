use eframe::emath::Align;
use egui::{Layout, Response, Ui, WidgetText};

pub fn parameter_menu_button(ui: &mut Ui, title: impl Into<WidgetText>) -> Response {
    let mut response = None;
    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
        response = Some(ui.button(title.into()));
    });
    ui.end_row();

    response.unwrap_or_else(|| ui.label(""))
}

// NOTE: Legacy menu/table helpers are intentionally commented out.
// They can be restored if that UI path is reintroduced.
