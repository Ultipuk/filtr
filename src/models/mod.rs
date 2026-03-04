use std::sync::LazyLock;

use egui::{Align, FontFamily, FontId, TextFormat, Ui};
use serde::{Deserialize, Serialize};

// pub mod app_state;
pub mod common;
pub mod common_filter;
pub mod cutoff;
pub mod design;
pub mod operation;
// pub mod plot;
pub mod recursive;
// pub mod table;

pub static MATH_FONT_FAMILY: LazyLock<FontFamily> =
    LazyLock::new(|| FontFamily::Name("math".into()));
pub static MATH_FONT: LazyLock<FontId> =
    LazyLock::new(|| FontId::new(16.0, MATH_FONT_FAMILY.clone()));
pub static MATH_FONT_SMALL: LazyLock<FontId> =
    LazyLock::new(|| FontId::new(11.0, MATH_FONT_FAMILY.clone()));

pub const MAX_DECIMALS_OPT: Option<usize> = Some(12);

pub fn math_text_format(ui: &Ui) -> TextFormat {
    TextFormat {
        font_id: MATH_FONT.clone(),
        color: ui.style().visuals.text_color(),
        ..Default::default()
    }
}

pub fn math_text_format_sub(ui: &Ui) -> TextFormat {
    TextFormat {
        font_id: MATH_FONT_SMALL.clone(),
        valign: Align::BOTTOM,
        color: ui.style().visuals.text_color(),
        ..Default::default()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilterMode {
    /// Режим проектирования фильтра
    #[default]
    Design = 0,
    /// Режим функционирования фильтра
    Operation = 1,
}

// IL
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilterType {
    /// Нерекурсивный фильтр
    #[default]
    NonRecursive = 1,
    /// Рекурсивный фильтр
    Recursive = 2,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilterCategory {
    /// Низкочастотный фильтр, НЧФ (LPF)
    #[default]
    LowPass = 1,

    /// Полосовой фильтр, ПФ (BPF)
    BandPass = 2,

    /// Режекторный фильтр, РФ (BSF)
    BandStop = 3,

    /// Высокочастотный фильтр, ВЧФ (HPF)
    HighPass = 4,

    /// Дифференцирующий фильтр, ДФ (DF)
    Differentiating = 5,
}

/// Тип модельной задачи
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperationModeType {
    /// Спектральный анализ сигнала
    #[default]
    SpectralAnalysis = 1,

    /// Сглаживание сигнала
    Smoothing,

    /// Дифференцирование сигнала
    Differentiating,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum SignalType {
    /// Сигнал первого типа
    ///
    /// x(t) = x1(t)
    #[default]
    X1 = 1,

    /// Сигнал второго типа
    ///
    /// x(t) = x1(t) + x2(t)
    X1X2,

    /// Сигнал третьего типа
    ///
    /// x(t) = x1(t) + x3(t)
    X1X3,

    /// Сигнал четвертого типа
    ///
    /// x(t) = x2(t)
    X2,

    /// Сигнал пятого типа
    ///
    /// x(t) = x2(t) + x3(t)
    X2X3,

    /// Сигнал шестого типа
    ///
    /// x(t) = x3(t)
    X3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_conversion() {
        assert_eq!(FilterMode::Design as i32, 0);
        assert_eq!(FilterMode::Operation as i32, 1);
    }

    #[test]
    fn test_filter_type_conversion() {
        assert_eq!(FilterType::NonRecursive as i32, 1);
        assert_eq!(FilterType::Recursive as i32, 2);
    }

    #[test]
    fn test_filter_category_conversion() {
        assert_eq!(FilterCategory::LowPass as i32, 1);
        assert_eq!(FilterCategory::BandPass as i32, 2);
        assert_eq!(FilterCategory::BandStop as i32, 3);
        assert_eq!(FilterCategory::HighPass as i32, 4);
        assert_eq!(FilterCategory::Differentiating as i32, 5);
    }

    #[test]
    fn test_operation_mode_type_conversion() {
        assert_eq!(OperationModeType::SpectralAnalysis as i32, 1);
        assert_eq!(OperationModeType::Smoothing as i32, 2);
        assert_eq!(OperationModeType::Differentiating as i32, 3);
    }
}
