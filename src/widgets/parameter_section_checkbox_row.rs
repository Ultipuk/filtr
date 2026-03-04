use egui::{Response, Ui, WidgetText};

use crate::parameter_section_row;

pub fn parameter_section_checkbox_row(
    ui: &mut Ui,
    label: impl Into<WidgetText>,
    checked: &mut bool,
    text: impl Into<WidgetText>,
) -> Response {
    parameter_section_row!(ui, label, |ui: &mut egui::Ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.checkbox(checked, text.into());
        });
    })
}
