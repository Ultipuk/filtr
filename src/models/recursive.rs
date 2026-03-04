use crate::i18n::tr;
use crate::parameter_section_row;
use egui::{DragValue, RichText, text::LayoutJob};
use serde::{Deserialize, Serialize};

use super::{MATH_FONT, math_text_format, math_text_format_sub};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RecursiveParameters {
    omega_h: f64,
    // Уровень усечения
    effective_length: f64,
}

impl Default for RecursiveParameters {
    fn default() -> Self {
        Self {
            omega_h: 0.00,
            effective_length: 0.0,
        }
    }
}

impl RecursiveParameters {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.show_omega_h(ui);
        self.show_effective_length(ui);
    }

    pub fn show_omega_h(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_row!(
            ui,
            RichText::new("δh").font(MATH_FONT.clone()),
            |ui: &mut egui::Ui| {
                ui.add(
                    DragValue::new(&mut self.omega_h)
                        .suffix(tr("unit-seconds"))
                        .range(0.0..=100.0)
                        .max_decimals_opt(super::MAX_DECIMALS_OPT),
                )
                .on_hover_text(tr("recursive-level-tooltip"));
            }
        );

        // parameter_menu_item(
        //     ui,
        //     RichText::new("δh").font(MATH_FONT.clone()),
        //     DragValue::new(&mut self.omega_h)
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Уровень ,определяющий эффективную длительность временной характеристики"),
        // );
    }

    pub fn show_effective_length(&mut self, ui: &mut egui::Ui) {
        let mut title = LayoutJob::default();

        title.append("T", 0.0, math_text_format(ui));
        title.append("h", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.effective_length)
                    .suffix(tr("unit-seconds"))
                    .range(0.0..=100.0)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("recursive-duration-tooltip"));
        });

        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.effective_length)
        //         .suffix(" сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Длительность определения временной характеристики"),
        // );
    }
    pub fn get_dh(&self) -> f64 {
        self.omega_h
    }

    pub fn get_th(&self) -> f64 {
        self.effective_length
    }

    pub fn get_nt(&self, dt: f64) -> i32 {
        (self.get_th() / dt + 1.1) as i32
    }
}
