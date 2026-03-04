use egui::Ui;

pub fn parameter_section_separator(ui: &mut Ui) {
    let margin = ui.style().spacing.item_spacing;
    ui.add_space(margin.y);
}
