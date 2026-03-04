use egui::{Response, Ui, WidgetText};

use crate::i18n::tr;
use crate::parameter_section_row;

pub fn parameter_section_value(
    ui: &mut Ui,
    title: impl Into<WidgetText>,
    value: impl Into<String>,
) -> Response {
    let value = value.into();

    parameter_section_row!(ui, title, |ui: &mut Ui| {
        let visuals = ui.visuals().widgets.inactive;
        egui::Frame::new()
            .fill(visuals.weak_bg_fill)
            .stroke(visuals.bg_stroke)
            .corner_radius(visuals.corner_radius)
            .inner_margin(egui::Margin::symmetric(6, 2))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let copy_clicked = ui
                        .add(egui::Button::new(tr("button-copy")).small())
                        .on_hover_text(tr("copy-tooltip"))
                        .clicked();

                    if copy_clicked {
                        ui.ctx().copy_text(value.clone());
                    }

                    ui.add(egui::Label::new(value.as_str()).truncate());
                });
            });
    })
}
