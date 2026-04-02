use egui::{CollapsingHeader, Ui, WidgetText};

/// Main container for parameter sections.
pub struct ParameterSection {
    title: WidgetText,
    default_open_: bool,
    fill: Option<egui::Color32>,
}

impl ParameterSection {
    pub fn new(title: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            default_open_: true,
            fill: None,
        }
    }

    pub fn with_fill(mut self, fill: egui::Color32) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn default_open(mut self, value: bool) -> Self {
        self.default_open_ = value;
        self
    }

    pub fn show<R>(&mut self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) {
        let available_width = ui.available_width();
        ui.set_width(available_width);

        let color = self.fill.unwrap_or(ui.style().visuals.faint_bg_color);
        let margin = ui.style().spacing.item_spacing;

        egui::Frame::new()
            .fill(color)
            .inner_margin(margin)
            .show(ui, |ui| {
                let width = ui.available_width();
                let half_width = (width - ui.spacing().item_spacing.x) * 0.5;
                let expanded_width = half_width + ui.spacing().item_spacing.x;

                let openness = CollapsingHeader::new(self.title.text())
                    .default_open(self.default_open_)
                    .show_unindented(ui, |ui| content(ui))
                    .openness;

                ui.set_width(half_width + (1.0 - openness) * expanded_width);
            });
    }
}
