use super::{MATH_FONT, math_text_format, math_text_format_sub};
use crate::i18n::tr;
use crate::parameter_section_row;
use egui::{DragValue, RichText, text::LayoutJob};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CommonFilterParameters {
    // dm
    passband_ripple: f64,
    // dl
    stopband_ripple: f64,
    // v1
    transition_bandwidth: f64,
    // fk
    gain: f64,
}

impl CommonFilterParameters {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.show_passband_ripple(ui);
        self.show_stopband_ripple(ui);
        self.show_transition_bandwidth(ui);
        self.show_gain(ui);
    }
    pub fn show_passband_ripple(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_row!(
            ui,
            RichText::new("Δ").font(MATH_FONT.clone()),
            |ui: &mut egui::Ui| {
                ui.add(
                    DragValue::new(&mut self.passband_ripple)
                        .range(0.01..=0.2)
                        .speed(0.0001)
                        .max_decimals_opt(super::MAX_DECIMALS_OPT),
                )
                .on_hover_text(tr("passband-ripple-tooltip"));
            }
        );
        // parameter_menu_item_with_range(
        //     ui,
        //     RichText::new("Δ").font(MATH_FONT.clone()),
        //     DragValue::new(&mut self.passband_ripple).max_decimals_opt(super::MAX_DECIMALS_OPT),
        //     Label::new("Неравномерность АЧХ фильтра в полосе пропускания"),
        //     RichText::new("0.01≤"),
        //     RichText::new("≤0.02"),
        // );
    }

    pub fn show_stopband_ripple(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_row!(
            ui,
            RichText::new("δ").font(MATH_FONT.clone()),
            |ui: &mut egui::Ui| {
                ui.add(
                    DragValue::new(&mut self.stopband_ripple)
                        .range(0.0001..=0.5)
                        .speed(0.001)
                        .max_decimals_opt(super::MAX_DECIMALS_OPT),
                )
                .on_hover_text(tr("stopband-ripple-tooltip"));
            }
        );
        // parameter_menu_item_with_range(
        //     ui,
        //     RichText::new("δ").font(MATH_FONT.clone()),
        //     DragValue::new(&mut self.stopband_ripple).max_decimals_opt(super::MAX_DECIMALS_OPT),
        //     Label::new("Неравномерность АЧХ фильтра в полосе задерживания"),
        //     RichText::new("0.0001≤"),
        //     RichText::new("≤0.5"),
        // );
    }

    pub fn show_transition_bandwidth(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_row!(
            ui,
            RichText::new("v*").font(MATH_FONT.clone()),
            |ui: &mut egui::Ui| {
                ui.add(
                    DragValue::new(&mut self.transition_bandwidth)
                        .range(1.05..=2.5)
                        .speed(0.01)
                        .max_decimals_opt(super::MAX_DECIMALS_OPT),
                )
                .on_hover_text(tr("transition-band-tooltip"));
            }
        );
        // parameter_menu_item_with_range(
        //     ui,
        //     RichText::new("v*").font(MATH_FONT.clone()),
        //     DragValue::new(&mut self.transition_bandwidth)
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT),
        //     Label::new("Параметр, определяющий ширину переходных полос АЧХ"),
        //     RichText::new("1.05≤"),
        //     RichText::new("≤2.5"),
        // );
    }

    pub fn show_gain(&mut self, ui: &mut egui::Ui) {
        let mut title = LayoutJob::default();

        title.append("k", 0.0, math_text_format(ui));
        title.append("φ", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.gain)
                    .range(0.1..=10.0)
                    .speed(0.1)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("gain-tooltip"));
        });

        // parameter_menu_item_with_range(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.gain).max_decimals_opt(super::MAX_DECIMALS_OPT),
        //     Label::new("Коэффициент усиления в полосе пропускания АЧХ фильтра"),
        //     RichText::new("0.1≤"),
        //     RichText::new("≤10"),
        // );
    }
}

impl Default for CommonFilterParameters {
    fn default() -> Self {
        Self {
            passband_ripple: 0.0,
            stopband_ripple: 0.0,
            transition_bandwidth: 0.0,
            gain: 0.0,
        }
    }
}

impl CommonFilterParameters {
    pub fn get_dm(&self) -> f64 {
        self.passband_ripple
    }

    pub fn get_dl(&self) -> f64 {
        self.stopband_ripple
    }

    pub fn get_v1(&self) -> f64 {
        self.transition_bandwidth
    }

    pub fn get_fk(&self) -> f64 {
        self.gain
    }
}
