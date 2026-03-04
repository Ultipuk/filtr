use egui::{Button, IntoAtoms, Response, Ui};

pub fn parameter_section_button<'a>(ui: &mut Ui, atoms: impl IntoAtoms<'a>) -> Response {
    ui.add_sized([ui.available_width(), 0.0], Button::new(atoms))
}
