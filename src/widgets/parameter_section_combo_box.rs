use egui::{Id, Response, Ui, Widget, WidgetText};

pub struct ParameterSectionComboBox<'a, Value: PartialEq + Copy> {
    id: Id,
    text: WidgetText,
    selected: &'a mut Value,
    values: Vec<(Value, WidgetText)>,
}

impl<'a, T> ParameterSectionComboBox<'a, T>
where
    T: PartialEq + Copy,
{
    pub fn new(
        id: impl core::hash::Hash,
        text: impl Into<WidgetText>,
        selected: &'a mut T,
    ) -> Self {
        Self {
            id: Id::new(id),
            text: text.into(),
            selected,
            values: Vec::new(),
        }
    }

    pub fn with_value(mut self, value: T, text: impl Into<WidgetText>) -> Self {
        self.values.push((value, text.into()));
        self
    }
}

impl<T> Widget for ParameterSectionComboBox<'_, T>
where
    T: PartialEq + Copy,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let width = ui.available_width();
        let mut label_response: Option<Response> = None;
        let mut combo_response: Option<Response> = None;

        ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                ui.set_max_height(18.0);

                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::exact(width * 0.4))
                    .size(egui_extras::Size::remainder())
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(
                                    ui.available_width(),
                                    ui.text_style_height(&egui::TextStyle::Body),
                                ),
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    label_response = Some(ui.add(egui::Label::new(self.text)));
                                },
                            );
                        });

                        strip.cell(|ui| {
                            let selected_text = self
                                .values
                                .iter()
                                .find_map(|(v, w)| {
                                    if v == self.selected {
                                        Some(w.text())
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or("...");

                            combo_response = Some(
                                egui::ComboBox::from_id_salt(self.id)
                                    .width(ui.available_width())
                                    .selected_text(selected_text)
                                    .show_ui(ui, |ui| {
                                        for (value, text) in self.values {
                                            ui.selectable_value(self.selected, value, text);
                                        }
                                    })
                                    .response,
                            );
                        });
                    });
            },
        );

        match (label_response, combo_response) {
            (Some(label), Some(combo)) => label.union(combo),
            (Some(label), None) => label,
            (None, Some(combo)) => combo,
            (None, None) => ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover()),
        }
    }
}

pub fn parameter_section_combo_box<T: PartialEq + Copy>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    text: impl Into<WidgetText>,
    selected: &mut T,
    items: impl IntoIterator<Item = (T, impl Into<WidgetText>)>,
) -> Response {
    let mut combo_box = ParameterSectionComboBox::new(id, text, selected);
    for (val, label) in items {
        combo_box = combo_box.with_value(val, label);
    }
    combo_box.ui(ui)
}
