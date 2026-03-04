use egui::{DragValue, text::LayoutJob};
use serde::{Deserialize, Serialize};

use crate::i18n::tr;
use crate::models::{FilterCategory, math_text_format, math_text_format_sub};
use crate::parameter_section_row;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CutoffParameters {
    lower: f64,
    upper: f64,
}

impl CutoffParameters {
    pub fn get_on(&self) -> f64 {
        self.lower
    }

    pub fn get_ox(&self) -> f64 {
        self.upper
    }

    pub fn show(&mut self, ui: &mut egui::Ui, filter_category: &FilterCategory) {
        use FilterCategory::*;
        match filter_category {
            BandPass | BandStop | HighPass => {
                let mut title = LayoutJob::default();

                title.append("ω", 0.0, math_text_format(ui));
                title.append("φ", 0.0, math_text_format_sub(ui));

                let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
                    ui.add(
                        DragValue::new(&mut self.lower)
                            .speed(0.01)
                            .suffix(tr("unit-rad-sec"))
                            .max_decimals_opt(super::MAX_DECIMALS_OPT),
                    )
                    .on_hover_text(tr("cutoff-lower-tooltip"));
                });

                // parameter_menu_item(
                //     ui,
                //     title,
                //     DragValue::new(&mut self.lower)
                //         .speed(0.01)
                //         .suffix(" рад/сек")
                //         .max_decimals_opt(super::MAX_DECIMALS_OPT),
                //     // .clamprange(range),
                //     Label::new("Нижняя граничная частота полосы пропускания АЧХ фильтра"),
                // );
            }
            _ => {}
        }

        match filter_category {
            LowPass | BandPass | BandStop | Differentiating => {
                let mut title = LayoutJob::default();

                title.append("Ω", 0.0, math_text_format(ui));
                title.append("φ", 0.0, math_text_format_sub(ui));

                let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
                    ui.add(
                        DragValue::new(&mut self.upper)
                            .speed(0.01)
                            .suffix(tr("unit-rad-sec"))
                            .max_decimals_opt(super::MAX_DECIMALS_OPT),
                    )
                    .on_hover_text(tr("cutoff-upper-tooltip"));
                });

                // parameter_menu_item(
                //     ui,
                //     title,
                //     DragValue::new(&mut self.upper)
                //         .speed(0.01)
                //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
                //         .suffix(" рад/сек"),
                //     // .clamp_range(range),
                //     Label::new("Верхняя граничная частота полосы пропускания АЧХ фильтра"),
                // );
            }
            FilterCategory::HighPass => {}
        }
    }
}

impl Default for CutoffParameters {
    fn default() -> Self {
        Self {
            lower: 0.0,
            upper: 0.0,
        }
    }
}
