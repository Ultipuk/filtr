use egui::{Align, Grid, Id, Layout, Response, Ui, Widget, WidgetText};

pub struct ParameterSectionButtonGroup<'a, Value: PartialEq + Copy> {
    id: Id,
    selected: &'a mut Value,
    values: Vec<(Value, WidgetText)>,
}

impl<'a, T> ParameterSectionButtonGroup<'a, T>
where
    T: PartialEq + Copy,
{
    pub fn new(id: impl core::hash::Hash, selected: &'a mut T) -> Self {
        Self {
            id: Id::new(id),
            selected,
            values: Vec::new(),
        }
    }

    pub fn with_value(mut self, value: T, text: impl Into<WidgetText>) -> Self {
        self.values.push((value, text.into()));
        self
    }
}

impl<'a, T> Widget for ParameterSectionButtonGroup<'a, T>
where
    T: PartialEq + Copy,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let width = ui.available_width();
        let col_width = width / self.values.len() as f32;

        Grid::new(self.id)
            .num_columns(self.values.len())
            .max_col_width(col_width)
            .show(ui, |ui| {
                for (value, label) in self.values {
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        ui.selectable_value(self.selected, value, label);
                    });
                }
            })
            .response
    }
}

pub fn parameter_section_button_group<T: PartialEq + Copy>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    selected: &mut T,
    items: impl IntoIterator<Item = (T, impl Into<WidgetText>)>,
) -> Response {
    let mut group = ParameterSectionButtonGroup::new(id, selected);
    for (val, label) in items {
        group = group.with_value(val, label);
    }
    group.ui(ui)
}
