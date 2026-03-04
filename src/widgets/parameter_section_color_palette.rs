use egui::{Color32, Response, Stroke, Ui};

pub fn parameter_section_color_palette(
    ui: &mut Ui,
    selected_index: &mut usize,
    colors: &[(String, Color32)],
) -> Response {
    ui.horizontal(|ui| {
        let count = colors.len().max(1) as f32;
        let spacing = ui.spacing().item_spacing.x;
        let height = ui.spacing().interact_size.y;
        let min_width = height;
        let total_spacing = spacing * (count - 1.0);
        let width = ((ui.available_width() - total_spacing) / count).max(min_width);

        for (index, (name, color)) in colors.iter().enumerate() {
            let selected = *selected_index == index;
            let stroke = if selected {
                Stroke::new(2.0, ui.visuals().strong_text_color())
            } else {
                Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color)
            };
            let button = egui::Button::new("")
                .min_size(egui::vec2(min_width, height))
                .fill(*color)
                .stroke(stroke);
            if ui
                .add_sized([width, height], button)
                .on_hover_text(name)
                .clicked()
            {
                *selected_index = index;
            }
        }
    })
    .response
}
