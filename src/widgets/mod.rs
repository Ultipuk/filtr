mod parameter_section;
mod parameter_section_button;
mod parameter_section_checkbox_row;
// mod parameter_section_button_group;
mod parameter_section_color_palette;
mod parameter_section_combo_box;
mod parameter_section_triple_button_row;
// mod parameter_section_selectables;
mod parameter_section_separator;
mod parameter_section_value;

pub use parameter_section::*;
pub use parameter_section_button::*;
pub use parameter_section_checkbox_row::*;
// pub use parameter_section_button_group::*;
pub use parameter_section_color_palette::*;
pub use parameter_section_combo_box::*;
pub use parameter_section_triple_button_row::*;
// pub use parameter_section_selectables::*;
pub use parameter_section_separator::*;
pub use parameter_section_value::*;

pub mod parameter_menu;

#[macro_export]
macro_rules! parameter_section_row {
    (
        $ui:expr,
        $label:expr,
        $content:expr
    ) => {{
        let width = $ui.available_width();

        $ui.with_layout(
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
                                    ui.add(egui::Label::new(
                                        std::convert::Into::<egui::WidgetText>::into($label),
                                    ));
                                },
                            );
                        });

                        strip.cell(|ui| $content(ui));
                    });
            },
        )
        .response
    }};
}

#[macro_export]
macro_rules! parameter_section_multirow {
    (
        $ui:expr,
        $label:expr,
        $content:expr,
        $rows:expr
    ) => {{
        let width = $ui.available_width();

        $ui.with_layout(
            egui::Layout::top_down_justified(egui::Align::Center),
            |ui| {
                ui.set_max_height(18.0 * ($rows as f32));

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
                                    ui.add(egui::Label::new(
                                        std::convert::Into::<egui::WidgetText>::into($label),
                                    ));
                                },
                            );
                        });

                        strip.cell(|ui| $content(ui));
                    });
            },
        )
        .response
    }};
}
