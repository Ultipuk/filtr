use egui::{Response, Ui, WidgetText};

use crate::parameter_section_row;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TripleButtonAction {
    Left,
    Center,
    Right,
}

pub fn parameter_section_triple_button_row(
    ui: &mut Ui,
    label: impl Into<WidgetText>,
    left: impl Into<WidgetText>,
    left_tooltip: impl Into<WidgetText>,
    center: impl Into<WidgetText>,
    center_tooltip: impl Into<WidgetText>,
    right: impl Into<WidgetText>,
    right_tooltip: impl Into<WidgetText>,
) -> (Response, Option<TripleButtonAction>) {
    let mut clicked = None;
    let response = parameter_section_row!(ui, label, |ui: &mut egui::Ui| {
        let spacing = ui.style().spacing.item_spacing.x;
        let width = ui.available_width();
        let button_width = ((width - 2.0 * spacing) / 3.0).max(1.0);
        let button_height = ui.spacing().interact_size.y;

        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            if ui
                .add_sized(
                    [button_width, button_height],
                    egui::Button::new(left.into()),
                )
                .on_hover_text(left_tooltip.into())
                .clicked()
            {
                clicked = Some(TripleButtonAction::Left);
            }
            if ui
                .add_sized(
                    [button_width, button_height],
                    egui::Button::new(center.into()),
                )
                .on_hover_text(center_tooltip.into())
                .clicked()
            {
                clicked = Some(TripleButtonAction::Center);
            }
            if ui
                .add_sized(
                    [button_width, button_height],
                    egui::Button::new(right.into()),
                )
                .on_hover_text(right_tooltip.into())
                .clicked()
            {
                clicked = Some(TripleButtonAction::Right);
            }
        });
    });

    (response, clicked)
}
