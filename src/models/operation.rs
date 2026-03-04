use crate::i18n::tr;
use crate::models::{
    FilterType, MATH_FONT, MATH_FONT_SMALL, math_text_format, math_text_format_sub,
};
use crate::parameter_section_row;
use crate::widgets::parameter_section_combo_box;
use eframe::epaint::text::LayoutJob;
use egui::{DragValue, RichText, Ui};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum Task {
    #[default]
    SpecralAnalysis = 1,
    Convolution = 2,
    Differentiation = 3,
}

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum Signal {
    #[default]
    X1 = 1,
    X1X2,
    X1X3,
    X2,
    X2X3,
    X3,
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X1 => write!(f, "x₁(t)"),
            Self::X1X2 => write!(f, "x₁(t)+x₂(t)"),
            Self::X1X3 => write!(f, "x₁(t)+x₃(t)"),
            Self::X2 => write!(f, "x₂(t)"),
            Self::X2X3 => write!(f, "x₂(t)+x₃(t)"),
            Self::X3 => write!(f, "x₃(t)"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OperationParameters {
    pub task: Task,
    pub main_interval_length: f64,
    pub impluse_response_length: i32,
    pub signal: Signal,
    pub effective_length: f64,
    pub phase_delay: f64,
    pub conv_diff_parameters: ConvDiffParameters,
}

impl Default for OperationParameters {
    fn default() -> Self {
        Self {
            main_interval_length: 0.0,
            impluse_response_length: 0,
            effective_length: 0.0,
            phase_delay: 0.0,
            task: Task::default(),
            signal: Signal::default(),
            conv_diff_parameters: ConvDiffParameters::default(),
        }
    }
}

impl OperationParameters {
    pub fn get_im(&self) -> Task {
        self.task
    }

    pub fn get_to(&self) -> f64 {
        self.main_interval_length
    }

    pub fn get_tp(&self) -> f64 {
        self.effective_length
    }

    pub fn get_tf(&self) -> f64 {
        self.phase_delay
    }

    pub fn get_l(&self) -> i32 {
        self.impluse_response_length
    }

    pub fn get_lf(&self, dt: f64) -> i32 {
        (self.get_tf() / dt + 1.1) as i32
    }

    pub fn get_ne(&self, dt: f64) -> i32 {
        ((self.get_to() / dt) + 1.1) as i32
    }

    pub fn get_iv(&self) -> i32 {
        self.signal as i32
    }

    pub fn get_nf(&self, dt: f64, filter_type: FilterType) -> i32 {
        match filter_type {
            FilterType::NonRecursive => 2 * self.get_l(),
            FilterType::Recursive => ((self.get_tp() / dt) + 1.1) as i32,
        }
    }

    pub fn get_nx(&self, dt: f64, filter_type: FilterType) -> i32 {
        match filter_type {
            FilterType::NonRecursive => {
                self.get_nf(dt, filter_type) + self.get_l() + self.get_ne(dt) + 1
            }
            FilterType::Recursive => {
                let lf = self.get_lf(dt);
                self.get_nf(dt, filter_type) + lf.abs() + self.get_ne(dt) + 1
            }
        }
    }
}

impl OperationParameters {
    pub fn show(&mut self, ui: &mut egui::Ui, filter_type: FilterType) {
        self.show_task(ui);
        self.show_main_interval_length(ui);

        match filter_type {
            FilterType::NonRecursive => {
                self.show_impluse_response_length(ui);
            }
            FilterType::Recursive => {
                self.show_effective_length(ui);
                self.show_phase_delay(ui);
            }
        }

        match self.task {
            Task::SpecralAnalysis => {
                self.show_signal(ui);
            }
            Task::Differentiation | Task::Convolution => {
                self.conv_diff_parameters.show(ui);
            }
        }
    }

    pub fn show_task(&mut self, ui: &mut egui::Ui) {
        let _ = parameter_section_combo_box(
            ui,
            "filter_operation",
            tr("label-task"),
            &mut self.task,
            [
                (Task::SpecralAnalysis, tr("option-task-spectral")),
                (Task::Differentiation, tr("option-task-diff")),
                (Task::Convolution, tr("option-task-smooth")),
            ],
        );

        // let mut task = SelectButtonGroup::new("task", &mut self.task);

        // task.value(Task::SpecralAnalysis, "Спектральный анализ");
        // task.value(Task::Differentiation, "Дифференцирование");
        // task.value(Task::Convolution, "Сглаживание");

        // parameter_menu_item(ui, "", task, Label::new("Выбор задачи"));
    }
    fn show_main_interval_length(&mut self, ui: &mut Ui) {
        let mut title = LayoutJob::default();

        title.append("T", 0.0, math_text_format(ui));
        title.append("0", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.main_interval_length)
                    .suffix(tr("unit-seconds"))
                    .range(0.0..=100.0)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("main-interval-tooltip"));
        });

        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.main_interval_length)
        //         .suffix(" сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Длительность основного интервала"),
        // );
    }
    fn show_impluse_response_length(&mut self, ui: &mut Ui) {
        let _ = parameter_section_row!(
            ui,
            RichText::new("L").font(MATH_FONT.clone()),
            |ui: &mut egui::Ui| {
                ui.add(DragValue::new(&mut self.impluse_response_length).range(0..=5000))
                    .on_hover_text(tr("impulse-len-tooltip"));
            }
        );
        // parameter_menu_item(
        //     ui,
        //     RichText::new("L").font(MATH_FONT.clone()),
        //     DragValue::new(&mut self.impluse_response_length),
        //     Label::new("Длина импульсной характеристики"),
        // );
    }
    fn show_effective_length(&mut self, ui: &mut Ui) {
        let mut title = LayoutJob::default();

        title.append("T", 0.0, math_text_format(ui));
        title.append("w", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.effective_length)
                    .suffix(tr("unit-seconds"))
                    .range(0.0..=100.0)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("effective-len-tooltip"));
        });

        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.effective_length)
        //         .suffix(" сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Эффективная длительность временной характеристики"),
        // );
    }

    fn show_phase_delay(&mut self, ui: &mut Ui) {
        let mut title = LayoutJob::default();

        title.append("τ ", 0.0, math_text_format(ui));
        title.append("Φ", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.phase_delay)
                    .suffix(tr("unit-seconds"))
                    .range(0.0..=100.0)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("phase-delay-tooltip"));
        });

        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.phase_delay)
        //         .suffix(" сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Фазовая задержка"),
        // );
    }
    fn show_signal(&mut self, ui: &mut Ui) {
        let _ = parameter_section_combo_box(
            ui,
            "filter_signal",
            tr("label-signal"),
            &mut self.signal,
            [
                (
                    Signal::X1,
                    RichText::new(Signal::X1.to_string()).font(MATH_FONT_SMALL.clone()),
                ),
                (
                    Signal::X1X2,
                    RichText::new(Signal::X1X2.to_string()).font(MATH_FONT_SMALL.clone()),
                ),
                (
                    Signal::X1X3,
                    RichText::new(Signal::X1X3.to_string()).font(MATH_FONT_SMALL.clone()),
                ),
                (
                    Signal::X2,
                    RichText::new(Signal::X2.to_string()).font(MATH_FONT_SMALL.clone()),
                ),
                (
                    Signal::X2X3,
                    RichText::new(Signal::X2X3.to_string()).font(MATH_FONT_SMALL.clone()),
                ),
                (
                    Signal::X3,
                    RichText::new(Signal::X3.to_string()).font(MATH_FONT_SMALL.clone()),
                ),
            ],
        );

        // let mut signal = SelectButtonGroup::new("signal", &mut self.signal);

        // signal.value(
        //     Signal::X1,
        //     RichText::new(Signal::X1.to_string()).font(MATH_FONT_SMALL.clone()),
        // );
        // signal.value(
        //     Signal::X1X2,
        //     RichText::new(Signal::X1X2.to_string()).font(MATH_FONT_SMALL.clone()),
        // );
        // signal.value(
        //     Signal::X1X3,
        //     RichText::new(Signal::X1X3.to_string()).font(MATH_FONT_SMALL.clone()),
        // );
        // signal.value(
        //     Signal::X2,
        //     RichText::new(Signal::X2.to_string()).font(MATH_FONT_SMALL.clone()),
        // );
        // signal.value(
        //     Signal::X2X3,
        //     RichText::new(Signal::X2X3.to_string()).font(MATH_FONT_SMALL.clone()),
        // );
        // signal.value(
        //     Signal::X3,
        //     RichText::new(Signal::X3.to_string()).font(MATH_FONT_SMALL.clone()),
        // );

        // parameter_menu_item(ui, "", signal, Label::new("Выбор сигнала"));
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConvDiffParameters {
    noise_strength: f64,
    lower_freq: f64,
    upper_freq: f64,
    transition_width: f64,
}

impl Default for ConvDiffParameters {
    fn default() -> Self {
        Self {
            noise_strength: 0.0,
            lower_freq: 0.0,
            upper_freq: 0.0,
            transition_width: 0.0,
        }
    }
}

impl ConvDiffParameters {
    pub fn get_px(&self) -> f64 {
        self.noise_strength
    }

    pub fn get_o1(&self) -> f64 {
        self.lower_freq
    }

    pub fn get_o2(&self) -> f64 {
        self.upper_freq
    }

    pub fn get_db(&self) -> f64 {
        self.transition_width
    }
}

impl ConvDiffParameters {
    pub fn show_noise_strength(&mut self, ui: &mut egui::Ui) {
        let mut title = LayoutJob::default();

        title.append("δX", 0.0, math_text_format(ui));
        title.append("m", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.noise_strength)
                    .suffix(tr("unit-seconds"))
                    .range(0.0..=100.0)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("noise-level-tooltip"));
        });

        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.noise_strength)
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Уровень помехи"),
        // );
    }

    pub fn show_lower_freq(&mut self, ui: &mut egui::Ui) {
        let mut title = LayoutJob::default();

        title.append("ω", 0.0, math_text_format(ui));
        title.append("δX", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.lower_freq)
                    .suffix(tr("unit-rad-sec"))
                    .range(0.0..=100.0)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("noise-low-tooltip"));
        });

        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.lower_freq)
        //         .suffix(" рад/сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Нижняя частота эффективной полосы частотного спектра помехи"),
        // );
    }

    pub fn show_upper_freq(&mut self, ui: &mut egui::Ui) {
        let mut title = LayoutJob::default();

        title.append("Ω", 0.0, math_text_format(ui));
        title.append("δX", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.upper_freq)
                    .suffix(tr("unit-rad-sec"))
                    .range(0.0..=100.0)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("noise-high-tooltip"));
        });

        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.upper_freq)
        //         .suffix(" рад/сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Верхняя частота эффективной полосы частотного спектра помехи"),
        // );
    }

    pub fn show_transition_width(&mut self, ui: &mut egui::Ui) {
        let mut title = LayoutJob::default();

        title.append("Δω", 0.0, math_text_format(ui));
        title.append("ΠδX", 0.0, math_text_format_sub(ui));

        let _ = parameter_section_row!(ui, title, |ui: &mut egui::Ui| {
            ui.add(
                DragValue::new(&mut self.transition_width)
                    .suffix(tr("unit-rad-sec"))
                    .range(0.0..=100.0)
                    .max_decimals_opt(super::MAX_DECIMALS_OPT),
            )
            .on_hover_text(tr("noise-transition-tooltip"));
        });

        // parameter_menu_item(
        //     ui,
        //     title,
        //     DragValue::new(&mut self.transition_width)
        //         .suffix(" рад/сек")
        //         .max_decimals_opt(super::MAX_DECIMALS_OPT)
        //         .range(0.0..=100.0),
        //     Label::new("Ширина переходной полосы частотного спектра помехи"),
        // );
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.show_noise_strength(ui);
        self.show_lower_freq(ui);
        self.show_upper_freq(ui);
        self.show_transition_width(ui);
    }
}
