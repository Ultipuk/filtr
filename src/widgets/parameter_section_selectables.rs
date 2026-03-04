use egui::{Align, Id, Layout, Response, Ui, Widget, WidgetText};

use crate::parameter_section_multirow;

pub struct ParameterSectionSelectables<'a, Value: PartialEq + Copy> {
    id: Id,
    text: WidgetText,
    selected: &'a mut Vec<Value>,
    values: Vec<(Value, WidgetText)>,
}

impl<'a, T> ParameterSectionSelectables<'a, T>
where
    T: PartialEq + Copy,
{
    pub fn new(
        id: impl core::hash::Hash,
        text: impl Into<WidgetText>,
        selected: &'a mut Vec<T>,
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

impl<'a, T> Widget for ParameterSectionSelectables<'a, T>
where
    T: PartialEq + Copy,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let rows = self.values.len();

        parameter_section_multirow!(
            ui,
            self.text,
            |ui: &mut Ui| {
                for (value, label) in self.values {
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        ui.selectable_label(self.selected.contains(&value), label)
                            .clicked()
                            .then(|| {
                                if !self.selected.contains(&value) {
                                    self.selected.push(value);
                                } else {
                                    self.selected.retain(|&x| x != value);
                                }
                            });
                    });
                }
            },
            rows
        )
    }
}

pub fn parameter_section_selectables<T: PartialEq + Copy>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    text: impl Into<WidgetText>,
    selected: &mut Vec<T>,
    items: impl IntoIterator<Item = (T, impl Into<WidgetText>)>,
) -> Response {
    let mut w = ParameterSectionSelectables::new(id, text, selected);
    for (val, label) in items {
        w = w.with_value(val, label);
    }
    w.ui(ui)
}
