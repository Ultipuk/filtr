use crate::i18n::tr;
use crate::parameter_section_row;
use egui::{DragValue, RichText, text::LayoutJob};
use serde::{Deserialize, Serialize};

use super::{MATH_FONT, math_text_format, math_text_format_sub};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DesignParameters {
    // df
    freq_sampling_rate: f64,
    // on
    lower_fr_frequency: f64,
    // ox
    upper_fr_frequency: f64,
}

impl Default for DesignParameters {
    fn default() -> Self {
        Self {
            freq_sampling_rate: 0.0,
            lower_fr_frequency: 0.0,
            upper_fr_frequency: 0.0,
        }
    }
}

impl DesignParameters {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.show_sampling_rate(ui);
        self.show_lower_fr_frequency(ui);
        self.show_upper_fr_frequency(ui);
    }

    pub fn show_sampling_rate(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_row!(
            ui,
            RichText::new("Δω").font(MATH_FONT.clone()),
            |ui: &mut egui::Ui| {
                ui.add(
                    DragValue::new(&mut self.freq_sampling_rate)
                        .range(0.0..=100.0)
                        .suffix(tr("unit-rad-sec"))
                        .max_decimals_opt(super::MAX_DECIMALS_OPT),
                )
                .on_hover_text(tr("dw-tooltip"));
            }
        );
        // parameter_menu_item(
        //     ui,
        //     RichText::new("Δω").font(MATH_FONT.clone()),
        //     DragValue::new(&mut self.freq_sampling_rate)
        //         .suffix(" рад/сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Шаг дискретизации по частоте"),
        // );
    }

    pub fn show_lower_fr_frequency(&mut self, ui: &mut egui::Ui) {
        let mut title = LayoutJob::default();

        title.append("ω", 0.0, math_text_format(ui));
        title.append("s", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.lower_fr_frequency)
                    .range(0.0..=self.upper_fr_frequency)
                    .suffix(tr("unit-rad-sec"))
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("design-low-tooltip"));
        });
        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.lower_fr_frequency)
        //         .suffix(" рад/сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=self.upper_fr_frequency),
        //     Label::new("Нижняя частота диапазона определения АЧХ и ФЧХ"),
        // );
    }

    pub fn show_upper_fr_frequency(&mut self, ui: &mut egui::Ui) {
        let mut title = LayoutJob::default();

        title.append("Ω", 0.0, math_text_format(ui));
        title.append("B", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.upper_fr_frequency)
                    .range(self.lower_fr_frequency..=100.0)
                    .suffix(tr("unit-rad-sec"))
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("design-high-tooltip"));
        });
        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.upper_fr_frequency)
        //         .suffix(" рад/сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(self.lower_fr_frequency..=100.0),
        //     Label::new("Верхняя частота диапазона определения АЧХ и ФЧХ"),
        // );
    }

    pub fn get_df(&self) -> f64 {
        self.freq_sampling_rate
    }

    pub fn get_os(&self) -> f64 {
        self.lower_fr_frequency
    }

    pub fn get_of(&self) -> f64 {
        self.upper_fr_frequency
    }

    pub fn get_ks(&self) -> i32 {
        (self.get_os() / self.get_df() + 1.1) as i32
    }

    pub fn get_kf(&self) -> i32 {
        (self.get_of() / self.get_df() + 1.1) as i32
    }
}
