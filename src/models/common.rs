use egui::{DragValue, RichText};
use serde::{Deserialize, Serialize};

use crate::i18n::tr;
use crate::models::{FilterCategory, FilterMode, FilterType};
use crate::parameter_section_row;
use crate::widgets::parameter_section_combo_box;

use super::MATH_FONT;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CommonParameters {
    // IW
    pub mode: FilterMode,
    // IL
    pub filter_type: FilterType,
    // IN
    pub filter_category: FilterCategory,
    // DT
    pub sampling_rate: f64,
}

impl CommonParameters {
    pub fn show_filter_mode(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_combo_box(
            ui,
            "filter_mode",
            tr("label-mode"),
            &mut self.mode,
            [
                (FilterMode::Design, tr("option-mode-design")),
                (FilterMode::Operation, tr("option-mode-operation")),
            ],
        );
        // let mut filter_mode = SelectButtonGroup::new("filter_mode", &mut self.mode);

        // filter_mode.value(FilterMode::Design, "Проектирование");
        // filter_mode.value(FilterMode::Operation, "Функционирование");

        // parameter_menu_item(ui, "", filter_mode, Label::new("Режим работы программы"));
    }

    pub fn show_filter_type(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_combo_box(
            ui,
            "filter_type",
            tr("label-filter-type"),
            &mut self.filter_type,
            [
                (FilterType::NonRecursive, tr("option-type-nonrecursive")),
                (FilterType::Recursive, tr("option-type-recursive")),
            ],
        );
    }

    pub fn show_filter_category(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_combo_box(
            ui,
            "filter_category",
            tr("label-filter-category"),
            &mut self.filter_category,
            [
                (FilterCategory::LowPass, tr("option-category-lowpass")),
                (FilterCategory::BandPass, tr("option-category-bandpass")),
                (FilterCategory::BandStop, tr("option-category-bandstop")),
                (FilterCategory::HighPass, tr("option-category-highpass")),
                (
                    FilterCategory::Differentiating,
                    tr("option-category-differentiating"),
                ),
            ],
        );
        // let mut filter_category =
        //     SelectButtonGroup::new("filter_category", &mut self.filter_category);

        // filter_category.value(FilterCategory::LowPass, "НЧФ");
        // filter_category.value(FilterCategory::BandPass, "ПФ");
        // filter_category.value(FilterCategory::BandStop, "РФ");
        // filter_category.value(FilterCategory::HighPass, "ВЧФ");
        // filter_category.value(FilterCategory::Differentiating, "ДФ");

        // parameter_menu_item(ui, "", filter_category, Label::new("Тип фильтра:\n• Низкочастотный фильтр (НЧФ)\n• Полосовой фильтр (ПФ)\n• Режекторный фильтр (РФ)\n• Высокочастотный фильтр (ВЧФ)\n• Дифференцирующий фильтр (ДФ)"));
    }

    pub fn show_sampling_rate(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_row!(
            ui,
            RichText::new("Δt").font(MATH_FONT.clone()),
            |ui: &mut egui::Ui| {
                ui.add(
                    DragValue::new(&mut self.sampling_rate)
                        .speed(0.01)
                        .suffix(tr("unit-seconds"))
                        .range(0.01..=100.0)
                        .max_decimals_opt(super::MAX_DECIMALS_OPT),
                )
                .on_hover_text(tr("dt-tooltip"));
            }
        );
        // parameter_menu_item_with_range(
        //     ui,
        //     RichText::new("Δt").font(MATH_FONT.clone()),
        //     DragValue::new(&mut self.sampling_rate)
        //         .suffix(" сек")
        //         .range(0.0..=100.0)
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT),
        //     Label::new("Шаг дескретизации по времени"),
        //     RichText::new("0.05≤"),
        //     RichText::new("≤0.25"),
        // );
    }
}

impl Default for CommonParameters {
    fn default() -> Self {
        Self {
            mode: FilterMode::default(),
            filter_type: FilterType::default(),
            filter_category: FilterCategory::default(),
            sampling_rate: 0.01,
        }
    }
}

impl CommonParameters {
    pub fn get_iw(&self) -> i32 {
        self.mode as i32
    }

    pub fn get_il(&self) -> i32 {
        self.filter_type as i32
    }

    pub fn get_in(&self) -> i32 {
        self.filter_category as i32
    }

    pub fn get_dt(&self) -> f64 {
        self.sampling_rate
    }
}
